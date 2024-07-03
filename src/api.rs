use actix_web::{web, App, HttpServer, Responder, HttpResponse, Error};
use reqwest;
use serde_json::Value;
use std::env;
use dotenv::dotenv;
use base64::encode;
use qrcode::{QrCode, render::svg};
use actix_files as fs;
async fn fetch_data() -> Result<HttpResponse, Error> {
    dotenv().ok();
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
    let encoded_token = encode(token);
    let url = format!("tg://login?token={}", encoded_token);

    let res = reqwest::get(&url).await;

    match res {
        Ok(response) => {
            match response.json::<Value>().await {
                Ok(data) => Ok(HttpResponse::Ok().json(data)),
                Err(_) => Ok(HttpResponse::InternalServerError().body("Error parsing JSON")),
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().body("Error fetching data")),
    }
}

async fn greet() -> impl Responder {
    "Hello, world!"
}

async fn login_url() -> impl Responder {
    dotenv().ok();
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
    let encoded_token = encode(token);
    let login_url = format!(
        "https://telegram.me/{}?start=auth&token={}",
        "infiniteproject_bot",
        encoded_token
    );
    println!("Login URL: {}", login_url);

    //  for QR code
    let code = QrCode::new(login_url.clone()).expect("Failed to generate QR code");
    let image = code.render::<svg::Color>().build();
    
    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(image)
}
