use actix_session::{Session};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;

async fn logout(session: Session) -> impl Responder {
    // Clear the session
    session.clear();
    // Redirect to the login page
    HttpResponse::Found().header("Location", "/login").finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_session::SessionMiddleware::new(
                actix_session::storage::CookieSessionStore::default(),
                actix_web::cookie::Key::generate(),
            ))
            .service(web::resource("/logout").route(web::get().to(logout)))
            .service(web::resource("/login").route(web::get().to(login_page)))
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn login_page() -> impl Responder {
    // Load the HTML file
    let path: std::path::PathBuf = "./static/index.html".parse().unwrap();
    actix_files::NamedFile::open(path)
}
