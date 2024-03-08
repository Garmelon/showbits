use axum::{extract::State, routing::post, Form, Router};
use serde::Deserialize;
use tokio::{net::TcpListener, sync::mpsc};

use crate::drawer::Command;

#[derive(Clone)]
struct Server {
    tx: mpsc::Sender<Command>,
}

pub async fn run(tx: mpsc::Sender<Command>, addr: String) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/stop", post(post_stop))
        .route("/test", post(post_test))
        .route("/rip", post(post_rip))
        .route("/text", post(post_text))
        .route("/chat_message", post(post_chat_message))
        .with_state(Server { tx });

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn post_stop(server: State<Server>) {
    let _ = server.tx.send(Command::Stop).await;
}

async fn post_test(server: State<Server>) {
    let _ = server.tx.send(Command::Test).await;
}

async fn post_rip(server: State<Server>) {
    let _ = server.tx.send(Command::Rip).await;
}

#[derive(Deserialize)]
struct PostTextForm {
    text: String,
}

async fn post_text(server: State<Server>, request: Form<PostTextForm>) {
    let _ = server.tx.send(Command::Text(request.0.text)).await;
}

#[derive(Deserialize)]
struct PostChatMessageForm {
    username: String,
    content: String,
}

async fn post_chat_message(server: State<Server>, request: Form<PostChatMessageForm>) {
    let _ = server
        .tx
        .send(Command::ChatMessage {
            username: request.0.username,
            content: request.0.content,
        })
        .await;
}
