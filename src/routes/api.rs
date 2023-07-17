use std::env;
use actix_web::{
    web::{Data, Json},
    post, HttpResponse, HttpRequest};
use redis::Client;
use crate::{
    models::api::{ApiKeyPair, ApiKey},
};


#[post("/api/generate")]
pub async fn gen_key(req: HttpRequest, redis: Data<Client>, key_pair: Json<ApiKeyPair>) -> HttpResponse {

    let key_gen_env = env::var("KEYGEN_TOKEN").unwrap();
    let key_gen_header = req.headers().get("X-LinkLily-KeyGen");
    let key_gen_header_string;
    match key_gen_header {
        Some(header) => key_gen_header_string = header.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().finish()
    };

    if key_gen_header_string != key_gen_env {
        return HttpResponse::Unauthorized().finish();
    }

    let key_pair_res = ApiKeyPair {
        api_key: key_pair.api_key.clone(),
        api_secret: key_pair.api_secret.clone(),
        permission_level: key_pair.permission_level.clone()
    };

    HttpResponse::Ok().json(key_pair_res)
}



