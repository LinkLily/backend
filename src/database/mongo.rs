use std::env;
use actix_web::body::BoxBody;
use actix_web::HttpResponse;
use futures::TryStreamExt;
use chrono::Utc;

use mongodb::{
    bson::{extjson::de::Error, doc},
    Client, Collection, IndexModel
};
use mongodb::options::{IndexOptions, UpdateModifications};

use crate::{
    database::models::{
        user::UserMongo,
        api::ApiKeyMongo
    },
    models::{
        user::User
    },
    routes::models::user::{UserRequest, UserEditRequest},
    utils::{hash_string, validate_password}
};


pub struct MongoRepo {
    user_col: Collection<UserMongo>,
    api_key_col: Collection<ApiKeyMongo>
}

impl MongoRepo {
    pub async fn init() -> Self {
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading `MONGO_URI` environment variable")
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("LinkLily");

        let user_col: Collection<UserMongo> = db.collection("Users");
        let api_key_col: Collection<ApiKeyMongo> = db.collection("API Keys");

        let index_options = IndexOptions::builder().unique(true).build();

        let username_model = IndexModel::builder()
            .keys(doc! { "username": 1u32 })
            .options(index_options.clone())
            .build();
        let email_model = IndexModel::builder()
            .keys(doc! { "email": 1u32 })
            .options(index_options.clone())
            .build();

        user_col.create_index(email_model, None)
            .await.expect("Error: Couldn't create index for `Users` collection");
        user_col.create_index(username_model, None)
            .await.expect("Error: Couldn't create index for `Users` collection");


        info!("Successfully connected to MongoDB database `LinkLily`");

        MongoRepo { user_col, api_key_col }

    }

    pub async fn create_user(&self, new_user: UserRequest, salt: String) -> HttpResponse {
        let new_document = UserMongo {
            id: None, // None tells MongoDB to auto-generate a user ID
            name: new_user.name,
            email: new_user.email,
            username: new_user.username.clone(),
            password: new_user.password,
            salt: salt,
            created_at: Utc::now().to_string(),
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
            Some(user) => Ok(
                User {
                    name: user.name,
                    username: user.username,
                    created_at: user.created_at
                }
            ),
            None => Err(
                Error::DeserializationError {
                    message: format!("Error: User `{}` not found!", username)
                }
            )
        }

    }

    pub async fn edit_user(&self, username: String, new_data: UserEditRequest) -> HttpResponse {
        let query = doc! {
            "username": username
        };

        let mut new_data_doc = doc! {};

        if new_data.name.is_some() {
            new_data_doc.insert("name", new_data.name);
        }
        if new_data.email.is_some() {
            new_data_doc.insert("email", new_data.email);
        }
        if new_data.username.is_some() {
            new_data_doc.insert("username", new_data.username);
        }
        if new_data.password.is_some() {
            if !validate_password(new_data.password.clone().unwrap()) {
                return HttpResponse::BadRequest().body("Invalid password: Too weak!");
            }

            let new_password =
                hash_string(new_data.password.unwrap()).unwrap();

            new_data_doc.insert("password", new_password.hash);
            new_data_doc.insert("salt", new_password.salt);
        }


        let update_data_doc = doc! {
            "$set": new_data_doc
        };

        let update_res = self
            .user_col
            .find_one_and_update(
                query,
                UpdateModifications::from(update_data_doc),
                None
            ).await;

        match update_res {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => HttpResponse::NotFound().body(err.to_string())
        }
    }

    pub async fn delete_user(&self, username: String) -> HttpResponse {
        let query_doc = doc! {
            "username": username
        };

        let delete_res = self
            .user_col
            .find_one_and_delete(query_doc, None)
            .await;

        match delete_res {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => HttpResponse::NotFound().body(err.to_string())
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

    pub async fn write_api_key(&self, key: String, permission_level: i8) -> Result<(), Error> {
        let new_api_key = ApiKeyMongo {
            id: None,
            hashed_api_key: key,
            permission_level
        };

        self
            .api_key_col
            .insert_one(new_api_key, None)
            .await.expect("Error writing API key to database");

        Ok(())
    }

    pub async fn get_all_api_keys(&self) -> Vec<ApiKeyMongo> {

        let search = match self.api_key_col.find(None, None).await {
            Ok(cursor) => cursor,
            Err(_) => return vec![]
        };

        search.try_collect().await.unwrap()
    }

}

