#![allow(unused)]

use palette::Srgba;

pub fn image_to_palette(color: image::Rgba<u8>) -> Srgba {
    let [r, g, b, a] = color.0;
    Srgba::new(r, g, b, a).into_format()
}

pub fn palette_to_image(color: Srgba) -> image::Rgba<u8> {
    let color = color.into_format::<u8, u8>();
    image::Rgba([color.red, color.green, color.blue, color.alpha])
}
