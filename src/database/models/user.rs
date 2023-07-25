use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DbUser {
    pub id: Uuid,
    pub username: String,
    pub name: Option<String>,
    pub avatar_url: String,
    pub created_at: NaiveDateTime,
    pub email: String,
    pub password: String,
    pub salt: String
}

