use hyper::http::StatusCode;
use serde::Serialize;
use std::{
    fmt::Display,
    io::{self, ErrorKind},
};

#[derive(Debug, Clone)]
pub struct Error {
    pub status_code: StatusCode,
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    status_code: u16,
    message: String,
}

impl From<Error> for ErrorResponse {
    fn from(err: Error) -> Self {
        Self {
            status_code: err.status_code.as_u16(),
            message: err.message,
        }
    }
}

impl From<Box<Error>> for ErrorResponse {
    fn from(err: Box<Error>) -> Self {
        Self {
            status_code: err.status_code.as_u16(),
            message: err.message,
        }
    }
}

impl Error {
    pub fn new<T: Into<String>>(message: T, status_code: StatusCode) -> Self {
        Self {
            status_code,
            message: message.into(),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            ErrorKind::OutOfMemory => Self::new(
                "Server is out of memory :(",
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            _ => Self::new(
                "Could not finish the request due to an error!",
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Self::new(
                "Could not find the requested resource!",
                StatusCode::NOT_FOUND,
            ),
            _ => Self::new(
                "Could not finish the request due to an error!",
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        match error.into_kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                Self::new("The token has expired!", StatusCode::UNAUTHORIZED)
            }
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                Self::new("The token is invalid!", StatusCode::UNAUTHORIZED)
            }
            _ => Self::new(
                "Could not finish the request due to an error!",
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} occurred with status code: {}",
            self.message, self.status_code
        )
    }
}
