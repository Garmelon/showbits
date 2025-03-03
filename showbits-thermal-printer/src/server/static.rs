use axum::{
    extract::Path,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;
use showbits_assets::{
    UNIFONT, UNIFONT_JP, UNIFONT_JP_NAME, UNIFONT_NAME, UNIFONT_UPPER, UNIFONT_UPPER_NAME,
};

use super::statuscode::status_code;

#[derive(RustEmbed)]
#[folder = "dist/assets"]
struct Assets;

pub async fn get_asset(Path(path): Path<String>) -> impl IntoResponse {
    match Assets::get(&path) {
        None => status_code(StatusCode::NOT_FOUND),
        Some(content) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
    }
}

pub async fn get_font(Path(path): Path<String>) -> Response {
    let font = if path == UNIFONT_NAME {
        UNIFONT
    } else if path == UNIFONT_JP_NAME {
        UNIFONT_JP
    } else if path == UNIFONT_UPPER_NAME {
        UNIFONT_UPPER
    } else {
        return status_code(StatusCode::NOT_FOUND);
    };

    ([(header::CONTENT_TYPE, "font/otf")], font).into_response()
}

pub async fn get_index() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        include_str!("../../dist/index.html"),
    )
}

pub async fn get_photo() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        include_str!("../../dist/photo.html"),
    )
}
