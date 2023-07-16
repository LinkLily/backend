extern crate dotenvy;
use std::env;
use dotenvy::dotenv;
use clap::{Parser};
extern crate pretty_env_logger;
#[macro_use] extern crate log;
use actix_web::{get, HttpServer, App, web::Data, Responder};
use actix_web::dev::Service;
use futures::FutureExt;
use crate::{
    routes::{
        user::{
            get_user,
            create_user,
            check_user_exists
        }
    },
    repo::mongo::MongoRepo,
    utils::gen_api_key
};

mod models;
mod repo;
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
        gen_api_key("admin")
    }

    info!("Server starting...");

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {

        let cors = actix_cors::Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b".linklily.me")
            });

        App::new()
            .app_data(db_data.clone())
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
