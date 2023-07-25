use std::future::{ready, Ready};
use std::rc::Rc;
use actix_web::{dev::{
    forward_ready,
    Transform,
    Service,
    ServiceRequest,
    ServiceResponse
}, web::Data, body::EitherBody, Error, HttpResponse};
use futures::future::LocalBoxFuture;
use redis::cmd;
use sqlx::postgres::PgPool;
use crate::{
    database::{
        redis::RedisPool,
        models::api::DbApiKey
    },
};
use crate::database::redis::cache_api_keys;
use crate::utils::hash_string_with_salt;


pub struct ValidateApiToken;

impl<S: 'static, B> Transform<S, ServiceRequest> for ValidateApiToken
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ValidateApiTokenMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ValidateApiTokenMiddleware { service: Rc::new(service) }))
    }
}


pub struct ValidateApiTokenMiddleware<S> {
    service: Rc<S>
}

impl<S, B> Service<ServiceRequest> for ValidateApiTokenMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {

            /*
              If it can't find the hashed key in the Redis cache, it update the cache
              from the MongoDB database!
             */

            let mut update_cache = false;
            let mut updated_cache = false;

            let result = loop {
                let redis = req.app_data::<Data<RedisPool>>().clone().unwrap();

                if update_cache {
                    let mongo = req.app_data::<Data<PgPool>>().clone().unwrap();

                    cache_api_keys(redis.clone(), mongo.clone()).await;
                    updated_cache = true;
                }

                let headers = req.headers().clone();
                let api_token_header = headers.get("X-LinkLily-Auth-Key");
                let api_secret_header = headers.get("X-LinkLily-Auth-Secret");


                let mut conn = redis.get().await.unwrap();

                let redis_data = cmd("LRANGE")
                    .arg("api_keys")
                    .arg(0)
                    .arg(-1)
                    .query_async::<_, Vec<String>>(&mut conn)
                    .await
                    .unwrap()
                    .into_iter();


                if api_token_header.is_none() || api_secret_header.is_none() {
                    break Ok(
                        req.into_response(
                            HttpResponse::Unauthorized().finish()
                        ).map_into_right_body()
                    )
                } else {
                    let api_token_string = api_token_header.unwrap().to_str().unwrap().to_string();
                    let api_secret_string = api_secret_header.unwrap().to_str().unwrap().to_string();

                    let hashed_token = hash_string_with_salt(api_token_string, api_secret_string);

                    let mut is_authorized = false;
                    // let mut auth_level: i32 = 0;

                    for key_pair_string in redis_data {
                        let key_pair: DbApiKey = serde_json::from_str(&*key_pair_string).unwrap();

                        if hashed_token == key_pair.hashed_key {
                            is_authorized = true;
                            // auth_level = key_pair.permission_level
                        }
                    }

                    if is_authorized {
                        let res = svc.call(req).await?;

                        break Ok(res.map_into_left_body())
                    } else if updated_cache {
                        break Ok(
                            req.into_response(
                                HttpResponse::Unauthorized().finish()
                            ).map_into_right_body()
                        )
                    } else {
                        update_cache = true;
                        continue;
                    }
                };
            };

            result
        })
    }

}


