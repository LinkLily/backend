use actix_web::{
    get, post, patch, delete,
    HttpResponse,
    web::{Data, Json, Path},
    http::StatusCode
};
use serde_json::json;
use crate::{
    database::mongo::MongoRepo,
    routes::models::user::{UserRequest, UserEditRequest},
    utils::{hash_string, validate_password}
};


#[get("/{username}")]
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

#[post("")]
pub async fn create_user(db: Data<MongoRepo>, user: Json<UserRequest>) -> HttpResponse {
    if !validate_password(user.password.to_string()) {
        return HttpResponse::BadRequest().body("Invalid password: Too weak!");
    }

    let password = hash_string(user.password.to_owned()).unwrap();

    let data = UserRequest {
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        username: user.username.to_owned(),
        password: password.hash.clone()
    };

    db.create_user(data, password.salt).await
}

#[patch("/{username}")]
pub async fn edit_user(db: Data<MongoRepo>, path: Path<String>, new_data: Json<UserEditRequest>) -> HttpResponse {
    let username = path.into_inner();

    db.edit_user(username, new_data.into_inner()).await
}

#[delete("/{username}")]
pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let username = path.into_inner();

    db.delete_user(username).await
}

// This should probably be a post request instead but here it is for now
#[get("/exists/{type}/{query}")]
pub async fn check_user_exists(db: Data<MongoRepo>, path: Path<(String, String)>) -> HttpResponse {
    let (field_type, query_value) = path.into_inner();

    if field_type == "email" || field_type == "username" {
        let exists = db.check_user_exists(field_type, query_value).await;
        HttpResponse::Ok().json(json!({ "exists": exists }))
    } else {
        HttpResponse::BadRequest().body(format!("Invalid query type `{}`.", field_type))
    }

}

