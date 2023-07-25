use std::env;
use rand::{
    Rng, thread_rng,
    distributions::Alphanumeric
};
use actix_web::{
    web::{Data, Path},
    get, HttpResponse, HttpRequest};
use sqlx::PgPool;
use uuid::Uuid;
use crate::{
    models::api::ApiKeyPair,
    utils::hash_string_with_salt,
};


#[get("/token/gen/{permission_level}")]
pub async fn gen_key(req: HttpRequest, db: Data<PgPool>, path: Path<i8>) -> HttpResponse {
    let key_gen_env = env::var("ADMIN_TOKEN").unwrap();
    let key_gen_header = req.headers().get("X-LinkLily-Admin-Token");
    let key_gen_header_string;
    match key_gen_header {
        Some(header) => key_gen_header_string = header.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().finish()
    };

    if key_gen_header_string != key_gen_env {
        return HttpResponse::Unauthorized().finish();
    }


    let new_key: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let new_secret: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let key_hash: String = hash_string_with_salt(
        new_key.clone(), new_secret.clone()
    );

    let permission_level = path.into_inner();
    match permission_level {
        1 => (),
        2 => (),
        3 => (),
        other => return HttpResponse::BadRequest().body(format!("Invalid permission level `{other}`!"))
    };


    let db_res = sqlx::query!(
        r#"
        INSERT INTO "api_key" (id, hashed_key, permission_level)
        VALUES ($1, $2, $3)
        "#, Uuid::new_v4(), key_hash, i32::from(permission_level)
    ).execute(&**db).await;

    match db_res {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().finish()
    };


    debug!("Wrote to database!");

    // rebuild_cache(redis, db).await;


    HttpResponse::Ok().json(
        ApiKeyPair {
            api_key: new_key.clone(),
            api_secret: new_secret.clone(),
            permission_level
        }
    )
}



