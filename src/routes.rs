

use actix_web::{web, Responder, HttpResponse};
use tokio::sync::Mutex;
use mongodb::{Database, Collection};
use mongodb::bson::oid::ObjectId;

use crate::models::{User, CreateUserResponse};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(index))
            .service(
                web::resource("/users")
                    .route(web::post().to(create_user))
            ),
    );
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Server is running fine!")
}

async fn create_user(
    user_data: web::Json<User>,
    db: web::Data<Mutex<Database>>,
) -> impl Responder {
    println!("Received request to create user {:?}", user_data);

    let collection: Collection<User> = db.lock().await.collection("users");
    let user = user_data.into_inner();

    println!("Inserting user into database: {:?}", user);
    let insert_result = collection.insert_one(user.clone(), None).await.unwrap();
    let new_id = insert_result.inserted_id.as_object_id().unwrap();

    println!("User inserted successfully with ID: {:?}", new_id);

    HttpResponse::Created().json(CreateUserResponse {
        id: new_id,
        name: user.name,
        email_id: user.email_id,
    })
}

