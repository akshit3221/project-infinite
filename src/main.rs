<<<<<<< HEAD
mod database;
mod upload;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use database::{MongoDB, User};
use std::env;
use chrono::Utc;
use actix_files::{NamedFile, Files};
use upload::handle_upload;

struct AppState {
    db: MongoDB,
=======


mod models;
mod routes;
mod db;
mod api;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tokio::sync::Mutex;
use env_logger::Env;
use actix_files::Files;
use actix_files as fs;

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
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .route("/fetch_data", web::get().to(api::fetch_data))
            .route("/login_url", web::get().to(api::login_url))
            .route("/greet", web::get().to(api::greet))
       
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
    .map_err(|e| {
        println!("Server error: {}", e);
        e
    })
>>>>>>> 6dafdcb6176bb56653a7134dd076f753f48f9e68
}

async fn get_user_data(db: &MongoDB, telegram_id: i64) -> Result<Option<User>, mongodb::error::Error> {
    db.find_user_by_telegram_id(telegram_id).await
}


#[get("/login")]
async fn login_page() -> impl Responder {
    // Load the HTML file
    let path: std::path::PathBuf = "./static/index.html".parse().unwrap();
    NamedFile::open(path)
}

#[get("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.clear();
    HttpResponse::Found().append_header(("Location", "/login")).finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db = MongoDB::new().await.expect("Failed to initialize MongoDB");
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .app_data(web::Data::new(token.clone()))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                actix_web::cookie::Key::generate(),
            ))
            
            .service(login_page)
            .service(logout)
            .service(Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/upload").route(web::get().to(handle_upload)))
            .route("/upload", web::post().to(handle_upload))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

