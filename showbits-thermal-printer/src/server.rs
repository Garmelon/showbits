pub mod somehow;
mod r#static;
pub mod statuscode;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use tokio::{net::TcpListener, sync::mpsc};

use crate::{documents, drawer::Command};

use self::r#static::get_static_file;

#[derive(Clone)]
pub struct Server {
    pub tx: mpsc::Sender<Command>,
}

pub async fn run(tx: mpsc::Sender<Command>, addr: String) -> anyhow::Result<()> {
    let app = Router::new()
        .route(
            "/calendar",
            post(documents::calendar::post).fallback(get_static_file),
        )
        .route(
            "/cells",
            post(documents::cells::post).fallback(get_static_file),
        )
        .route(
            "/chat",
            post(documents::chat::post).fallback(get_static_file),
        )
        .route("/egg", post(documents::egg::post).fallback(get_static_file))
        .route(
            "/image",
            post(documents::image::post).fallback(get_static_file),
        )
        .route(
            "/text",
            post(documents::text::post).fallback(get_static_file),
        )
        .route(
            "/tictactoe",
            post(documents::tictactoe::post).fallback(get_static_file),
        )
        .fallback(get(get_static_file))
        .layer(DefaultBodyLimit::max(32 * 1024 * 1024)) // 32 MiB
        .with_state(Server { tx });

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
