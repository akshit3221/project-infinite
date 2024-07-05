

use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub email_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserResponse {
    pub id: ObjectId,
    pub name: String,
    pub email_id: String,
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            name: self.name.clone(),
            email_id: self.email_id.clone(),
        }
    }
}
