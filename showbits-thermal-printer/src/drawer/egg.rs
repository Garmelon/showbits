use image::{RgbaImage, imageops};
use rand::{Rng, seq::IndexedRandom};
use showbits_assets::{EGG_BAD_COVERS, EGG_BAD_PATTERNS, EGG_COVERS, EGG_PATTERNS};
use showbits_common::{
    Node, Tree, WidgetExt,
    color::{self, WHITE},
    widgets::{Image, Text},
};
use taffy::{prelude::length, style_helpers::percent, AlignItems, Display, FlexDirection};

use crate::persistent_printer::PersistentPrinter;

use super::{Context, Drawing, FEED};

pub struct EggDrawing;

fn load_image(bytes: &[u8]) -> RgbaImage {
    image::load_from_memory(bytes)
        .expect("malformed image data")
        .into_rgba8()
}

impl Drawing for EggDrawing {
    fn draw(&self, printer: &mut PersistentPrinter, ctx: &mut Context) -> anyhow::Result<()> {
        let mut rng = rand::rng();

        // Choose which set of egg images to use
        let bad_egg = rng.random_range(0..8) == 0;
        let (covers, patterns) = if bad_egg {
            (EGG_BAD_COVERS, EGG_BAD_PATTERNS)
        } else {
            (EGG_COVERS, EGG_PATTERNS)
        };

        // Load images from memory
        let covers = covers.iter().map(|img| load_image(img)).collect::<Vec<_>>();
        let patterns = patterns
            .iter()
            .map(|img| load_image(img))
            .collect::<Vec<_>>();

        // Choose a random cover
        let cover = covers.choose(&mut rng).expect("too few covers");

        // Prepare image of appropriate size
        let mut image =
            RgbaImage::from_pixel(cover.width(), cover.height(), color::to_image_color(WHITE));

        // Draw patterns onto egg
        let mut last_idx = None;
        let mut y = rng.random_range(-100_i64..0);
        let height: i64 = image.height().into();
        while y < height {
            let idx = loop {
                let idx = rng.random_range(0..patterns.len());
                if Some(idx) != last_idx {
                    break idx;
                }
            };

            let paint = &patterns[idx];
            imageops::overlay(&mut image, paint, 0, y);
            y += <_ as Into<i64>>::into(paint.height());
            last_idx = Some(idx);
        }

        // Finally, draw the cover
        imageops::overlay(&mut image, cover, 0, 0);

        let mut tree = Tree::<Context>::new(WHITE);

        let image = Image::new(image)
            .with_grow(false)
            .with_shrink(false)
            .node()
            .register(&mut tree)?;

        let text = Text::new()
            .with_metrics(Text::default_metrics().scale(2.0))
            .and_plain("Frohe Ostern!")
            .widget(&mut ctx.font_stuff)
            .node()
            .register(&mut tree)?;

        let root = Node::empty()
            .with_size_width(percent(1.0))
            .with_padding_bottom(length(FEED))
            .with_display(Display::Flex)
            .with_flex_direction(FlexDirection::Column)
            .with_align_items(Some(AlignItems::Center))
            .and_child(image)
            .and_child(text)
            .register(&mut tree)?;

        printer.print_tree(&mut tree, ctx, root)?;
        Ok(())
    }
}
