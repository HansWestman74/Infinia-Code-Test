use actix_web::{App, HttpServer};
use serde::{Deserialize, Serialize};

mod infinia;
use infinia::services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(services::config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
