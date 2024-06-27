// db.rs

use mongodb::{Client, options::ClientOptions, Database};
use std::env;

pub async fn get_database() -> Database {
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    println!("Connecting to MongoDB...");
    let client_options = ClientOptions::parse(&mongodb_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("test_db");
    println!("Connected to MongoDB database 'test_db'");
    db
}
