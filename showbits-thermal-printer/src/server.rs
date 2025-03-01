pub mod somehow;
mod r#static;
pub mod statuscode;

use axum::{
    Form, Router,
    extract::{DefaultBodyLimit, State},
    routing::{get, post},
};
use serde::Deserialize;
use tokio::{net::TcpListener, sync::mpsc};

use crate::{
    documents,
    drawer::{ChatMessageDrawing, Command, TicTacToeDrawing},
};

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
        .route("/chat_message", post(post_chat_message))
        .route("/egg", post(documents::egg::post).fallback(get_static_file))
        .route(
            "/image",
            post(documents::image::post).fallback(get_static_file),
        )
        .route(
            "/text",
            post(documents::text::post).fallback(get_static_file),
        )
        .route("/tictactoe", post(post_tictactoe))
        .fallback(get(get_static_file))
        .layer(DefaultBodyLimit::max(32 * 1024 * 1024)) // 32 MiB
        .with_state(Server { tx });

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// /chat_message

#[derive(Deserialize)]
struct PostChatMessageForm {
    username: String,
    content: String,
}

async fn post_chat_message(server: State<Server>, request: Form<PostChatMessageForm>) {
    let _ = server
        .tx
        .send(Command::draw(ChatMessageDrawing {
            username: request.0.username,
            content: request.0.content,
        }))
        .await;
}

// /tictactoe

async fn post_tictactoe(server: State<Server>) {
    let _ = server.tx.send(Command::draw(TicTacToeDrawing)).await;
}
