use actix_web::{App, HttpServer};
use serde::{Deserialize, Serialize};

mod infinia;
use infinia::services;

#[derive(Serialize, Deserialize, Clone)]
struct TodolistEntry {
    id: i32,
    date: i64,
    title: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(services::config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
