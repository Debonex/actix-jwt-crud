use crate::{
    error::ServerError,
    services::auth::{decode_token, get_token},
};
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use redis::Client;
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    rc::Rc,
};

pub struct AuthMiddlewareFactory;
pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type InitError = ();
    type Response = ServiceResponse<EitherBody<B>>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type Transform = AuthMiddleware<S>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Error = Error;
    type Response = ServiceResponse<EitherBody<B>>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            let mut auth_pass = false;
            let token = req
                .headers()
                .get("AUTHORIZATION")
                .and_then(|auth_header| auth_header.to_str().ok())
                .filter(|auth_str| {
                    auth_str.starts_with("bearer ") || auth_str.starts_with("Bearer ")
                })
                .map(|auth_str| &auth_str[7..])
                .map(str::trim)
                .unwrap_or("");

            if !token.is_empty() {
                if let Ok(token_data) = decode_token(token) {
                    let redis_client = req
                        .app_data::<web::Data<Client>>()
                        .ok_or(ServerError::RedisError)?;
                    let current_token = get_token(redis_client, token_data.claims.user_id)
                        .await
                        .map_err(|_| ServerError::AuthInvalid)?;
                    if current_token.eq(token) {
                        let mut extensions = req.extensions_mut();
                        extensions.insert(token_data.claims);
                        auth_pass = true;
                    }
                    if !auth_pass {
                        return ServerError::AuthExpired.into();
                    }
                }
            }

            if !auth_pass {
                return ServerError::AuthInvalid.into();
            }

            let res = service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
