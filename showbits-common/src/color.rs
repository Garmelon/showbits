use palette::Srgb;

pub fn from_image_rgb(color: image::Rgb<u8>) -> Srgb {
    let [r, g, b] = color.0;
    Srgb::new(r, g, b).into_format()
}

pub fn to_image_rgb(color: Srgb) -> image::Rgb<u8> {
    let color = color.into_format::<u8>();
    image::Rgb([color.red, color.green, color.blue])
}

pub fn from_text_color(color: cosmic_text::Color) -> Srgb {
    let [r, g, b, _] = color.as_rgba();
    Srgb::new(r, g, b).into_format()
}

pub fn to_text_color(color: Srgb) -> cosmic_text::Color {
    let color = color.into_format::<u8>();
    cosmic_text::Color::rgb(color.red, color.green, color.blue)
}
