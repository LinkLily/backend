use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use crate::models::user::UserLinks;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserContentMongo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user: String,
    pub links: Vec<UserLinks>
}

