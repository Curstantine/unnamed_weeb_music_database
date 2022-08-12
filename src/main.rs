#![allow(clippy::manual_map)]
mod constants;
mod controllers;
mod database;
mod models;
mod utils;

use tracing::info;
use utils::{config, startup};

sea_query::sea_query_driver_postgres!();

#[tokio::main]
async fn main() {
    let conf = config::get_config();
    let (server, addr) = startup::up(conf).await;

    info!("App is running on: {}", addr);

    if let Err(err) = server.await {
        eprintln!("Server error: {err}");
    }
}
