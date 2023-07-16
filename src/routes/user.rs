use crate::{
    models::{user::User},
    repo::mongo::MongoRepo,
    utils::hash_string
};
use actix_web::{get, post, HttpResponse, web::{Data, Json, Path}};
use actix_web::http::StatusCode;
use serde_json::json;

#[get("/user/{username}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {

    let username = path.into_inner();

    let user_response = db.get_user(username.clone()).await;
    match user_response {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError()
            .status(StatusCode::NOT_FOUND)
            .finish()
    }
}

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, user: Json<User>) -> HttpResponse {
    let password = hash_string(user.password.to_owned()).unwrap();

    let data = User {
        id: None,
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        username: user.username.to_owned(),
        password: Option::from(password.hash),
        salt: Option::from(password.salt)
    };

    db.create_user(data).await
}

// This should probably be a post request instead but here it is for now
#[get("/user/exists/{type}/{query}")]
pub async fn check_user_exists(db: Data<MongoRepo>, path: Path<(String, String)>) -> HttpResponse {
    let (field_type, query_value) = path.into_inner();

    if field_type == "email" || field_type == "username" {
        let exists = db.check_user_exists(field_type, query_value).await;
        HttpResponse::Ok().json(json!({ "exists": exists }))
    } else {
        HttpResponse::BadRequest().body(format!("Invalid query type `{}`.", field_type))
    }

}

