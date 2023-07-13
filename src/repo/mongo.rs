use std::env;

use mongodb::{
    bson::{extjson::de::Error, doc},
    results::{InsertOneResult},
    Client, Collection
};
use crate::models::user::User;


pub struct MongoRepo {
    col: Collection<User>
}

impl MongoRepo {
    pub async fn init() -> Self {
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading `MONGO_URI` environment variable")
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("LinkLily");
        let col: Collection<User> = db.collection("Users");

        info!("Successfully connected to MongoDB database `LinkLily`");

        MongoRepo { col }

    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_document = User {
            id: None, // None tells MongoDB to auto-generate a user ID
            name: new_user.name,
            username: new_user.username.clone(),
            password: new_user.password,
            salt: new_user.salt
        };

        let user = self
            .col
            .insert_one(new_document, None)
            .await
            .ok()
            .expect(&*format!("Error creating user with username `{}`!", new_user.username));
        Ok(user)
    }

    pub async fn get_user(&self, username: String) -> Result<User, Error> {

        let query_document = doc! {
            "username": username.clone()
        };

        let user_response = self
            .col
            .find_one(query_document, None)
            .await
            .ok()
            .expect(&*format!("Error: Couldn't find user with username `{}`!", username.clone()));

        match user_response {
            Some(user) => Ok(user),
            None => Err(
                Error::DeserializationError {
                    message: format!("Error: User `{}` not found!", username)
                }
            )
        }

    }
}

