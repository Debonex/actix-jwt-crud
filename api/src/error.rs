use actix_web::{http::StatusCode, ResponseError};
use derive_more::Display;
use log::error;

#[derive(Debug, Display)]
pub enum ServerError {
    InternalError,
}

impl ResponseError for ServerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub type ServerResult<T> = Result<T, ServerError>;

pub fn internal<E>(e: E) -> ServerError
where
    E: std::error::Error,
{
    error!("{}", e);
    ServerError::InternalError
}
