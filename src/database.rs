use mongodb::{options::ClientOptions, Client, Database, error::Error};
use serde::{Deserialize, Serialize};
use std::env;
use bson::doc;  // Import doc macro

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub first_name: String,
    pub last_name: Option<String>,
    pub telegram_id: i64,
    pub telegram_username: Option<String>,
    pub profile_picture: Option<String>,
    pub id: Option<i64>,
}

#[derive(Clone)]
pub struct MongoDB {
    pub db: Database,
}

impl MongoDB {
    pub async fn new() -> Result<Self, Error> {
        dotenv::dotenv().ok();
        let database_url = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
        let client_options = ClientOptions::parse(&database_url).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("telegram_login");
        Ok(MongoDB { db })
    }

    pub async fn insert_user(&self, user: &User) -> Result<(), Error> {
        let collection = self.db.collection::<User>("users");
        collection.insert_one(user, None).await?;
        Ok(())
    }

    pub async fn find_user_by_telegram_id(&self, telegram_id: i64) -> Result<Option<User>, Error> {
        let collection = self.db.collection::<User>("users");
        let filter = doc! { "telegram_id": telegram_id };
        let user = collection.find_one(filter, None).await?;
        Ok(user)
    }

    pub async fn update_user(&self, telegram_id: i64, update_doc: impl Into<bson::Document>) -> Result<(), Error> {
        let collection = self.db.collection::<User>("users");
        let filter = doc! { "telegram_id": telegram_id };
        collection.update_one(filter, update_doc.into(), None).await?;
        Ok(())
    }

    pub async fn delete_user(&self, telegram_id: i64) -> Result<(), Error> {
        let collection = self.db.collection::<User>("users");
        let filter = doc! { "telegram_id": telegram_id };
        collection.delete_one(filter, None).await?;
        Ok(())
    }
}

