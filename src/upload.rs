use actix_web::{web, HttpResponse, Error};
use reqwest::Client;
use futures_util::StreamExt;
use std::fs::File;
use std::io::Write;

const TELEGRAM_TOKEN: &str = "<?= BOT_TOKEN ?>";
const CHAT_ID: &str = "5079459675";

pub async fn upload_chunk(client: &Client, token: &str, chat_id: &str, file_chunk: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.telegram.org/bot{}/sendDocument", token);
    let form = reqwest::multipart::Form::new()
        .text("chat_id", chat_id.to_string())
        .part("document", reqwest::multipart::Part::bytes(file_chunk).file_name("chunk.txt"));

    let response = client.post(&url).multipart(form).send().await?;
    
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Failed to upload chunk: {:?}", response.text().await?).into())
    }
}

pub async fn handle_upload(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let client = Client::new();
    let mut file = File::create("uploaded_file.txt").expect("Failed to create file");

    while let Some(chunk) = payload.next().await {
        match chunk {
            Ok(data) => {
                if let Err(e) = file.write_all(&data) {
                    eprintln!("Failed to write chunk to file: {}", e);
                    return Ok(HttpResponse::InternalServerError().finish());
                }

                // Upload the chunk to Telegram
                if let Err(e) = upload_chunk(&client, TELEGRAM_TOKEN, CHAT_ID, data.to_vec()).await {
                    eprintln!("Error uploading chunk to Telegram: {}", e);
                    return Ok(HttpResponse::InternalServerError().finish());
                }
            }
            Err(e) => {
                eprintln!("Failed to read chunk: {}", e);
                return Ok(HttpResponse::InternalServerError().finish());
            }
        }
    }

    Ok(HttpResponse::Ok().finish())
}
