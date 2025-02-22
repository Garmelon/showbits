use image::{
    imageops::{self, FilterType},
    Rgba, RgbaImage,
};
use showbits_common::{color, widgets::Image, Node, Tree, WidgetExt};
use taffy::{style_helpers::percent, AlignItems, Display, FlexDirection};

use crate::printer::Printer;

use super::{Context, Drawing};

const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);
const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);

fn b2c(bool: bool) -> Rgba<u8> {
    match bool {
        true => BLACK,
        false => WHITE,
    }
}

fn c2b(color: Rgba<u8>) -> bool {
    color == BLACK
}

fn neighbors_at(image: &RgbaImage, x: u32, y: u32) -> [bool; 3] {
    let left = x
        .checked_sub(1)
        .map(|x| *image.get_pixel(x, y))
        .unwrap_or(WHITE);

    let mid = *image.get_pixel(x, y);

    let right = image.get_pixel_checked(x + 1, y).copied().unwrap_or(WHITE);

    [c2b(left), c2b(mid), c2b(right)]
}

fn apply_rule(rule: u8, neighbors: [bool; 3]) -> bool {
    let [left, mid, right] = neighbors.map(|n| n as u8);
    let index = (left << 2) | (mid << 1) | right;
    rule & (1 << index) != 0
}

pub struct CellsDrawing {
    pub rule: u8,
    pub rows: u32,
    pub scale: u32,
}

impl Drawing for CellsDrawing {
    fn draw(&self, printer: &mut Printer, ctx: &mut Context) -> anyhow::Result<()> {
        let mut image = RgbaImage::new(Printer::WIDTH / self.scale, self.rows);

        // Initialize first line randomly
        for x in 0..image.width() {
            image.put_pixel(x, 0, b2c(rand::random()));
        }

        // Calculate next rows
        for y in 1..self.rows {
            for x in 0..image.width() {
                let neighbors = neighbors_at(&image, x, y - 1);
                let state = apply_rule(self.rule, neighbors);
                image.put_pixel(x, y, b2c(state));
            }
        }

        let image = imageops::resize(
            &image,
            image.width() * self.scale,
            image.height() * self.scale,
            FilterType::Nearest,
        );

        let mut tree = Tree::<Context>::new(color::WHITE);

        let image = Image::new(image).node().register(&mut tree)?;

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
