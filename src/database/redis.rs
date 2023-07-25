use std::env;
use actix_web::web::Data;
use deadpool_redis::{Config, Pool, Runtime, CreatePoolError, Connection};
use redis::cmd;
use crate::database::{
    // models::api::{
    //     ApiKeyRedis
    // }
};

pub type RedisPool = Pool;

pub fn create_pool() -> Result<RedisPool, CreatePoolError> {
    let uri = env::var("REDIS_URI").unwrap();
    let config = Config::from_url(uri);
    let pool = config.create_pool(Some(Runtime::Tokio1));

    info!("Successfully connected to Redis database!");

    pool
}

// pub async fn rebuild_cache(redis: Data<RedisPool>, mongo: Data<MongoRepo>) {
//     let mut conn = clear_cache(redis).await;
//
//     info!("Rebuilding Redis cache...");
//
//     let mongo_vec = mongo.get_all_api_keys().await;
//
//
//     for key in mongo_vec {
//         let api_key_redis = ApiKeyRedis {
//             hashed_api_key: key.hashed_api_key,
//             permission_level: key.permission_level
//         };
//
//         cmd("LPUSH")
//             .arg("api_keys")
//             .arg(serde_json::to_string(&api_key_redis).unwrap())
//             .query_async(&mut conn)
//             .await
//             .expect(
//                 &*format!(
//                     "Couldn't write API key to Redis with object ID {}",
//                     key.id.unwrap().to_string()
//                 )
//             )
//
//     }
//
//
//     info!("Rebuilt Redis cache!");
// }

// pub async fn clear_cache(redis: Data<RedisPool>) -> Connection {
//     let mut conn = redis.get().await.unwrap();
//
//     info!("Clearing Redis cache...");
//
//     cmd("FLUSHDB")
//         .query_async::<_, ()>(&mut conn)
//         .await.expect("Couldn't clear Redis cache");
//
//     info!("Cleared Redis cache!");
//
//     conn
// }
