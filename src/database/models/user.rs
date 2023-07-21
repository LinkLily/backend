use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMongo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: Option<String>,
    pub email: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub created_at: String
}

