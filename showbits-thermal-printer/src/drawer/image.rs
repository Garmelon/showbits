use image::RgbaImage;
use palette::{FromColor, IntoColor, LinLumaa};
use showbits_common::{
    Node, Tree, WidgetExt,
    color::{self, BLACK, WHITE},
    widgets::{DitherAlgorithm, Image},
};
use taffy::{AlignItems, Display, FlexDirection, prelude::length, style_helpers::percent};

use crate::persistent_printer::PersistentPrinter;

use super::{Context, Drawing, FEED};

pub struct ImageDrawing {
    pub image: RgbaImage,
    pub bright: bool,
    pub algo: DitherAlgorithm,
    pub scale: u32,
}

impl Drawing for ImageDrawing {
    fn draw(&self, printer: &mut PersistentPrinter, ctx: &mut Context) -> anyhow::Result<()> {
        let mut image = self.image.clone();
        if self.bright {
            for pixel in image.pixels_mut() {
                let mut color = LinLumaa::from_color(color::from_image_color(*pixel));
                color.luma = 1.0 - 0.4 * (1.0 - color.luma);
                *pixel = color::to_image_color(color.into_color());
            }
        }

        let mut tree = Tree::<Context>::new(WHITE);

        let image = Image::new(image)
            .with_dither_palette(&[BLACK, WHITE])
            .with_dither_algorithm(self.algo)
            .with_scale(self.scale)
            .node()
            .register(&mut tree)?;

        let root = Node::empty()
            .with_size_width(percent(1.0))
            .with_padding_bottom(length(FEED))
            .with_display(Display::Flex)
            .with_flex_direction(FlexDirection::Column)
            .with_align_items(Some(AlignItems::Center))
            .and_child(image)
            .register(&mut tree)?;

        printer.print_tree(&mut tree, ctx, root)?;
        Ok(())
    }
}
