use std::env;
use actix_web::web::Data;
use deadpool_redis::{Config, Pool, Runtime, CreatePoolError, Connection};
use redis::{cmd, RedisError};
use sqlx::postgres::PgPool;
use crate::database::{
    models::{
        api::DbApiKey
    }
};

pub type RedisPool = Pool;

pub fn create_pool() -> Result<RedisPool, CreatePoolError> {
    let uri = env::var("REDIS_CACHE_URI").unwrap();
    let config = Config::from_url(uri);
    let pool = config.create_pool(Some(Runtime::Tokio1));

    info!("Successfully connected to Redis database!");

    pool
}

pub async fn cache_api_keys(redis: Data<RedisPool>, db: Data<PgPool>) {
    let mut conn = redis.get().await.unwrap();

    info!("Caching API keys...");

    cmd("DEL")
        .arg("api_keys")
        .query_async::<_, ()>(&mut conn)
        .await.expect("Could not delete \"api_keys\" from Redis cache");


    let api_key_vec = sqlx::query!(
        r#"
        SELECT * FROM api_key
        "#
    ).fetch_all(&**db).await.unwrap();


    for key in api_key_vec {
        let api_key_redis = DbApiKey {
            id: key.id,
            hashed_key: key.hashed_key,
            permission_level: key.permission_level
        };

        cmd("LPUSH")
            .arg("api_keys")
            .arg(serde_json::to_string(&api_key_redis).unwrap())
            .query_async(&mut conn)
            .await
            .expect(
                &*format!(
                    "Couldn't write API key to Redis with ID {}",
                    key.id.to_string()
                )
            )

    }


    info!("Succesfully cached API keys!");
}

pub async fn clear_cache(redis: Data<RedisPool>) -> Result<Connection, RedisError> {
    let mut conn = redis.get().await.unwrap();

    info!("Clearing Redis cache...");

    cmd("FLUSHDB")
        .query_async::<_, ()>(&mut conn)
        .await.expect("Couldn't clear Redis cache");

    info!("Cleared Redis cache!");

    Ok(conn)
}
