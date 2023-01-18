use crate::{controllers, models::user::AccessLevel, utils::middleware};
use hyper::{server::conn::AddrIncoming, Body, Server};
use routerify::{Middleware, Router, RouterService};
use sqlx::{postgres::PgPoolOptions, Row};
use tracing::Level;
use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
    sync::Arc, env,
};
use tracing_subscriber::FmtSubscriber;

pub type ServerStart = Server<AddrIncoming, RouterService<Body, io::Error>>;

pub async fn up(conf: super::config::Config) -> (ServerStart, SocketAddr) {
    let subscriber = FmtSubscriber::builder().with_max_level(get_trace_level()).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(conf.db.max_connections)
        .acquire_timeout(std::time::Duration::from_secs(conf.db.connect_timeout))
        .connect(&conf.db.url)
        .await
        .unwrap();

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // Check if this is the first run by checking if the admin user exists
    let admin_exists = sqlx::query(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM users
            WHERE username = 'admin'
        )
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap()
    .get::<bool, _>(0);

    // If the admin user doesn't exist, create it

    if !admin_exists {
        let admin_email = "admin@localhost".to_string();
        let admin_access_level = AccessLevel::Admin;

        crate::database::user::create_user(
            admin_email,
            conf.default_admin_username.clone(),
            conf.default_admin_password.clone(),
            admin_access_level,
            &pool,
        )
        .await
        .unwrap();
    }

    let schema = Arc::new(crate::controllers::graphql::make_schema());

    let router: Router<Body, io::Error> = Router::builder()
        .data(schema)
        .data(pool)
        .data(conf.clone())
        .middleware(Middleware::pre(middleware::logger))
        .middleware(Middleware::post(middleware::setup_headers))
        .middleware(Middleware::pre(middleware::auth))
        .scope("/", controllers::handle_routes())
        .err_handler(middleware::handle_error)
        .build()
        .unwrap();

    let service = RouterService::new(router).unwrap();
    let ip = IpAddr::V4(Ipv4Addr::from_str(&conf.ip.to_string()).unwrap());
    let addr = SocketAddr::new(ip, conf.port);
    let server = Server::bind(&addr).serve(service);

    (server, addr)
}

fn get_trace_level() -> Level {
    // Get the value of the RUST_LOG environment variable
    let level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    // Convert the string to lowercase and then match it
    match level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    }
}