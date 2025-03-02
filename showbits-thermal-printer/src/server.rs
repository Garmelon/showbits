pub mod somehow;
mod r#static;
pub mod statuscode;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use showbits_typst::Typst;
use tokio::{net::TcpListener, sync::mpsc};

use crate::{documents, drawer::Command};

use self::r#static::get_static_file;

#[derive(Clone)]
pub struct Server {
    tx: mpsc::Sender<Command>,
}

impl Server {
    pub async fn print_typst(&self, typst: Typst) {
        let _ = self.tx.send(Command::Typst(typst)).await;
    }
}

pub async fn run(tx: mpsc::Sender<Command>, addr: String) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/api/calendar", post(documents::calendar::post))
        .route("/api/cells", post(documents::cells::post))
        .route("/api/chat", post(documents::chat::post))
        .route("/api/egg", post(documents::egg::post))
        .route("/api/image", post(documents::image::post))
        .route("/api/text", post(documents::text::post))
        .route("/api/tictactoe", post(documents::tictactoe::post))
        .fallback(get(get_static_file))
        .layer(DefaultBodyLimit::max(32 * 1024 * 1024)) // 32 MiB
        .with_state(Server { tx });

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
