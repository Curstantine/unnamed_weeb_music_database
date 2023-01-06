use crate::{controllers, models::user::AccessLevel, utils::middleware};
use hyper::{server::conn::AddrIncoming, Body, Server};
use routerify::{Middleware, Router, RouterService};
use sqlx::{postgres::PgPoolOptions, Row};
use std::{io, net::SocketAddr, sync::Arc};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub type ServerStart = Server<AddrIncoming, RouterService<Body, io::Error>>;

pub async fn up(conf: super::config::Config) -> (ServerStart, SocketAddr) {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(conf.db.max_connections)
        .acquire_timeout(conf.db.connect_timeout)
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
        let admin_username = "admin".to_string();
        let admin_password = "admin".to_string();
        let admin_email = "admin@localhost".to_string();
        let admin_access_level = AccessLevel::Admin;

        crate::database::user::create_user(
            admin_email,
            admin_username,
            admin_password,
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
    let addr = SocketAddr::new(conf.ip, conf.port);
    let server = Server::bind(&addr).serve(service);

    (server, addr)
}
