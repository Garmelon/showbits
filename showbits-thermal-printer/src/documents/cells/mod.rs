use std::{collections::VecDeque, io::Cursor};

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

fn generate_image(rows: u32, cols: u32, rule: u8) -> RgbaImage {
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

    image
}

fn read_row(image: &RgbaImage, y: u32) -> Vec<bool> {
    let mut result = Vec::with_capacity(image.width() as usize);
    for x in 0..image.width() {
        result.push(c2b(*image.get_pixel(x, y)));
    }
    result
}

fn is_interesting(image: &RgbaImage) -> bool {
    let mut last_rows = VecDeque::new();
    for y in 0..image.height() {
        let row = read_row(image, y);
        if last_rows.contains(&row) {
            return false;
        }
        last_rows.push_back(row);
        while last_rows.len() > 5 {
            last_rows.pop_front();
        }
    }
    true
}

fn generate_interesting_image(rows: u32, cols: u32) -> (u8, RgbaImage) {
    loop {
        let rule = rand::random();
        let image = generate_image(rows, cols, rule);
        if is_interesting(&image) {
            break (rule, image);
        }
        println!("Uninteresting automaton, generating a new one");
    }
}

#[derive(Serialize)]
struct Data {
    rule: Option<u8>,
    feed: bool,
}

#[derive(Deserialize)]
pub struct FormData {
    pub show_rule: Option<bool>,
    pub rule: Option<u8>,
    pub rows: Option<u32>,
    pub scale: Option<u32>,
    pub feed: Option<bool>,
}

pub async fn post(server: State<Server>, Form(form): Form<FormData>) -> somehow::Result<()> {
    let show_rule = form.show_rule.unwrap_or(true);
    let scale = form.scale.unwrap_or(4).clamp(1, 16);
    let rows = form.rows.unwrap_or(128 * 4 / scale).clamp(1, 1024 / scale);
    let cols = Printer::WIDTH / scale;

    let (rule, image) = match form.rule {
        Some(rule) => (rule, generate_image(rows, cols, rule)),
        None => generate_interesting_image(rows, cols),
    };

    let image = imageops::resize(
        &image,
        image.width() * scale,
        image.height() * scale,
        imageops::Nearest,
    );

    let data = Data {
        rule: Some(rule).filter(|_| show_rule),
        feed: form.feed.unwrap_or(true),
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

    server.print_typst(typst).await;
    Ok(())
}
