use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use mongodb::{bson::doc, Client, Database};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    first_name: String,
    last_name: Option<String>,
    telegram_id: i64,
    telegram_username: Option<String>,
    profile_picture: Option<String>,
    id: Option<i64>,
}

struct AppState {
    db: Database,
}

async fn get_user_data(db: &Database, telegram_id: i64) -> Result<User, mongodb::error::Error> {
    let users = db.collection::<User>("users");
    let filter = doc! { "telegram_id": telegram_id };
    let user = users.find_one(filter, None).await?;
    Ok(user.expect("User not found"))
}

#[get("/user")]
async fn user_page(session: Session, data: web::Data<AppState>) -> impl Responder {
    if let Some(logged_in) = session.get::<bool>("logged-in").unwrap() {
        if logged_in {
            if let Some(telegram_id) = session.get::<i64>("telegram_id").unwrap() {
                match get_user_data(&data.db, telegram_id).await {
                    Ok(user_data) => {
                        let mut html = if let Some(last_name) = &user_data.last_name {
                            format!("<h1>Hello, {} {}!</h1>", user_data.first_name, last_name)
                        } else {
                            format!("<h1>Hello, {}!</h1>", user_data.first_name)
                        };

                        if let Some(profile_picture) = &user_data.profile_picture {
                            html += &format!(
                                r#"<a href="{}" target="_blank"><img class="profile-picture" src="{}?v={}"></a>"#,
                                profile_picture,
                                profile_picture,
                                chrono::Utc::now().timestamp()
                            );
                        }

                        html += &format!(
                            "<h2 class='user-data'>First Name: {}</h2>",
                            user_data.first_name
                        );

                        if let Some(last_name) = &user_data.last_name {
                            html += &format!(
                                "<h2 class='user-data'>Last Name: {}</h2>",
                                last_name
                            );
                        }

                        if let Some(username) = &user_data.telegram_username {
                            html += &format!(
                                r#"<h2 class='user-data'>Username: <a href="https://t.me/{}" target="_blank">@{}</a></h2>"#,
                                username,
                                username
                            );
                        }

                        html += &format!(
                            "<h2 class='user-data'>Telegram ID: {}</h2>",
                            user_data.telegram_id
                        );

                        if let Some(user_id) = &user_data.id {
                            html += &format!(
                                "<h2 class='user-data'>User ID: {}</h2>",
                                user_id
                            );
                        }

                        html += r#"<a href="/logout"><h2 class='logout'>Logout</h2></a>"#;

                        return HttpResponse::Ok().content_type("text/html").body(format!(
                            r#"
                            <!DOCTYPE html>
                            <html lang="en-US">
                            <head>
                                <title>Logged In User</title>
                                <meta charset="UTF-8">
                                <meta name="viewport" content="width=device-width, initial-scale=1">
                                <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Nanum+Gothic">
                                <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/animate.css/4.1.1/animate.min.css">
                                <link rel="stylesheet" href="assets/style.css">
                            </head>
                            <body>
                                <div class="middle-center">
                                    {}
                                </div>
                            </body>
                            </html>
                            "#,
                            html
                        ));
                    }
                    Err(_) => {
                        return HttpResponse::InternalServerError()
                            .body("Failed to retrieve user data");
                    }
                }
            }
        }
    }

    HttpResponse::Found().header("Location", "/login").finish()
}

#[get("/login")]
async fn login_page() -> impl Responder {
    // Load the HTML file
    let path: std::path::PathBuf = "./static/login.html".parse().unwrap();
    actix_files::NamedFile::open(path)
}

#[get("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.clear();
    HttpResponse::Found().header("Location", "/login").finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&database_url).await.unwrap();
    let db = client.database("test");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                actix_web::cookie::Key::generate(),
            ))
            .service(user_page)
            .service(login_page)
            .service(logout)
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
