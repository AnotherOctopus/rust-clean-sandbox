use actix_web::{get, web, App, HttpServer, Responder};
mod controllers;
mod entities;
mod external;
mod usecases;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(external::greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
