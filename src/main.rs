use std::env;

use actix_web::{web, App, HttpServer};
use log::info;
use tokio::{task, time};

use crate::server::Server;

mod server;

#[actix_web::main]
async fn main() {
    //set info var for logging
    env::set_var("RUST_LOG", "test_server=info");
    pretty_env_logger::init();

    info!("Starting kitten");

    let server = Server::new();

    let local = task::LocalSet::new();

    local.spawn_local(async move { server.start().await });
    local.spawn_local(async move { startUI().await });
    local.await;
}

async fn startUI() {
    info!("Starting User Interface..");
}
