use image::RgbaImage;
use palette::{FromColor, IntoColor, LinLumaa};
use showbits_common::{
    color::{self, BLACK, WHITE},
    widgets::Image,
    Node, Tree, WidgetExt,
};
use taffy::{style_helpers::percent, AlignItems, Display, FlexDirection};

use crate::printer::Printer;

use super::{Context, Drawing};

pub struct ImageDrawing {
    pub image: RgbaImage,
    pub bright: bool,
}

impl Drawing for ImageDrawing {
    fn draw(&self, printer: &mut Printer, ctx: &mut Context) -> anyhow::Result<()> {
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
            .node()
            .register(&mut tree)?;

        let root = Node::empty()
            .with_size_width(percent(1.0))
            .with_display(Display::Flex)
            .with_flex_direction(FlexDirection::Column)
            .with_align_items(Some(AlignItems::Center))
            .and_child(image)
            .register(&mut tree)?;

        printer.print_tree(&mut tree, ctx, root)?;
        printer.feed()?;
        Ok(())
    }
}
