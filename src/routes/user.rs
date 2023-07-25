use actix_web::{
    get, post, delete,
    HttpResponse,
    web::{Data, Json, Path}
};
use serde_json::json;
use sqlx::PgPool;
use chrono::Utc;
use crate::{
    database::models::user::DbUser,
    routes::models::user::{UserRequest, UserEditRequest},
    utils::{hash_string, validate_password},
    models::user::User
};


#[get("/{username}")]
pub async fn get_user(db: Data<PgPool>, path: Path<String>) -> HttpResponse {
    let username = path.into_inner();

    let db_res = sqlx::query_as!(
        DbUser,
        r#"SELECT * FROM "user" WHERE username = $1"#,
        username
    ).fetch_one(&**db).await;

    match db_res {
        Ok(res) => {
            let user_res = User {
                name: res.name,
                username: res.username,
                created_at: res.created_at
            };
            return HttpResponse::Ok().json(user_res);
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[post("")]
pub async fn create_user(db: Data<PgPool>, user: Json<UserRequest>) -> HttpResponse {
    if !validate_password(user.password.to_string()) {
        return HttpResponse::BadRequest().body("Invalid password: Too weak!");
    }

    let password = hash_string(user.password.to_owned()).unwrap();

    let current_time = Utc::now().to_string();

    let query = sqlx::query!(
        r#"
        INSERT INTO "user" (username, name, avatar_url, created_at, email, password, salt)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        user.username, user.name, "", current_time, user.email, password.hash, password.salt

    ).execute(&**db).await;

    match query {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

// #[patch("/{username}")]
// pub async fn edit_user(db: Data<PgPool>, path: Path<String>, new_data: Json<UserEditRequest>) -> HttpResponse {
//     let username = path.into_inner();
//
//     /*
//       This might be a pain :')
//     */
//
//     db.edit_user(username, new_data.into_inner()).await
// }

#[delete("/{username}")]
pub async fn delete_user(db: Data<PgPool>, path: Path<String>) -> HttpResponse {
    let username = path.into_inner();

    let query = sqlx::query!(
        r#"
        DELETE FROM "user"
        WHERE username = $1
        "#, username
    ).execute(&**db).await;

    match query {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

// This should probably be a post request instead but here it is for now
// #[get("/exists/{type}/{query}")]
// pub async fn check_user_exists(db: Data<PgPool>, path: Path<(String, String)>) -> HttpResponse {
//     let (field_type, query_value) = path.into_inner();

//     if field_type == "email" || field_type == "username" {
//         let exists = db.check_user_exists(field_type, query_value).await;
//         HttpResponse::Ok().json(json!({ "exists": exists }))
//     } else {
//         HttpResponse::BadRequest().body(format!("Invalid query type `{}`.", field_type))
//     }

// }

