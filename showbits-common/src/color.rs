use palette::Srgb;

pub trait Color: Copy {
    /// Convert to an sRGB color.
    ///
    /// Useful for debugging or dithering.
    fn to_srgb(self) -> Srgb;

    /// Combine two colors by putting one "over" the other.
    fn over(self, below: Self) -> Self;
}
