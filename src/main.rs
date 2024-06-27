

mod models;
mod routes;
mod db;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tokio::sync::Mutex;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();  // Load environment variables from .env file
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let db = db::get_database().await;
    let db = web::Data::new(Mutex::new(db));

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .configure(routes::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
    .map_err(|e| {
        println!("Server error: {}", e);
        e
    })
}
