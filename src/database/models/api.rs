use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyMongo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub hashed_api_key: String,
    pub permission_level: i8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyRedis {
    pub hashed_api_key: String,
    pub permission_level: i8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyMapMongo {
    pub api_keys: Vec<ApiKeyMongo>
}


