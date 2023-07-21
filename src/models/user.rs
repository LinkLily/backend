use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: Option<String>,
    pub username: String,
    pub created_at: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserContent {
    pub user: String,
    pub links: Vec<UserLinks>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLinks {
    pub id: i16,
    pub label: String,
    pub link: String,
    pub order_id: i16,
    pub featured: bool
}


