use crate::SharedData;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use log::info;
use openssl::ssl::{SslAcceptor, SslMethod, SslFiletype};
use std::sync::{Arc, Mutex};

pub mod handlers {
    use crate::SharedData;
    use actix_web::{web, Responder};
    use std::sync::Mutex;

    pub async fn index(data: web::Data<Mutex<SharedData>>) -> impl Responder {
        let mut shared_data = data.lock().unwrap();
        shared_data.counter += 1;
        format!("Request number: {}", &shared_data.counter)
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

        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file("key.pem", SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file("cert.pen").unwrap();

        HttpServer::new(move || {
            App::new()
                .app_data(data.clone())
                .route("/", web::get().to(handlers::index))
        })
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
    }
}
