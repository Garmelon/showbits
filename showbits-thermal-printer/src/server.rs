pub mod somehow;
mod r#static;
pub mod statuscode;

use axum::{
    Form, Router,
    extract::{DefaultBodyLimit, Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use serde::Deserialize;
use tokio::{net::TcpListener, sync::mpsc};

use crate::{
    documents,
    drawer::{ChatMessageDrawing, Command, PhotoDrawing, TicTacToeDrawing, TypstDrawing},
};

use self::{r#static::get_static_file, statuscode::status_code};

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
        .route("/photo", post(post_photo).fallback(get_static_file))
        .route(
            "/text",
            post(documents::text::post).fallback(get_static_file),
        )
        .route("/tictactoe", post(post_tictactoe))
        .route("/typst", post(post_typst).fallback(get_static_file))
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

// /photo

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

    let _ = server
        .tx
        .send(Command::draw(PhotoDrawing { image, title }))
        .await;
    Ok(Redirect::to("photo").into_response())
}

// /tictactoe

async fn post_tictactoe(server: State<Server>) {
    let _ = server.tx.send(Command::draw(TicTacToeDrawing)).await;
}

// /typst

#[derive(Deserialize)]
struct PostTypstForm {
    source: String,
}

async fn post_typst(server: State<Server>, request: Form<PostTypstForm>) {
    let _ = server
        .tx
        .send(Command::draw(TypstDrawing(request.0.source)))
        .await;
}
