mod application;
mod server;

use crate::server::Server;
use log::info;
use std::{
    env,
    sync::{Arc, Mutex},
};
use tokio::task;

use application::Kitten;

#[derive(Debug)]
pub struct SharedData {
    counter: usize,
}

#[actix_web::main]
async fn main() {
    //set info var for logging
    env::set_var("RUST_LOG", "test_server=info");
    pretty_env_logger::init();

    info!("Starting kitten");

    let shared_data = Arc::new(Mutex::new(SharedData { counter: 0 }));

    let server = Server::new();

    let local = task::LocalSet::new();
    let server_data = shared_data.clone();
    let ui_data = shared_data.clone();

    local.spawn_local(async move { server.start(server_data).await });
    local.spawn_local(async move { start_ui(ui_data).await });
    local.await;
}

async fn start_ui(shared_data: Arc<Mutex<SharedData>>) {
    info!("Starting User Interface..");

    let native_options = eframe::NativeOptions::default();

    let eframe_data = shared_data;
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(Kitten::new(cc, eframe_data))),
    );
}
