use image::Rgb;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn from_rgb(rgb: Rgb<u8>) -> Self {
        let [r, g, b] = rgb.0;
        if r <= 127 || g <= 127 || b <= 127 {
            Self::Black
        } else {
            Self::White
        }
    }

    pub fn into_rgb(self) -> Rgb<u8> {
        match self {
            Self::White => Rgb([255, 255, 255]),
            Self::Black => Rgb([0, 0, 0]),
        }
    }
}
