use rand::{
    Rng, thread_rng,
    distributions::Alphanumeric
};
use actix_web::{
    get, post,
    web::{Data, Path},
    HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use crate::{
    routes::models::api::ApiKeyPair,
    utils::hash_string_with_salt, 
    database::redis::{
        RedisPool, clear_cache
    },
};


#[get("/token/gen/{permission_level}")]
pub async fn gen_key(db: Data<PgPool>, path: Path<i8>) -> HttpResponse {
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

#[post("/clear-cache")]
pub async fn post_clear_cache(redis: Data<RedisPool>) -> HttpResponse {
    let res = clear_cache(redis).await;

    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


