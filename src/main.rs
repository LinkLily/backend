use actix_web::{HttpServer, App, web::Data, get, HttpResponse, Responder};
use repo::mongo::MongoRepo;
use crate::routes::{
    user::create_user
};

mod models;
mod repo;
mod routes;
mod utils;


#[get("/")]
async fn hello() -> impl Responder {
    "hiya!"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(hello)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
