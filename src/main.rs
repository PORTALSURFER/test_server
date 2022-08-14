use crate::server::Server;

mod server;

#[tokio::main]
async fn main() {
    let server = Server::new();

    server.start().await.expect("Failed to start server.");

    somethingelse().await;

    println!("Hello, world!");
}

async fn somethingelse() {
    println!("YO")
}
k