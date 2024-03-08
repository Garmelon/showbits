use palette::Srgba;

pub const TRANSPARENT: Srgba = Srgba::new(0.0, 0.0, 0.0, 0.0);
pub const BLACK: Srgba = Srgba::new(0.0, 0.0, 0.0, 1.0);
pub const WHITE: Srgba = Srgba::new(1.0, 1.0, 1.0, 1.0);

pub fn from_image_color(color: image::Rgba<u8>) -> Srgba {
    let [r, g, b, a] = color.0;
    Srgba::new(r, g, b, a).into_format()
}

pub fn to_image_color(color: Srgba) -> image::Rgba<u8> {
    let color = color.into_format::<u8, u8>();
    image::Rgba([color.red, color.green, color.blue, color.alpha])
}

pub fn from_text_color(color: cosmic_text::Color) -> Srgba {
    let [r, g, b, a] = color.as_rgba();
    Srgba::new(r, g, b, a).into_format()
}

pub fn to_text_color(color: Srgba) -> cosmic_text::Color {
    let color = color.into_format::<u8, u8>();
    cosmic_text::Color::rgba(color.red, color.green, color.blue, color.alpha)
}
