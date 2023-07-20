use std::env;
use deadpool_redis::{Config, Pool, Runtime, CreatePoolError};

pub type RedisPool = Pool;

pub fn create_pool() -> Result<RedisPool, CreatePoolError> {
    let uri = env::var("REDIS_URI").unwrap();
    let config = Config::from_url(uri);
    let pool = config.create_pool(Some(Runtime::Tokio1));

    info!("Successfully connected to Redis database");

    pool
}
