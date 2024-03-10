use axum::{
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static"]
struct StaticFiles;

struct StaticFile(String);

fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

fn look_up_path(path: &str) -> Option<Response> {
    let path = path.trim_start_matches('/');
    let file = StaticFiles::get(path)?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    let response = ([(header::CONTENT_TYPE, mime.as_ref())], file.data).into_response();
    Some(response)
}

impl IntoResponse for StaticFile {
    fn into_response(self) -> Response {
        let mut path = self.0;
        if path.is_empty() {
            path.push('/')
        };

        if path.ends_with(".html") {
            // A file `/foo/bar.html` should not be accessible directly, only
            // indirectly at `/foo/bar`.
            return not_found();
        }

        if path.ends_with("/index") {
            // A file `/foo/index.html` should not be accessible directly, only
            // indirectly at `/foo/`.
            return not_found();
        }

        if path.ends_with('/') {
            path.push_str("index");
        }

        if let Some(response) = look_up_path(&path) {
            return response;
        }

        path.push_str(".html");

        if let Some(response) = look_up_path(&path) {
            return response;
        }

        not_found()
    }
}

pub async fn get_static_file(uri: Uri) -> impl IntoResponse {
    StaticFile(uri.path().to_string())
}
