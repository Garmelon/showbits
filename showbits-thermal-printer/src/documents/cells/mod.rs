use std::io::Cursor;

use anyhow::Context;
use axum::{Form, extract::State};
use image::{ImageFormat, Rgba, RgbaImage, imageops};
use serde::{Deserialize, Serialize};

use crate::{
    printer::Printer,
    server::{Server, somehow},
};

const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);
const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);

fn b2c(bool: bool) -> Rgba<u8> {
    match bool {
        true => BLACK,
        false => WHITE,
    }
}

fn c2b(color: Rgba<u8>) -> bool {
    color == BLACK
}

fn neighbors_at(image: &RgbaImage, x: u32, y: u32) -> [bool; 3] {
    let left = x
        .checked_sub(1)
        .map(|x| *image.get_pixel(x, y))
        .unwrap_or(WHITE);

    let mid = *image.get_pixel(x, y);

    let right = image.get_pixel_checked(x + 1, y).copied().unwrap_or(WHITE);

    [c2b(left), c2b(mid), c2b(right)]
}

fn apply_rule(rule: u8, neighbors: [bool; 3]) -> bool {
    let [left, mid, right] = neighbors.map(|n| n as u8);
    let index = (left << 2) | (mid << 1) | right;
    rule & (1 << index) != 0
}

#[derive(Serialize)]
struct Data {
    feed: bool,
}

#[derive(Deserialize)]
pub struct FormData {
    pub rule: Option<u8>,
    pub rows: Option<u32>,
    pub scale: Option<u32>,
    pub feed: Option<bool>,
}

pub async fn post(server: State<Server>, Form(form): Form<FormData>) -> somehow::Result<()> {
    let data = Data {
        feed: form.feed.unwrap_or(true),
    };

    let rule = form.rule.unwrap_or_else(rand::random);
    let scale = form.scale.unwrap_or(4).clamp(1, 16);
    let rows = form.rows.unwrap_or(128 * 4 / scale).clamp(1, 1024 / scale);
    let cols = Printer::WIDTH / scale;

    let mut image: image::ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(cols, rows);

    // Initialize first line randomly
    for x in 0..image.width() {
        image.put_pixel(x, 0, b2c(rand::random()));
    }

    // Calculate next rows
    for y in 1..image.height() {
        for x in 0..image.width() {
            let neighbors = neighbors_at(&image, x, y - 1);
            let state = apply_rule(rule, neighbors);
            image.put_pixel(x, y, b2c(state));
        }
    }

    let image = imageops::resize(
        &image,
        image.width() * scale,
        image.height() * scale,
        imageops::Nearest,
    );

    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .context("failed to encode image as png")
        .map_err(somehow::Error)?;

    let typst = super::typst_with_lib()
        .with_json("/data.json", &data)
        .with_file("/image.png", bytes)
        .with_main_file(include_str!("main.typ"));

    server.print_typst(typst).await;
    Ok(())
}
