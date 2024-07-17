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

