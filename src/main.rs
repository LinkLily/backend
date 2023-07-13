extern crate dotenvy;

use std::env;
use dotenvy::dotenv;
extern crate pretty_env_logger;
#[macro_use] extern crate log;
use actix_web::{HttpServer, App, web::Data, get, Responder};
use repo::mongo::MongoRepo;
use crate::routes::{
    user::{get_user, create_user}
};

mod models;
mod repo;
mod routes;
mod utils;


#[get("/")]
async fn root() -> impl Responder {
    "Root route :)"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(root)
            .service(get_user)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
