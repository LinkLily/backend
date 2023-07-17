#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate dotenvy;
use std::env;
use dotenvy::dotenv;
use clap::{Parser};
use actix_web::{get, HttpServer, App, web::Data, Responder};
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
    database::mongo::MongoRepo,
    utils::gen_api_key
};

mod database;
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
    env::set_var("RUST_LOG", "info");
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


    let redis_uri = env::var("REDIS_URI").unwrap();
    let redis = redis::Client::open(redis_uri).unwrap();


    let mongo_db = MongoRepo::init().await;
    let mongo_db_data = Data::new(mongo_db);

    HttpServer::new(move || {

        let cors = actix_cors::Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b".linklily.me")
            });

        App::new()
            .app_data(Data::new(redis.clone()))
            .app_data(mongo_db_data.clone())
            .wrap_fn(|req, srv| {
                let headers = req.headers().clone();

                srv.call(req).map(move |res| {
                    let auth_header = headers.get("X-LinkLily-Auth-Token");
                    match auth_header {
                        Some(header) => header,
                        None => return Err(actix_web::error::ErrorUnauthorized(""))
                    };
                    let auth_header_string = auth_header
                        .unwrap().to_str().unwrap();

                    let api_key = env::var("API_KEY").unwrap();

                    return if auth_header_string != api_key {
                        Err(actix_web::error::ErrorUnauthorized(""))
                    } else {
                        res
                    }
                })
            })
            .wrap(cors)
            .service(root)
            .service(get_user)
            .service(create_user)
            .service(check_user_exists)
            .service(gen_key)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
