pub mod graphql;
pub mod page;

use hyper::{Body, Method};
use routerify::Router;
use std::io;

use self::graphql::{graphiql, graphql};

pub fn handle_routes() -> Router<Body, io::Error> {
    Router::builder()
        .add("/graphql", vec![Method::GET, Method::POST], graphql)
        .get("/graphiql", graphiql)
        .build()
        .unwrap()
}
