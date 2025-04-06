use std::{fs, io::Cursor};

use anyhow::{Context, anyhow, bail};
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use image::{
    DynamicImage, EncodableLayout, ImageDecoder, ImageFormat, ImageReader, Luma, Pixel, RgbaImage,
    imageops,
};
use jiff::Timestamp;
use mark::dither::{AlgoFloydSteinberg, AlgoStucki, Algorithm, DiffEuclid, Palette};
use palette::LinSrgb;
use serde::Serialize;

use crate::server::{Server, somehow, statuscode::status_code};

pub fn dither(
    mut image: RgbaImage,
    max_width: Option<u32>,
    max_height: Option<u32>,
    bright: bool,
    algorithm: &str,
) -> anyhow::Result<RgbaImage> {
    let image_width = image.width();
    let image_height = image.height();

    let scale_factor = match (max_width, max_height) {
        (None, None) => 1.0,
        (None, Some(height)) => height as f32 / image_height as f32,
        (Some(width), None) => width as f32 / image_width as f32,
        (Some(width), Some(height)) => {
            (width as f32 / image_width as f32).min(height as f32 / image_height as f32)
        }
    };

    let target_width = (image_width as f32 * scale_factor) as u32;
    let target_height = (image_height as f32 * scale_factor) as u32;

    if image_width != target_width || image_height != target_height {
        image = imageops::resize(&image, target_width, target_height, imageops::CatmullRom);
    }

    if bright {
        for pixel in image.pixels_mut() {
            let [l] = pixel.to_luma().0;
            let l = l as f32 / 255.0; // Convert to [0, 1]
            let l = 1.0 - (0.4 * (1.0 - l)); // Lerp to [0.6, 1]
            let l = (l.clamp(0.0, 1.0) * 255.0) as u8; // Convert back to [0, 255]
            *pixel = Luma([l]).to_rgba();
        }
    }

    let palette = Palette::new(vec![
        LinSrgb::new(0.0, 0.0, 0.0),
        LinSrgb::new(1.0, 1.0, 1.0),
    ]);

    let dithered = match algorithm {
        "floyd-steinberg" => {
            <AlgoFloydSteinberg as Algorithm<LinSrgb, DiffEuclid>>::run(image, &palette)
        }
        "stucki" => <AlgoStucki as Algorithm<LinSrgb, DiffEuclid>>::run(image, &palette),
        it => bail!("Unknown dithering algorithm: {it}"),
    };

    Ok(dithered)
}

fn bool_from_str(s: &str) -> somehow::Result<bool> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(somehow::Error(anyhow!(
            "invalid boolean value {s:?}, must be true or false"
        ))),
    }
}

#[derive(Serialize)]
struct Data {
    title: Option<String>,
    caption: Option<String>,
    seamless: bool,
    feed: bool,
}

pub async fn post(server: State<Server>, mut multipart: Multipart) -> somehow::Result<Response> {
    let mut image = None;
    let mut algo = "stucki".to_string();
    let mut bright = true;

    let mut data = Data {
        title: None,
        caption: None,
        seamless: false,
        feed: true,
    };

    while let Some(field) = multipart.next_field().await? {
        match field.name() {
            Some("image") => {
                image = Some(field.bytes().await?);
            }
            Some("title") => {
                data.title = Some(field.text().await?).filter(|it| !it.is_empty());
            }
            Some("caption") => {
                data.caption = Some(field.text().await?).filter(|it| !it.is_empty());
            }
            Some("algo") => {
                algo = field.text().await?;
            }
            Some("bright") => {
                bright = !field.text().await?.is_empty();
            }
            Some("seamless") => {
                data.seamless = !field.text().await?.is_empty();
            }
            Some("feed") => {
                data.feed = bool_from_str(&field.text().await?)?;
            }
            _ => {}
        }
    }

    let Some(image) = image else {
        return Ok(status_code(StatusCode::UNPROCESSABLE_ENTITY));
    };

    // Export original image if requested
    if let Some(dir) = &server.originals {
        fs::create_dir_all(dir)?;
        let path = dir.join(Timestamp::now().as_millisecond().to_string());
        fs::write(path, &image)?;
    }

    // Decode image data
    let image = {
        // https://github.com/image-rs/image/issues/2392#issuecomment-2547393362
        let mut decoder = ImageReader::new(Cursor::new(image.as_bytes()))
            .with_guessed_format()?
            .into_decoder()?;
        let orientation = decoder.orientation()?;
        let mut decoded = DynamicImage::from_decoder(decoder)?;
        decoded.apply_orientation(orientation);
        decoded.to_rgba8()
    };

    // Dither image
    let max_width = Some(384);
    let max_height = Some(1024);
    let image = dither(image, max_width, max_height, bright, &algo).map_err(somehow::Error)?;

    // Encode dithered image for typst
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
