use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use mongodb::{Client, options::ClientOptions, Database, Collection, bson::{doc, oid::ObjectId}};
use std::env;
use dotenv::dotenv;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    name: String,
    email_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateUserResponse {
    id: ObjectId,
    name: String,
    email_id: String,
}

async fn get_database() -> Database {
    dotenv().ok();
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client_options = ClientOptions::parse(&mongodb_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client.database("test_db")
}

#[actix_web::post("/users")]
async fn create_user(
    user_data: web::Json<User>,
    db: web::Data<Mutex<Database>>,
) -> impl Responder {
    let collection: Collection<User> = db.lock().await.collection("users");
    let user = user_data.into_inner();
    let insert_result = collection.insert_one(user.clone(), None).await.unwrap();
    let new_id = insert_result.inserted_id.as_object_id().unwrap();

    HttpResponse::Created().json(CreateUserResponse {
        id: new_id,
        name: user.name,
        email_id: user.email_id,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = get_database().await;
    let db = web::Data::new(Mutex::new(db));

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
