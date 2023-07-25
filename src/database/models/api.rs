use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DbApiKey {
    pub id: Uuid,
    pub hashed_key: String,
    pub permission_level: i32
}
