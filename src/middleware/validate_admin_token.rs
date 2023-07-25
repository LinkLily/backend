use std::future::{ready, Ready};
use std::rc::Rc;
use std::env;
use actix_web::{
    dev::{
        forward_ready,
        Transform,
        Service,
        ServiceRequest,
        ServiceResponse
    }, 
    body::EitherBody, 
    HttpResponse,
    Error,
};
use futures::future::LocalBoxFuture;


pub struct ValidateAdminToken;

impl<S: 'static, B> Transform<S, ServiceRequest> for ValidateAdminToken
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ValidateAdminTokenMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ValidateAdminTokenMiddleware { service: Rc::new(service) }))
    }
}


pub struct ValidateAdminTokenMiddleware<S> {
    service: Rc<S>
}

impl<S, B> Service<ServiceRequest> for ValidateAdminTokenMiddleware<S>
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

            let admin_token = env::var("ADMIN_TOKEN").unwrap();

            let admin_header = req.headers().get("X-LinkLily-Admin-Token");

            if admin_header.is_none() {
                return Ok(req.into_response(
                    HttpResponse::Unauthorized().finish()
                ).map_into_right_body())
            } else if admin_header.unwrap().to_str().unwrap() == admin_token {
                return Ok(req.into_response(
                    HttpResponse::Unauthorized().finish()
                ).map_into_right_body())
            }
            let res = svc.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }

}


