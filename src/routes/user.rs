use crate::{
    models::{user::User},
    repo::mongo::MongoRepo,
    utils::hash_password
};
use actix_web::{get, post, HttpResponse, web::{Data, Json, Path}};
use actix_web::http::StatusCode;

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

    let password = hash_password(user.password.to_owned()).unwrap();

    let data = User {
        id: None,
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        username: user.username.to_owned(),
        password: Option::from(password.hashed_password),
        salt: Option::from(password.salt)
    };

    db.create_user(data).await
}

