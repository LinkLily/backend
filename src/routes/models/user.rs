use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    pub name: Option<String>,
    pub email: String,
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: Option<String>,
    pub username: String,
    pub created_at: String,
    pub can_edit: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEditRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserExistsRequest {
    #[serde(rename = "type")]
    pub exists_type: String,
    pub value: String
}

