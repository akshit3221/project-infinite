use actix_files::NamedFile;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use std::env;
use std::path::PathBuf;

struct AppState {
    bot_username: String,
}

// Handler to serve the login page
async fn login_page(data: web::Data<AppState>) -> Result<HttpResponse> {
    // Load the HTML file
    let path: PathBuf = "./static/login.html".parse().unwrap();
    let bot_username = &data.bot_username;
    let content = std::fs::read_to_string(&path)?;
    let content = content.replace("<?= BOT_USERNAME ?>", bot_username);

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(content))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let bot_username = env::var("BOT_USERNAME").expect("BOT_USERNAME must be set");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                bot_username: bot_username.clone(),
            }))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                actix_web::cookie::Key::generate(),
            ))
            .service(web::resource("/login").route(web::get().to(login_page)))
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
