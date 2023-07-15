use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: Option<String>,
    pub email: String,
    pub username: String,
    pub password: Option<String>,
    pub salt: Option<String>
}

