use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{bson::doc, Client, Database};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;

#[derive(Debug, Deserialize)]
struct AuthData {
    id: i64,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
    photo_url: Option<String>,
    auth_date: i64,
    hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    first_name: String,
    last_name: Option<String>,
    telegram_id: i64,
    telegram_username: Option<String>,
    profile_picture: Option<String>,
    auth_date: i64,
}

async fn check_telegram_authorization(auth_data: &AuthData, bot_token: &str) -> Result<(), String> {
    let mut data_check_arr: Vec<String> = vec![
        format!("auth_date={}", auth_data.auth_date),
        format!("first_name={}", auth_data.first_name),
        format!("id={}", auth_data.id),
    ];

    if let Some(last_name) = &auth_data.last_name {
        data_check_arr.push(format!("last_name={}", last_name));
    }
    if let Some(username) = &auth_data.username {
        data_check_arr.push(format!("username={}", username));
    }
    if let Some(photo_url) = &auth_data.photo_url {
        data_check_arr.push(format!("photo_url={}", photo_url));
    }

    data_check_arr.sort();
    let data_check_string = data_check_arr.join("\n");
    let secret_key = Sha256::digest(bot_token.as_bytes());

    let mut hmac = hmac_sha256::HMAC::new(secret_key.as_ref());
    hmac.update(data_check_string.as_bytes());

    let result = hmac.finalize();
    let hash = base64::encode(result.into_bytes());

    if hash != auth_data.hash {
        return Err("Data is NOT from Telegram".into());
    }
    if (chrono::Utc::now().timestamp() - auth_data.auth_date) > 86400 {
        return Err("Data is outdated".into());
    }
    Ok(())
}

async fn user_authentication(db: &Database, auth_data: AuthData) -> Result<(), String> {
    let users = db.collection::<User>("users");

    let filter = doc! { "telegram_id": auth_data.id };
    let user = users.find_one(filter.clone(), None).await.unwrap();

    let new_user = User {
        first_name: auth_data.first_name,
        last_name: auth_data.last_name,
        telegram_id: auth_data.id,
        telegram_username: auth_data.username,
        profile_picture: auth_data.photo_url,
        auth_date: auth_data.auth_date,
    };

    if let Some(_) = user {
        users
            .update_one(filter, doc! { "$set": &new_user }, None)
            .await
            .unwrap();
    } else {
        users.insert_one(new_user, None).await.unwrap();
    }

    Ok(())
}

#[get("/login")]
async fn login(
    session: Session,
    query: web::Query<AuthData>,
    data: web::Data<AppState>,
) -> impl Responder {
    let auth_data = query.into_inner();
    match check_telegram_authorization(&auth_data, &data.bot_token).await {
        Ok(_) => match user_authentication(&data.db, auth_data).await {
            Ok(_) => {
                session.insert("logged-in", true).unwrap();
                session.insert("telegram_id", auth_data.id).unwrap();
                HttpResponse::Found().header("location", "/user").finish()
            }
            Err(err) => HttpResponse::InternalServerError().body(err),
        },
        Err(err) => HttpResponse::Unauthorized().body(err),
    }
}

#[get("/user")]
async fn user(session: Session) -> impl Responder {
    if let Some(logged_in) = session.get::<bool>("logged-in").unwrap() {
        if logged_in {
            return HttpResponse::Ok().body("Welcome, user!");
        }
    }
    HttpResponse::Unauthorized().body("Please log in first.")
}

struct AppState {
    db: Database,
    bot_token: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN must be set");
    let client = Client::with_uri_str(&database_url).await.unwrap();
    let db = client.database("test");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: db.clone(),
                bot_token: bot_token.clone(),
            }))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                actix_web::cookie::Key::generate(),
            ))
            .service(login)
            .service(user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
