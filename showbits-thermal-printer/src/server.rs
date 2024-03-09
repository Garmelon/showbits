use axum::{
    extract::{DefaultBodyLimit, Multipart, State},
    http::StatusCode,
    routing::post,
    Form, Router,
};
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
        .route("/image", post(post_image))
        .route("/chat_message", post(post_chat_message))
        .layer(DefaultBodyLimit::max(32 * 1024 * 1024)) // 32 MiB
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

async fn post_image(server: State<Server>, mut multipart: Multipart) -> Result<(), StatusCode> {
    let mut image = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let name = field.name().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        if name == "image" {
            let data = field
                .bytes()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let decoded = image::load_from_memory(&data)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .into_rgba8();

            image = Some(decoded);
        }
    }

    if let Some(image) = image {
        let _ = server.tx.send(Command::Image(image)).await;
        Ok(())
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
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
