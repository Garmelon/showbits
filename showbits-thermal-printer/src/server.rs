pub mod somehow;
mod r#static;
pub mod statuscode;

use std::path::PathBuf;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use showbits_typst::Typst;
use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
};

use crate::{documents, drawer::Command};

#[derive(Clone)]
pub struct Server {
    tx: mpsc::Sender<Command>,
    pub originals: Option<PathBuf>,
}

impl Server {
    pub async fn print_typst(&self, typst: Typst) -> somehow::Result<()> {
        let (tx, rx) = oneshot::channel();
        let _ = self.tx.send(Command::Typst(typst, tx)).await;
        rx.await?.map_err(somehow::Error)
    }
}

pub async fn run(
    tx: mpsc::Sender<Command>,
    addr: String,
    originals: Option<PathBuf>,
) -> anyhow::Result<()> {
    let app = Router::new()
        // Files
        .route("/", get(r#static::get_index))
        .route("/assets/{*path}", get(r#static::get_asset))
        .route("/fonts/{*path}", get(r#static::get_font))
        .route("/photo.html", get(r#static::get_photo))
        // API
        .route("/api/banner", post(documents::banner::post))
        .route("/api/calendar", post(documents::calendar::post))
        .route("/api/cells", post(documents::cells::post))
        .route("/api/chat", post(documents::chat::post))
        .route("/api/egg", post(documents::egg::post))
        .route(
            "/api/image",
            post(documents::image::post).get(documents::image::get),
        )
        .route("/api/sunrise", post(documents::sunrise::post))
        .route("/api/text", post(documents::text::post))
        .route("/api/tictactoe", post(documents::tictactoe::post))
        .route("/api/xkcd", post(documents::xkcd::post))
        // Rest
        .layer(DefaultBodyLimit::max(32 * 1024 * 1024)) // 32 MiB
        .with_state(Server { tx, originals });

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
