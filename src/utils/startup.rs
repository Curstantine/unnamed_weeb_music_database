use crate::{controllers, utils::middleware};
use hyper::{server::conn::AddrIncoming, Body, Server};
use routerify::{Middleware, Router, RouterService};
use sqlx::postgres::PgPoolOptions;
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
        .connect_timeout(conf.db.connect_timeout)
        .connect(&conf.db.url)
        .await
        .unwrap();

    //let ctx = crate::utils::context::Context::new(pool);

    let schema = Arc::new(crate::controllers::graphql::make_schema());

    let router: Router<Body, io::Error> = Router::builder()
        .data(schema)
        .data(pool)
        .middleware(Middleware::pre(middleware::logger))
        .middleware(Middleware::post(middleware::setup_headers))
        .scope("/", controllers::handle_routes())
        .err_handler(middleware::handle_error)
        .build()
        .unwrap();

    let service = RouterService::new(router).unwrap();
    let addr = SocketAddr::new(conf.ip, conf.port);
    let server = Server::bind(&addr).serve(service);

    (server, addr)
}
