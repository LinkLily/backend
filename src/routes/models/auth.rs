use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct ServerIdentifier {
    pub user_id: Uuid,
    pub username: String,
    pub role: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String
}
