use crate::{
    models::{user::User, util::Password},
    repo::mongo::MongoRepo,
    utils::hash_password
};
use actix_web::{
    post, web::{Data, Json}, HttpResponse
};
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};


#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, user: Json<User>) -> HttpResponse {

    let password = hash_password(user.password.to_owned()).unwrap();

    println!("{}", format!("Password hash: {}", password.hashed_password.clone()));

    let data = User {
        id: None,
        name: user.name.to_owned(),
        username: user.username.to_owned(),
        password: password.hashed_password,
        salt: Option::from(password.salt)
    };

    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

