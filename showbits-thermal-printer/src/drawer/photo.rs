use image::{Luma, Pixel, RgbaImage};
use showbits_common::{
    Node, Tree, WidgetExt,
    color::{BLACK, WHITE},
    widgets::{Image, Text},
};
use taffy::{
    AlignItems, Display, FlexDirection,
    style_helpers::{length, percent},
};

use crate::persistent_printer::PersistentPrinter;

use super::{Context, Drawing, FEED};

pub struct PhotoDrawing {
    pub image: RgbaImage,
    pub title: String,
}

impl Drawing for PhotoDrawing {
    fn draw(&self, printer: &mut PersistentPrinter, ctx: &mut Context) -> anyhow::Result<()> {
        let mut tree = Tree::<Context>::new(WHITE);

        let mut image = self.image.clone();
        for pixel in image.pixels_mut() {
            let [l] = pixel.to_luma().0;
            let l = l as f32 / 255.0; // Convert to [0, 1]
            let l = 1.0 - (0.4 * (1.0 - l)); // Lerp to [0.6, 1]
            let l = (l.clamp(0.0, 1.0) * 255.0) as u8; // Convert back to [0, 255]
            *pixel = Luma([l]).to_rgba();
        }

        let image = Image::new(image)
            .with_dither_palette(&[BLACK, WHITE])
            .node()
            .register(&mut tree)?;

        let title = Text::new()
            .with_metrics(Text::default_metrics().scale(2.0))
            .and_plain(&self.title)
            .widget(&mut ctx.font_stuff)
            .node()
            .register(&mut tree)?;

        let root = Node::empty()
            .with_size_width(percent(1.0))
            .with_padding_bottom(length(FEED))
            .with_display(Display::Flex)
            .with_flex_direction(FlexDirection::Column)
            .with_align_items(Some(AlignItems::Center))
            .with_gap(length(8.0))
            .and_child(image)
            .and_child(title)
            .register(&mut tree)?;

        printer.print_tree(&mut tree, ctx, root)?;
        Ok(())
    }
}
