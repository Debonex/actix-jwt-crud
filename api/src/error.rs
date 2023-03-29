use actix_web::{error, http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;
use log::error;
use serde::Serialize;

#[derive(Debug, Display)]
pub enum ServerError {
    InternalError,
    AuthExpired,
    AuthInvalid,
    UserNameUsed,
    UserNotFound,
    UserPasswordError,
    UserLogoutFailed,
    DbError
}

impl ResponseError for ServerError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(ErrorBody {
            code: self.to_string(),
            msg: self.msg(),
        })
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ServerError::AuthExpired | ServerError::AuthInvalid => StatusCode::UNAUTHORIZED,
            ServerError::UserNameUsed => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ServerError {
    fn msg(&self) -> String {
        match self {
            ServerError::AuthExpired => "Your authorization is expired, please login again.",
            ServerError::AuthInvalid => "Unauthorized, please login first.",
            _ => "",
        }
        .to_string()
    }
}

impl<T> From<ServerError> for Result<T, error::Error> {
    fn from(val: ServerError) -> Self {
        Err(error::Error::from(val))
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

#[derive(Serialize)]
struct ErrorBody {
    code: String,
    msg: String,
}
