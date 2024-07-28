use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, Result, get, post};
use std::path::PathBuf;

mod upload;

#[get("/login")]
async fn login_page() -> Result<HttpResponse> {
    let path: PathBuf = "./static/login.html".parse().unwrap();
    let content = std::fs::read_to_string(&path)?;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(content))
}



#[get("/upload")]
async fn upload_page(req: HttpRequest) -> Result<HttpResponse> {
    let path: PathBuf = "./frontend/upload.html".parse().unwrap();
    match NamedFile::open(path) {
        Ok(file) => Ok(file.into_response(&req)),
        Err(e) => {
            eprintln!("Error opening upload page: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[post("/upload")]
async fn handle_upload(req: HttpRequest) -> Result<HttpResponse> {
   
    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(login_page)
            
            .service(upload_page)
            .service(handle_upload)
            .service(web::resource("/upload").route(web::post().to(upload::handle_upload)))
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .service(actix_files::Files::new("/frontend", "./frontend").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
