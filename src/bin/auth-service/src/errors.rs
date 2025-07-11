use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use diesel_async::pooled_connection::PoolError;
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error(transparent)]
    PoolError(#[from] PoolError),

    #[error(transparent)]
    DatabaseError(#[from] diesel_async::pooled_connection::deadpool::PoolError),

    #[error(transparent)]
    BuildError(#[from] diesel_async::pooled_connection::deadpool::BuildError),

    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),

    #[error(transparent)]
    SetLoggerError(#[from] log::SetLoggerError),

    #[error(transparent)]
    ParseLevelError(#[from] log::ParseLevelError),

    #[error(transparent)]
    MailboxError(#[from] actix::MailboxError),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error("Cannot get token from Auth0")]
    InvalidToken,

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    ToStrError(#[from] actix_web::http::header::ToStrError),

    #[error(transparent)]
    DotenvError(#[from] dotenv::Error),

    #[error(transparent)]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    ErrorStackError(#[from] openssl::error::ErrorStack),

    #[error(transparent)]
    AlcoholicJwtValidationError(#[from] alcoholic_jwt::ValidationError),

    #[error("Error: {0}")]
    StringError(String),

    #[error(transparent)]
    ClapError(#[from] clap::Error),

    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),

    #[error(transparent)]
    Auth0RequestBuildError(#[from] crate::services::auth0::errors::BuildError),

    #[error(transparent)]
    UuidError(#[from] uuid::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessageResponse {
    pub message: String,
}

impl ErrorMessageResponse {
    pub fn response_from(status: StatusCode, err: &Error) -> HttpResponse {
        HttpResponse::build(status).json(Self {
            message: err.to_string(),
        })
    }

    pub fn response_with_message(status: StatusCode, message: String) -> HttpResponse {
        HttpResponse::build(status).json(Self { message })
    }
}

impl actix_web::ResponseError for Error {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        log::warn!("{:?}", self);
        match self {
            Error::Io(_) => ErrorMessageResponse::response_from(StatusCode::BAD_REQUEST, self),
            Error::Parse(_) => ErrorMessageResponse::response_from(StatusCode::BAD_REQUEST, self),
            Error::InvalidInput(_) => {
                ErrorMessageResponse::response_from(StatusCode::BAD_REQUEST, self)
            }
            Error::PoolError(_) => {
                ErrorMessageResponse::response_from(StatusCode::INTERNAL_SERVER_ERROR, self)
            }
            Error::DatabaseError(_) => {
                ErrorMessageResponse::response_from(StatusCode::INTERNAL_SERVER_ERROR, self)
            }
            Error::DieselError(_) => {
                ErrorMessageResponse::response_from(StatusCode::INTERNAL_SERVER_ERROR, self)
            }
            Error::SerdeJsonError(_) => ErrorMessageResponse::response_with_message(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Incorrect data, please check your request".to_string(),
            ),
            _ => ErrorMessageResponse::response_from(StatusCode::INTERNAL_SERVER_ERROR, self),
        }
    }
}
