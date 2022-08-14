use actix_web::{web, App, HttpServer};

pub struct Server {}

pub mod handlers {
    use actix_web::web;

    pub async fn index(data: web::Data<String>) -> String {
        format!("Hello") // <- response with count
    }
}

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start(self) -> std::io::Result<()> {
        println!("Starting server...");

        HttpServer::new(move || App::new().route("/", web::get().to(handlers::index)))
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
    }
}
