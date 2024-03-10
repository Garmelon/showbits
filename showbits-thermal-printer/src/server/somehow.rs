use std::{error, fmt, result};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::statuscode::status_code_with_info;

pub struct Error(pub anyhow::Error);

impl<E> From<E> for Error
where
    E: error::Error + Send + Sync + 'static,
{
    fn from(value: E) -> Self {
        Self(anyhow::Error::from(value))
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        status_code_with_info(StatusCode::INTERNAL_SERVER_ERROR, &self.0)
    }
}

pub type Result<T> = result::Result<T, Error>;
