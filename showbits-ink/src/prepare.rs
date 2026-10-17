use anyhow::bail;
use image::RgbImage;

use crate::display;

pub fn prepare(mut image: RgbImage) -> anyhow::Result<RgbImage> {
    if image.width() != display::WIDTH || image.height() != display::HEIGHT {
        bail!("image must be {}x{}", display::WIDTH, display::HEIGHT);
    }

    for pixel in image.pixels_mut() {
        let [r, g, b] = pixel.0;
        if r > 127 && g <= 127 && b <= 127 {
            *pixel = display::RED;
        } else if r <= 127 && g <= 127 && b <= 127 {
            *pixel = display::BLACK;
        } else {
            *pixel = display::WHITE;
        }
    }

    Ok(image)
}
