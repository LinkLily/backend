use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub hashed_api_key: String,
    pub permission_level: i8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyPair {
    pub api_key: String,
    pub api_secret: String,
    pub permission_level: i8
}

