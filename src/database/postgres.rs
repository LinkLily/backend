use std::env;
use sqlx::{
    pool::Pool,
    postgres::PgPool,
    Postgres, Error
};

pub async fn create_pool() -> Result<Pool<Postgres>, Error> {
    let conn_uri = env::var("DATABASE_URL").unwrap();
    let conn = PgPool::connect(&*conn_uri).await;

    info!("Successfully connected to Postgres database!");

    conn
}



