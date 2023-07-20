#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate dotenvy;
use std::ops::Deref;
use dotenvy::dotenv;
use clap::{Parser};
use actix_web::{get, HttpServer, App, web::Data, Responder, web};
use actix_web::dev::Service;
use futures::FutureExt;
use redis;
use crate::{
    routes::{
        user::{
            get_user,
            create_user,
            check_user_exists
        },
        api::gen_key
    },
    database::{
        mongo::MongoRepo,
        redis::rebuild_cache
    },
    utils::gen_api_key,
    middleware::validate_api_token::ValidateApiToken
};

mod database;
mod middleware;
mod models;
mod routes;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long,
        default_missing_value = "true",
        help = "Generates, stores, and prints an API key.")]
    gen_api_key: bool
}


#[get("/")]
async fn root() -> impl Responder {
    "Root route :)"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let args = Args::parse();

    if args.gen_api_key {
        /*
            3 - Admin User
            2 - Privileged User - Official site will use this
            1 - Basic User
        */
        gen_api_key(2)
    }

    info!("Server starting...");


    let redis_pool = database::redis::create_pool().unwrap();
    let redis_data = Data::new(redis_pool);

    let mongo_db = MongoRepo::init().await;
    let mongo_data = Data::new(mongo_db);

    rebuild_cache(redis_data.clone(), mongo_data.clone()).await;

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b".linklily.me")
            });

        App::new()
            .app_data(redis_data.clone())
            .app_data(mongo_data.clone())
            .wrap(ValidateApiToken)
            .wrap(cors)
            .service(root)
            .service(get_user)
            .service(create_user)
            .service(check_user_exists)
            .service(
                web::scope("/admin")
                    .service(gen_key)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
