use std::env;
use sqlx::{
    pool::Pool,
    postgres::PgPool,
    Postgres, Error
};
use actix_web::web::Data;

pub async fn create_pool() -> Result<Pool<Postgres>, Error> {
    let conn_uri = env::var("DATABASE_URL").unwrap();
    let conn = PgPool::connect(&*conn_uri).await;

    info!("Successfully connected to Postgres database!");

    conn
}

pub async fn is_username_available(db: Data<PgPool>, username: String) -> bool {
    let query = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM \"user\" WHERE username = $1)",
        username.to_owned()
    ).fetch_one(&**db).await;

    return !query.unwrap().exists.unwrap()
}

pub async fn is_email_available(db: Data<PgPool>, email: String) -> bool {
    let query = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM \"user\" WHERE email = $1)",
        email.to_owned()
    ).fetch_one(&**db).await;

    return !query.unwrap().exists.unwrap()
}

