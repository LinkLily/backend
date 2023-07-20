use std::env;
use rand::{
    Rng, thread_rng,
    distributions::Alphanumeric
};
use actix_web::{
    web::{Data, Path},
    get, HttpResponse, HttpRequest};
use log::kv::Source;
use redis::{AsyncCommands, cmd};
use crate::{
    models::api::{ApiKeyPair, ApiKey},
    utils::hash_string_with_salt,
    database::redis::{RedisPool}
};


#[get("/token/gen/{permission_level}")]
pub async fn gen_key(req: HttpRequest, db: Data<RedisPool>, path: Path<i8>) -> HttpResponse {
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

    let mut db_conn = db.get().await.unwrap();

    let new_api_key = ApiKey {
        id: None,
        hashed_api_key: key_hash,
        permission_level
    };

    let redis_res = cmd("LPUSH")
        .arg("api_keys")
        .arg(serde_json::to_string(&new_api_key).unwrap())
        .query_async::<_, ()>(&mut db_conn).await;

    match redis_res {
        Ok(res) => res,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string())
    }

    debug!("Wrote to redis database!");


    HttpResponse::Ok().json(
        ApiKeyPair {
            api_key: new_key.clone(),
            api_secret: new_secret.clone(),
            permission_level
        }
    )
}



