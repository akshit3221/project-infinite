use actix_web::{web, HttpResponse, Result};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use base64;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct GetFileResponse {
    ok: bool,
    result: Option<FileResult>,
}

#[derive(Deserialize)]
struct FileResult {
    file_id: String,
    file_unique_id: String,
    file_size: Option<u64>,
    file_path: Option<String>,
}

async fn get_file_id(token: &str, file_identifier: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://api.telegram.org/bot{}/getFile", token);

    let response = client.post(&url)
        .form(&[("file_id", file_identifier)])
        .send()
        .await?
        .json::<GetFileResponse>()
        .await?;

    if let Some(file) = response.result {
        Ok(file.file_id)
    } else {
        Err("Failed to get file ID".into())
    }
}

async fn upload_file_part(token: &str, file_id: i64, part_index: i32, part: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://api.telegram.org/bot{}/upload.saveFilePart", token);

    let response = client.post(&url)
        .form(&[
            ("file_id", file_id.to_string()),
            ("file_part", part_index.to_string()),
            ("bytes", base64::encode(&part)),
        ])
        .send()
        .await?
        .json::<GetFileResponse>()
        .await?;

    if response.ok {
        Ok(())
    } else {
        Err("Failed to upload file part".into())
    }
}

async fn send_media(token: &str, file_id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://api.telegram.org/bot{}/messages.sendMedia", token);

    let media = format!("attach://file_{}", file_id);
    let response = client.post(&url)
        .form(&[
            ("media", media.to_string()),
            ("message", "Here is the large file".to_string()),
        ])
        .send()
        .await?
        .json::<GetFileResponse>()
        .await?;

    if response.ok {
        Ok(())
    } else {
        Err("Failed to send media".into())
    }
}

async fn upload_large_file(token: &str, file_identifier: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_id_str = get_file_id(token, file_identifier).await?;
    let file_id: i64 = file_id_str.parse()?;
    let mut file = File::open(file_path).await?;
    let file_size = file.metadata().await?.len();
    let part_size: u64 = 512 * 1024; // 512KB
    let total_parts = (file_size + part_size - 1) / part_size;
    let mut part_index: i32 = 0;

    while part_index < total_parts as i32 {
        let mut buffer = vec![0; part_size as usize];
        file.seek(tokio::io::SeekFrom::Start(part_index as u64 * part_size)).await?;
        let n = file.read(&mut buffer).await?;
        buffer.truncate(n);

        upload_file_part(token, file_id, part_index, buffer).await?;
        part_index += 1;
    }

    send_media(token, file_id).await?;

    Ok(())
}

pub async fn handle_upload(
    token: web::Data<String>,
    mut payload: web::Payload
) -> Result<HttpResponse, actix_web::Error> {
    let file_identifier = "your_file_identifier"; // Update this with the actual file identifier
    let file_path = "frontend/upload.html"; // Update this path as necessary

    upload_large_file(&token, file_identifier, file_path).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Upload error: {:?}", e))
    })?;

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}
