use std::io::Cursor;

use anyhow::Context;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use image::ImageFormat;
use serde::Serialize;

use crate::server::{Server, somehow, statuscode::status_code};

#[derive(Serialize)]
struct Data {
    title: Option<String>,
    caption: Option<String>,
    algo: String,
    bright: bool,
    seamless: bool,
    feed: bool,
}

pub async fn post(server: State<Server>, mut multipart: Multipart) -> somehow::Result<Response> {
    let mut image = None;
    let mut data = Data {
        title: None,
        caption: None,
        algo: "stucki".to_string(),
        bright: true,
        seamless: false,
        feed: true,
    };

    while let Some(field) = multipart.next_field().await? {
        match field.name() {
            Some("image") => {
                let data = field.bytes().await?;
                let decoded = image::load_from_memory(&data)?.into_rgba8();
                image = Some(decoded);
            }
            Some("title") => {
                data.title = Some(field.text().await?);
            }
            Some("caption") => {
                data.caption = Some(field.text().await?);
            }
            Some("algo") => {
                data.algo = field.text().await?;
            }
            Some("bright") => {
                data.bright = !field.text().await?.is_empty();
            }
            Some("seamless") => {
                data.seamless = !field.text().await?.is_empty();
            }
            Some("feed") => {
                data.feed = !field.text().await?.is_empty();
            }
            _ => {}
        }
    }

    let Some(image) = image else {
        return Ok(status_code(StatusCode::UNPROCESSABLE_ENTITY));
    };

    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .context("failed to encode image as png")
        .map_err(somehow::Error)?;

    let typst = super::typst_with_lib()
        .with_json("/data.json", &data)
        .with_file("/image.png", bytes)
        .with_main_file(include_str!("main.typ"));

    server.print_typst(typst).await?;
    Ok(().into_response())
}
