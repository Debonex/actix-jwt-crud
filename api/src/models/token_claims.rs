use actix_web::{FromRequest, HttpMessage};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

use crate::error::{ServerError, ServerResult};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TokenClaims {
    pub user_id: i32,
    pub iat: i64,
    pub exp: i64,
}

impl FromRequest for TokenClaims {
    type Error = ServerError;
    type Future = Ready<ServerResult<Self>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.extensions().get::<TokenClaims>() {
            Some(claims) => ready(Ok(*claims)),
            None => ready(Err(ServerError::InternalError)),
        }
    }
}
