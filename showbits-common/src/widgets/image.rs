use image::{
    RgbaImage,
    imageops::{self, FilterType},
};
use mark::dither::{AlgoFloydSteinberg, AlgoStucki, Algorithm, DiffEuclid, Palette};
use palette::{IntoColor, LinSrgb, Srgba};
use taffy::prelude::{AvailableSpace, Layout, Size};

use crate::Widget;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DitherAlgorithm {
    FloydSteinberg,
    Stucki,
}

impl DitherAlgorithm {
    fn dither(self, image: RgbaImage, palette: &Palette<LinSrgb>) -> RgbaImage {
        match self {
            Self::FloydSteinberg => {
                <AlgoFloydSteinberg as Algorithm<LinSrgb, DiffEuclid>>::run(image, palette)
            }
            Self::Stucki => <AlgoStucki as Algorithm<LinSrgb, DiffEuclid>>::run(image, palette),
        }
    }
}

pub struct Image {
    image: RgbaImage,
    shrink: bool,
    grow: bool,
    filter: FilterType,

    dither_palette: Option<Palette<LinSrgb>>,
    dither_algorithm: DitherAlgorithm,
}

impl Image {
    pub fn new(image: RgbaImage) -> Self {
        Self {
            image,
            shrink: true,
            grow: false,
            filter: FilterType::CatmullRom,
            dither_palette: None,
            dither_algorithm: DitherAlgorithm::FloydSteinberg,
        }
    }

    pub fn with_shrink(mut self, shrink: bool) -> Self {
        self.shrink = shrink;
        self
    }

    pub fn with_grow(mut self, grow: bool) -> Self {
        self.grow = grow;
        self
    }

    pub fn with_filter(mut self, filter: FilterType) -> Self {
        self.filter = filter;
        self
    }

    pub fn with_dither_palette(mut self, palette: &[Srgba]) -> Self {
        let palette = palette
            .iter()
            .map(|c| c.color.into_color())
            .collect::<Vec<LinSrgb>>();

        self.dither_palette = Some(Palette::new(palette));
        self
    }

    pub fn with_dither_algorithm(mut self, algorithm: DitherAlgorithm) -> Self {
        self.dither_algorithm = algorithm;
        self
    }
}

impl<C> Widget<C> for Image {
    fn size(
        &mut self,
        _ctx: &mut C,
        known: Size<Option<f32>>,
        available: Size<AvailableSpace>,
    ) -> Size<f32> {
        if self.image.width() == 0 || self.image.height() == 0 {
            // We don't want to divide by zero later on
            return Size {
                width: 0.0,
                height: 0.0,
            };
        }

        let size = Size {
            width: self.image.width() as f32,
            height: self.image.height() as f32,
        };

        let max_width = known.width.or(match available.width {
            AvailableSpace::Definite(width) => Some(width),
            AvailableSpace::MinContent => Some(0.0),
            AvailableSpace::MaxContent => None,
        });

        let max_height = known.height.or(match available.height {
            AvailableSpace::Definite(height) => Some(height),
            AvailableSpace::MinContent => Some(0.0),
            AvailableSpace::MaxContent => None,
        });

        let scale_factor = match (max_width, max_height) {
            (None, None) => 1.0,
            (None, Some(height)) => height / size.height,
            (Some(width), None) => width / size.width,
            (Some(width), Some(height)) => (width / size.width).min(height / size.height),
        };

        if (scale_factor < 1.0 && self.shrink) || (scale_factor > 1.0 && self.grow) {
            Size {
                width: size.width * scale_factor,
                height: size.height * scale_factor,
            }
        } else {
            size
        }
    }

    fn draw_below(
        &mut self,
        _ctx: &mut C,
        view: &mut crate::View<'_>,
        _layout: &Layout,
    ) -> anyhow::Result<()> {
        let (width, height) = view.size().to_u32();
        let image = imageops::resize(&self.image, width, height, self.filter);

        let image = if let Some(palette) = &self.dither_palette {
            self.dither_algorithm.dither(image, palette)
        } else {
            image
        };

        view.image(&image);
        Ok(())
    }
}
