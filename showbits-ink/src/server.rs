use std::fmt::Display;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use tokio::net::TcpListener;

use crate::{prepare, queue::Queue};

fn status_code_with_info(code: StatusCode, info: impl Display) -> Response {
    let message = format!("{code}\n\n{info}");
    (code, message).into_response()
}

fn server_error(error: impl Display) -> Response {
    status_code_with_info(StatusCode::INTERNAL_SERVER_ERROR, error)
}

fn user_error(error: impl Display) -> Response {
    status_code_with_info(StatusCode::BAD_REQUEST, error)
}

#[derive(Clone)]
struct Server {
    queue: Queue,
}

#[expect(clippy::result_large_err)]
async fn post_image(server: State<Server>, mut multipart: Multipart) -> Result<Response, Response> {
    let mut image = None;

    while let Some(field) = multipart.next_field().await.map_err(user_error)? {
        match field.name() {
            Some("image") => {
                image = Some(field.bytes().await.map_err(user_error)?);
            }
            Some(name) => return Err(user_error(format!("unexpected field: {name}"))),
            None => return Err(user_error("unexpected unnamed field")),
        }
    }

    let Some(image) = image else {
        return Err(user_error("no field named \"image\""));
    };

    let image = ::image::load_from_memory(&image).map_err(user_error)?;
    let image = prepare::prepare(image.to_rgba8());
    server.queue.enqueue(&image).map_err(server_error)?;

    Ok(status_code_with_info(StatusCode::OK, "image queued"))
}

pub async fn run(address: String, queue: Queue) -> anyhow::Result<()> {
    let app = Router::new()
        // Files
        .route("/", post(post_image))
        // Rest
        .layer(DefaultBodyLimit::max(32 * 1024 * 1024)) // 32 MiB
        .with_state(Server { queue });

    let listener = TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
