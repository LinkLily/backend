use actix_web::{post, HttpResponse, web::{Json, Data}};
use actix_session::Session;
use argon2::{PasswordVerifier, PasswordHash, Argon2};
use sqlx::{PgPool, Error};
use crate::{
    routes::models::auth::UserLogin,
    database::models::user::DbUser,
};


#[post("/login")]
pub async fn login(session: Session, db: Data<PgPool>, login: Json<UserLogin>) -> HttpResponse {

    let username = login.username.clone();

    let db_user_req: Result<DbUser, Error> = sqlx::query_as!(
        DbUser,
        r#"SELECT * FROM "user" WHERE username = $1"#,
        username.clone()
    ).fetch_one(&**db).await;

    let db_user = match db_user_req {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().body(
            format!("User with the username `{}` does not exist.", username.clone())
        )
    };
    
    let parsed_hash = PasswordHash::new(&db_user.password.as_ref()).unwrap();
    if Argon2::default().verify_password(login.password.clone().as_bytes(), &parsed_hash).is_ok() {
        let _ = session.insert("user_id", &db_user.id.clone());
        let _ = session.insert("user_role", &db_user.role.clone());
        session.renew();

        return HttpResponse::Ok().finish()
    }
    
    return HttpResponse::Unauthorized().body("Invalid password.")
}

#[post("/logout")]
pub async fn logout(session: Session) -> HttpResponse {
    let id: Option<String> = session.get("user_id").unwrap();

    match id {
        Some(_) => {
            session.purge();
            return HttpResponse::Ok().finish()
        },
        None => {
            return HttpResponse::BadRequest().finish()
        }
    }
}

