use palette::Srgb;
use showbits_common::Color;

#[derive(Clone, Copy)]
pub enum PixelBw {
    Black,
    White,
    Transparent,
    Invert,
}

impl Color for PixelBw {
    fn to_srgb(self) -> Srgb {
        match self {
            Self::Black => Srgb::new(0.0, 0.0, 0.0),
            Self::White => Srgb::new(1.0, 1.0, 1.0),
            _ => Srgb::new(1.0, 0.0, 1.0),
        }
    }

    fn over(self, other: Self) -> Self {
        match (self, other) {
            (Self::Black, _) => Self::Black,
            (Self::White, _) => Self::White,
            (Self::Transparent, p) => p,
            (Self::Invert, Self::Black) => Self::White,
            (Self::Invert, Self::White) => Self::Black,
            (Self::Invert, Self::Invert) => Self::Transparent,
            (Self::Invert, Self::Transparent) => Self::Invert,
        }
    }
}
