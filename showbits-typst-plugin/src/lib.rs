use std::io::Cursor;

use image::{
    ImageFormat,
    imageops::{self, FilterType},
};
use mark::dither::{AlgoFloydSteinberg, AlgoStucki, Algorithm, DiffEuclid, Palette};
use palette::{FromColor, IntoColor, LinLumaa, LinSrgb, Srgba};
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

initiate_protocol!();

// Palette <-> image color conversions

fn image_to_palette(color: image::Rgba<u8>) -> Srgba {
    let [r, g, b, a] = color.0;
    Srgba::new(r, g, b, a).into_format()
}

fn palette_to_image(color: Srgba) -> image::Rgba<u8> {
    let color = color.into_format::<u8, u8>();
    image::Rgba([color.red, color.green, color.blue, color.alpha])
}

// Typst type conversions

fn i64_from_bytes(bytes: &[u8]) -> Result<i64, String> {
    let bytes: [u8; 8] = bytes.try_into().map_err(|it| format!("{it}"))?;
    Ok(i64::from_le_bytes(bytes))
}

fn bool_from_bytes(bytes: &[u8]) -> Result<bool, String> {
    Ok(i64_from_bytes(bytes)? != 0)
}

fn str_from_bytes(bytes: &[u8]) -> Result<&str, String> {
    std::str::from_utf8(bytes).map_err(|it| format!("{it}"))
}

fn size_from_bytes(bytes: &[u8]) -> Result<Option<u32>, String> {
    let size = i64_from_bytes(bytes)?;

    if size < 0 {
        return Ok(None); // Unlimited width
    }

    let size: u32 = size.try_into().map_err(|_| "size too large")?;
    Ok(Some(size))
}

// Typst methods

#[wasm_func]
pub fn dither(
    image: &[u8],
    max_width: &[u8],
    max_height: &[u8],
    bright: &[u8],
    algorithm: &[u8],
) -> Result<Vec<u8>, String> {
    let max_width = size_from_bytes(max_width)?;
    let max_height = size_from_bytes(max_height)?;
    let bright = bool_from_bytes(bright)?;
    let algorithm = str_from_bytes(algorithm)?;

    let mut image = image::load_from_memory(image)
        .map_err(|it| format!("Failed to read image: {it:?}"))?
        .to_rgba8();

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
        image = imageops::resize(&image, target_width, target_height, FilterType::CatmullRom);
    }

    if bright {
        for pixel in image.pixels_mut() {
            let mut color = LinLumaa::from_color(image_to_palette(*pixel));
            color.luma = 1.0 - 0.4 * (1.0 - color.luma);
            *pixel = palette_to_image(color.into_color());
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
        it => Err(format!("Unknown algorithm: {it}"))?,
    };

    let mut bytes: Vec<u8> = Vec::new();
    dithered
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .map_err(|it| format!("Failed to write image: {it:?}"))?;

    Ok(bytes)
}
