use crate::SharedData;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use log::info;
use std::sync::{Arc, Mutex};

pub mod handlers {
    use crate::SharedData;
    use actix_web::{web, Responder};
    use std::sync::Mutex;

    pub async fn index(data: web::Data<Mutex<SharedData>>) -> impl Responder {
        let mut counter = data.lock().unwrap().counter;
        counter += 1;
        format!("Request number: {}", &counter)
    }
}

#[derive(Debug)]
pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start(self, shared_data: Arc<Mutex<SharedData>>) -> std::io::Result<()> {
        info!("Starting server...");

        let data = Data::from(shared_data);

        HttpServer::new(move || {
            App::new()
                .app_data(data.clone())
                .route("/", web::get().to(handlers::index))
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }
}
