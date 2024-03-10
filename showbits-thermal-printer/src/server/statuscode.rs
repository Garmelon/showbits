use core::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub fn status_code(code: StatusCode) -> Response {
    (code, code.to_string()).into_response()
}

pub fn status_code_with_info<I: fmt::Display>(code: StatusCode, info: &I) -> Response {
    let message = format!("{code}\n\n{info}");
    (code, message).into_response()
}
