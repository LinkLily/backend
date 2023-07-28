#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate dotenvy;
use std::env;

use dotenvy::dotenv;
use clap::Parser;
use actix_web::{get, HttpServer, App, web::Data, Responder, web};
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_session::{SessionMiddleware, storage::RedisActorSessionStore};
use crate::{
    routes::{
        api::*,
        auth::*,
        user::*,
        user_content::*
    },
    utils::gen_api_key,
    middleware::{
        validate_admin_token::ValidateAdminToken,
        validate_api_token::ValidateApiToken
    }
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

    let rate_limit_conf = GovernorConfigBuilder::default()
        .per_second(3)
        .burst_size(20)
        .finish()
        .unwrap();


    let keygen_token = env::var("KEYGEN_TOKEN").unwrap();

    let redis_sessions_url = env::var("REDIS_SESSIONS_URL").unwrap();

    let redis_pool = database::redis::create_pool().unwrap();
    let redis_data = Data::new(redis_pool);

    let pg_pool = database::postgres::create_pool()
        .await.unwrap();

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b".linklily.me")
            });

        App::new()
            .app_data(redis_data.clone())
            .app_data(Data::new(pg_pool.clone()))
            .wrap(cors)
            .wrap(Governor::new(&rate_limit_conf))
            .service(root)
            .service(
                web::scope("/api")
                    .wrap(ValidateApiToken)
                    .wrap(
                        SessionMiddleware::builder(
                            RedisActorSessionStore::new(redis_sessions_url.clone()),
                            actix_web::cookie::Key::from(keygen_token.as_bytes())
                        )
                        .build(),
                    )
                    .service(
                        web::scope("/user")
                            .service(get_user)
                            .service(create_user)
                            .service(edit_user)
                            .service(delete_user)
                            .service(check_user_exists)
                            .service(
                                web::scope("/content")
                                    .service(get_links)
                                    .service(create_link)
                                    .service(edit_link)
                                    .service(delete_link)
                            )
                    )
                    .service(
                        web::scope("/auth")
                        .service(login)
                        .service(logout)
                    )
            )
            .service(
                web::scope("/admin")
                    .wrap(ValidateAdminToken)
                    .service(gen_key)
                    .service(post_clear_cache)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
