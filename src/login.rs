use actix_web::{web, HttpResponse, Result};
use actix_session::Session;
use serde::Deserialize;
use reqwest::Client;
use crate::AppState;

#[derive(Deserialize)]
pub struct TelegramLogin {
    pub id: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: String,
    pub photo_url: Option<String>,
    pub auth_date: String,
    pub hash: String,
}


