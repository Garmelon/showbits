use axum::{
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static"]
struct StaticFiles;

struct StaticFile(pub String);

impl IntoResponse for StaticFile {
    fn into_response(self) -> Response {
        match StaticFiles::get(&self.0) {
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
            Some(file) => {
                let mime = mime_guess::from_path(self.0).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], file.data).into_response()
            }
        }
    }
}

pub async fn get_static_file(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();
    StaticFile(path)
}
