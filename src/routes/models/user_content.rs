use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order: i32,
    pub label: String,
    pub link: String,
    pub is_nsfw: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkRequest {
    pub user_id: Uuid,
    pub label: String,
    pub link: String,
    pub is_nsfw: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkEditRequest {
    pub order: Option<i32>,
    pub label: Option<String>,
    pub link: Option<String>,
    pub is_nsfw: Option<bool>
}

