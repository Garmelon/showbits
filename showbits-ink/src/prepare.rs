use image::{Rgb, RgbImage, Rgba, RgbaImage};
use palette::{LinSrgba, Srgba, blend::Compose};

use crate::display;

fn rgb_to_rgba(color: Rgb<u8>) -> Rgba<u8> {
    let [r, g, b] = color.0;
    Rgba([r, g, b, 255])
}

fn rgba_to_rgb(color: Rgba<u8>) -> Rgb<u8> {
    let [r, g, b, _] = color.0;
    Rgb([r, g, b])
}

fn rgba_to_lin(color: Rgba<u8>) -> LinSrgba {
    let [r, g, b, a] = color.0;
    Srgba::new(r, g, b, a).into_linear()
}

fn lin_to_rgba(color: LinSrgba) -> Rgba<u8> {
    let (r, g, b, a) = Srgba::from_linear(color).into_components();
    Rgba([r, g, b, a])
}

pub fn prepare(image: RgbaImage) -> RgbImage {
    let bg = rgba_to_lin(rgb_to_rgba(display::WHITE));

    let mut result = RgbImage::new(display::WIDTH, display::HEIGHT);
    for (x, y, pixel) in result.enumerate_pixels_mut() {
        let color = image
            .get_pixel_checked(x, y)
            .copied()
            .map(rgba_to_lin)
            .unwrap_or(bg);

        // Alpha blending over background color
        let color = color.over(bg);

        *pixel = rgba_to_rgb(lin_to_rgba(color));
    }
    result
}
