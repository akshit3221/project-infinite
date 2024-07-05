use actix_web::{web, HttpResponse, Error, Responder};
use reqwest;
use serde_json::Value;
use std::env;
use dotenv::dotenv;
use base64::encode;
use qrcode::{QrCode, render::svg};

pub async fn fetch_data() -> Result<HttpResponse, Error> {
    dotenv().ok();
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
    let encoded_token = encode(token);
    let url = format!("https://api.telegram.org/bot{}/getMe", encoded_token);

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

pub async fn greet() -> impl Responder {
    "Hello, world!"
}

pub async fn login_url() -> impl Responder {
    dotenv().ok();
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
    let encoded_token = encode(token);
    let login_url = format!(
        "https://telegram.me/{}?start=auth&token={}",
        "infiniteproject_bot",
        encoded_token
    );
    println!("Login URL: {}", login_url);

    // Generate the QR code
    let code = QrCode::new(login_url.clone()).expect("Failed to generate QR code");
    let image = code.render::<svg::Color>().build();
    
    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(image)
}
