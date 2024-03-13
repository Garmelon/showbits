mod somehow;
mod r#static;
mod statuscode;

use axum::{
    extract::{DefaultBodyLimit, Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use tokio::{net::TcpListener, sync::mpsc};

use crate::drawer::Command;

use self::{r#static::get_static_file, statuscode::status_code};

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
        .route("/image", post(post_image).fallback(get_static_file))
        .route("/photo", post(post_photo).fallback(get_static_file))
        .route("/chat_message", post(post_chat_message))
        .route("/calendar", post(post_calendar))
        .fallback(get(get_static_file))
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

async fn post_image(server: State<Server>, mut multipart: Multipart) -> somehow::Result<Response> {
    let mut image = None;

    while let Some(field) = multipart.next_field().await? {
        if let Some("image") = field.name() {
            let data = field.bytes().await?;
            let decoded = image::load_from_memory(&data)?.into_rgba8();
            image = Some(decoded);
        }
    }

    let Some(image) = image else {
        return Ok(status_code(StatusCode::UNPROCESSABLE_ENTITY));
    };

    let _ = server.tx.send(Command::Image(image)).await;
    Ok(Redirect::to("image").into_response())
}

async fn post_photo(server: State<Server>, mut multipart: Multipart) -> somehow::Result<Response> {
    let mut image = None;
    let mut title = None;

    while let Some(field) = multipart.next_field().await? {
        match field.name() {
            Some("image") => {
                let data = field.bytes().await?;
                let decoded = image::load_from_memory(&data)?.into_rgba8();
                image = Some(decoded);
            }
            Some("title") => {
                title = Some(field.text().await?);
            }
            _ => {}
        }
    }

    let Some(image) = image else {
        return Ok(status_code(StatusCode::UNPROCESSABLE_ENTITY));
    };

    let Some(title) = title else {
        return Ok(status_code(StatusCode::UNPROCESSABLE_ENTITY));
    };

    let _ = server.tx.send(Command::Photo { image, title }).await;
    Ok(Redirect::to("photo").into_response())
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

#[derive(Deserialize)]
struct PostCalendarForm {
    year: i32,
    month: u8,
}

async fn post_calendar(server: State<Server>, request: Form<PostCalendarForm>) {
    let _ = server
        .tx
        .send(Command::Calendar {
            year: request.0.year,
            month: request.0.month,
        })
        .await;
}
