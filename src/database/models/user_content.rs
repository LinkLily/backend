use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DbUserContent {
    pub links: Vec<DbLink>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbLink {
    pub id: Uuid,
    pub for_username: String,
    pub order: i16,
    pub label: String,
    pub link: String,
    pub is_nsfw: bool
}

