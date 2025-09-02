use std::io::Cursor;

use anyhow::Context;
use axum::{
    Form,
    extract::State,
    response::{IntoResponse, Response},
};
use image::{ImageFormat, imageops};
use serde::{Deserialize, Serialize};

use crate::server::{Server, somehow};

#[derive(Deserialize)]
struct ComicInfo {
    num: u32,
    title: String,
    alt: String,
    img: String,
}

#[derive(Serialize)]
struct Data {
    number: u32,
    title: String,
    alt: String,
    dither: bool,
    feed: bool,
}

#[derive(Deserialize)]
pub struct FormData {
    pub number: Option<u32>,
    pub rotate: Option<bool>,
    pub dither: Option<bool>,
    pub bright: Option<bool>,
    pub feed: Option<bool>,
}

pub async fn post(server: State<Server>, Form(form): Form<FormData>) -> somehow::Result<Response> {
    let client = reqwest::Client::builder()
        .user_agent(crate::USER_AGENT)
        .build()?;

    let url = match form.number {
        None => "https://xkcd.com/info.0.json".to_string(),
        Some(number) => format!("https://xkcd.com/{number}/info.0.json"),
    };

    let info = client.get(url).send().await?.json::<ComicInfo>().await?;

    let image_data = client.get(&info.img).send().await?.bytes().await?;
    let mut image = image::load_from_memory(&image_data)?.into_rgba8();

    let data = Data {
        number: info.num,
        title: info.title,
        alt: info.alt,
        dither: form.dither.unwrap_or(true),
        feed: form.feed.unwrap_or(true),
    };

    if form.rotate.unwrap_or(false) {
        image = imageops::rotate90(&image);
    }

    if data.dither {
        let max_width = Some(384);
        let max_height = Some(1024);
        image = super::image::dither(
            image,
            max_width,
            max_height,
            form.bright.unwrap_or(true),
            "stucki",
        )
        .map_err(somehow::Error)?;
    }

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
