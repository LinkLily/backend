use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    pub hashed_password: String,
    pub salt: String
}
