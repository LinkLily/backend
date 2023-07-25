use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DbUser {
    pub username: String,
    pub name: Option<String>,
    pub avatar_url: String,
    pub created_at: String,
    pub email: String,
    pub password: String,
    pub salt: String
}

