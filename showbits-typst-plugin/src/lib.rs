use std::io::Cursor;

use image::ImageFormat;
use mark::dither::{AlgoFloydSteinberg, Algorithm, DiffEuclid, Palette};
use palette::LinSrgb;
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

initiate_protocol!();

#[wasm_func]
pub fn dither(image: &[u8]) -> Result<Vec<u8>, String> {
    let image = image::load_from_memory(image)
        .map_err(|it| format!("Failed to read image: {it:?}"))?
        .to_rgba8();

    let palette = Palette::new(vec![
        LinSrgb::new(0.0, 0.0, 0.0),
        LinSrgb::new(1.0, 1.0, 1.0),
    ]);

    let dithered = <AlgoFloydSteinberg as Algorithm<LinSrgb, DiffEuclid>>::run(image, &palette);

    let mut bytes: Vec<u8> = Vec::new();
    dithered
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .map_err(|it| format!("Failed to write image: {it:?}"))?;

    Ok(bytes)
}
