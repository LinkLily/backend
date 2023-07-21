use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    pub name: Option<String>,
    pub email: String,
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEditRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>
}

