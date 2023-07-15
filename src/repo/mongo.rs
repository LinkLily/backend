use std::env;
use actix_web::body::BoxBody;
use actix_web::HttpResponse;

use mongodb::{
    bson::{extjson::de::Error, doc},
    Client, Collection, IndexModel
};
use mongodb::options::IndexOptions;
use crate::models::user::User;


pub struct MongoRepo {
    user_col: Collection<User>
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

        let index_options = IndexOptions::builder().unique(true).build();

        let username_model = IndexModel::builder()
            .keys(doc! { "username": 1u32 })
            .options(index_options.clone())
            .build();
        let email_model = IndexModel::builder()
            .keys(doc! { "email": 1u32 })
            .options(index_options.clone())
            .build();

        col.create_index(email_model, None)
            .await.expect("Error: Couldn't create index for `Users` collection");
        col.create_index(username_model, None)
            .await.expect("Error: Couldn't create index for `Users` collection");


        info!("Successfully connected to MongoDB database `LinkLily`");

        MongoRepo { user_col: col }

    }

    pub async fn create_user(&self, new_user: User) -> HttpResponse {
        let new_document = User {
            id: None, // None tells MongoDB to auto-generate a user ID
            name: new_user.name,
            email: new_user.email,
            username: new_user.username.clone(),
            password: new_user.password,
            salt: new_user.salt
        };

        let user = self
            .user_col
            .insert_one(new_document, None)
            .await;

        match user {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => {
                HttpResponse::Conflict().body(BoxBody::new("Username or email taken."))
            }
        }
    }

    pub async fn get_user(&self, username: String) -> Result<User, Error> {

        let query_document = doc! {
            "username": username.clone()
        };

        let user_response = self
            .user_col
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

    pub async fn check_user_exists(&self, query_type: String, query_string: String) -> bool {
        match query_type.as_str() {
            "email" => {
                let query_doc = doc! {
                    "email": query_string.clone()
                };

                let query_response = self
                    .user_col
                    .find_one(query_doc, None)
                    .await
                    .ok()
                    .expect(&*format!(
                        "Error finding user with email `{}`", query_string.clone()
                    ));

                match query_response {
                    Some(_) => true,
                    None => false
                }
            },
            "username" => {
                let query_doc = doc! {
                    "username": query_string.clone()
                };

                let query_response = self
                    .user_col
                    .find_one(query_doc, None)
                    .await
                    .ok()
                    .expect(&*format!(
                        "Error finding user with username `{}`", query_string.clone()
                    ));

                match query_response {
                    Some(_) => true,
                    None => false
                }
            },
            _ => {
                panic!("{}", format!(
                    "Called `check_user_exists` with invalid type `{}`!",
                    query_string
                ))
            }




        }


    }

}

