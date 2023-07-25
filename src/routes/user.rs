use actix_web::{
    get, post, patch, delete,
    HttpResponse,
    web::{Data, Json, Path}
};
use serde_json::json;
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use crate::{
    database::{
        models::user::DbUser,
        postgres::{is_username_available, is_email_available}
    },
    routes::models::user::{UserRequest, UserEditRequest, UserExistsRequest},
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
                created_at: res.created_at.to_string()
            };
            return HttpResponse::Ok().json(user_res);
        },
        Err(_) => HttpResponse::NotFound().finish()
    }
}

#[post("")]
pub async fn create_user(db: Data<PgPool>, user: Json<UserRequest>) -> HttpResponse {
    if !is_username_available(db.clone(), user.username.to_string()).await {
        return HttpResponse::Conflict().body(
            format!("User with username `{}` already exists.", user.username.to_string())
        )
    }
    if !is_email_available(db.clone(), user.email.to_string()).await {
        return HttpResponse::Conflict().body(
            format!("User with email `{}` already exists.", user.email.to_string())
        )
    }

    if !validate_password(user.password.to_string()) {
        return HttpResponse::BadRequest().body("Invalid password: Too weak!");
    }

    let password = hash_string(user.password.to_owned()).unwrap();

    let current_time = Utc::now();

    let query = sqlx::query!(
        r#"
        INSERT INTO "user" (id, username, name, avatar_url, created_at, email, password, salt)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        Uuid::new_v4(), user.username, user.name, "", current_time.naive_utc(), user.email, password.hash, password.salt

    ).execute(&**db).await;

    match query {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[patch("/{username}")]
pub async fn edit_user(db: Data<PgPool>, path: Path<String>, new_data: Json<UserEditRequest>) -> HttpResponse {
    let username = path.into_inner();


    // Name
    if new_data.name.is_some() {
        let query = sqlx::query!(
            r#"
            UPDATE "user"
            SET name = $2
            WHERE username = $1
            "#, username, new_data.name.as_ref().unwrap()
        ).execute(&**db).await;

        if query.is_err() {
            return HttpResponse::InternalServerError().body(
                query.err().unwrap().to_string()
            );
        }
    }
    // Email
    if new_data.email.is_some() {
        if !is_email_available(db.clone(), new_data.email.clone().unwrap().to_string()).await {
            return HttpResponse::Conflict().body(
                format!(
                    "User with email `{}` already exists.", 
                    new_data.email.clone().unwrap().to_string()
                )
            )
        }

        let query = sqlx::query!(
            r#"
            UPDATE "user"
            SET email = $2
            WHERE username = $1
            "#, username, new_data.email.as_ref().unwrap()
        ).execute(&**db).await;

        if query.is_err() {
            return HttpResponse::InternalServerError().body(
                query.err().unwrap().to_string()
            );
        }
    }
    // Username
    if new_data.username.is_some() {
        if !is_username_available(db.clone(), new_data.username.clone().unwrap().to_string()).await {
            return HttpResponse::Conflict().body(
                format!(
                    "User with username `{}` already exists.", 
                    new_data.username.clone().unwrap().to_string()
                )
            )
        }

        let query = sqlx::query!(
            r#"
            UPDATE "user"
            SET username = $2
            WHERE username = $1
            "#, username, new_data.username.as_ref().unwrap()
        ).execute(&**db).await;

        if query.is_err() {
            return HttpResponse::InternalServerError().body(
                query.err().unwrap().to_string()
            );
        }
    }
    // Password
    if new_data.password.is_some() {
        let new_pass_hash = hash_string(new_data.password.as_ref().unwrap().to_string()).unwrap();

        let query = sqlx::query!(
            r#"
            UPDATE "user"
            SET password = $2, salt = $3
            WHERE username = $1
            "#, username, new_pass_hash.hash, new_pass_hash.salt
        ).execute(&**db).await;

        if query.is_err() {
            return HttpResponse::InternalServerError().body(
                query.err().unwrap().to_string()
            );
        }
    }


    HttpResponse::Ok().finish()
}

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

#[post("/exists")]
pub async fn check_user_exists(db: Data<PgPool>, exists: Json<UserExistsRequest>) -> HttpResponse {
    if exists.exists_type == "email" {
        let does_exist = !is_email_available(db.clone(), exists.value.clone()).await;

        return HttpResponse::Ok().json(json!({ "exists": does_exist }))
    } else if exists.exists_type == "username" {
        let does_exist = !is_username_available(db.clone(), exists.value.clone()).await;

        return HttpResponse::Ok().json(json!({ "exists": does_exist }))
    } else {
        return HttpResponse::BadRequest().finish()
    }
}

