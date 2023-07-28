use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyPair {
    pub api_key: String,
    pub api_secret: String,
    pub permission_level: i8
}

