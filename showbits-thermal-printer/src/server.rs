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

use crate::drawer::{
    CalendarDrawing, CellsDrawing, ChatMessageDrawing, Command, EggDrawing, ImageDrawing,
    PhotoDrawing, TextDrawing, TicTacToeDrawing,
};

use self::{r#static::get_static_file, statuscode::status_code};

#[derive(Clone)]
struct Server {
    tx: mpsc::Sender<Command>,
}

pub async fn run(tx: mpsc::Sender<Command>, addr: String) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/calendar", post(post_calendar))
        .route("/cells", post(post_cells))
        .route("/chat_message", post(post_chat_message))
        .route("/egg", post(post_egg).fallback(get_static_file))
        .route("/image", post(post_image).fallback(get_static_file))
        .route("/photo", post(post_photo).fallback(get_static_file))
        .route("/text", post(post_text))
        .route("/tictactoe", post(post_tictactoe))
        .fallback(get(get_static_file))
        .layer(DefaultBodyLimit::max(32 * 1024 * 1024)) // 32 MiB
        .with_state(Server { tx });

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// /calendar

#[derive(Deserialize)]
struct PostCalendarForm {
    year: i32,
    month: u8,
}

async fn post_calendar(server: State<Server>, request: Form<PostCalendarForm>) {
    let _ = server
        .tx
        .send(Command::draw(CalendarDrawing {
            year: request.0.year,
            month: request.0.month,
        }))
        .await;
}

// /cells

#[derive(Deserialize)]
struct PostCellsForm {
    rule: u8,
    rows: Option<u32>,
    scale: Option<u32>,
}

async fn post_cells(server: State<Server>, request: Form<PostCellsForm>) {
    let _ = server
        .tx
        .send(Command::draw(CellsDrawing {
            rule: request.0.rule,
            rows: request.0.rows.unwrap_or(32).min(512),
            scale: request.0.scale.unwrap_or(4),
        }))
        .await;
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

// /egg

async fn post_egg(server: State<Server>) -> impl IntoResponse {
    let _ = server.tx.send(Command::draw(EggDrawing)).await;
    Redirect::to("egg")
}

// /image

async fn post_image(server: State<Server>, mut multipart: Multipart) -> somehow::Result<Response> {
    let mut image = None;
    let mut bright = false;

    while let Some(field) = multipart.next_field().await? {
        match field.name() {
            Some("image") => {
                let data = field.bytes().await?;
                let decoded = image::load_from_memory(&data)?.into_rgba8();
                image = Some(decoded);
            }
            Some("bright") => {
                bright = true;
            }
            _ => {}
        }
    }

    let Some(image) = image else {
        return Ok(status_code(StatusCode::UNPROCESSABLE_ENTITY));
    };

    let _ = server
        .tx
        .send(Command::draw(ImageDrawing { image, bright }))
        .await;
    Ok(Redirect::to("image").into_response())
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

// /text

#[derive(Deserialize)]
struct PostTextForm {
    text: String,
}

async fn post_text(server: State<Server>, request: Form<PostTextForm>) {
    let _ = server
        .tx
        .send(Command::draw(TextDrawing(request.0.text)))
        .await;
}

// /tictactoe

async fn post_tictactoe(server: State<Server>) {
    let _ = server.tx.send(Command::draw(TicTacToeDrawing)).await;
}
