use showbits_common::{
    color::{BLACK, WHITE},
    widgets::{Block, Text},
    Node, Tree, WidgetExt,
};
use taffy::{
    style_helpers::{length, percent, repeat},
    AlignItems, Display, FlexDirection,
};

use crate::printer::Printer;

use super::{Context, Drawing};

pub struct TicTacToeDrawing;

impl Drawing for TicTacToeDrawing {
    fn draw(&self, printer: &mut Printer, ctx: &mut Context) -> anyhow::Result<()> {
        let block_size = length(128.0);
        let width = length(2.0);

        let mut tree = Tree::<Context>::new(WHITE);

        let mut grid = Node::empty()
            .with_display(Display::Grid)
            .with_grid_template_columns(vec![repeat(3, vec![block_size])])
            .with_grid_auto_rows(vec![block_size]);

        for y in 0..3 {
            for x in 0..3 {
                let mut block = Block::new().with_border(BLACK).node();

                if x >= 1 {
                    block = block.with_border_left(width);
                }
                if x <= 1 {
                    block = block.with_border_right(width);
                }

                if y >= 1 {
                    block = block.with_border_top(width);
                }
                if y <= 1 {
                    block = block.with_border_bottom(width);
                }

                grid = grid.and_child(block.register(&mut tree)?);
            }
        }

        let title = Text::new()
            .with_metrics(Text::default_metrics().scale(2.0))
            .and_plain("Tick-Tack-Zeh")
            .widget(&mut ctx.font_stuff)
            .node()
            .register(&mut tree)?;

        let root = Node::empty()
            .with_size_width(percent(1.0))
            .with_display(Display::Flex)
            .with_flex_direction(FlexDirection::Column)
            .with_align_items(Some(AlignItems::Center))
            .with_gap(length(16.0))
            .and_child(title)
            .and_child(grid.register(&mut tree)?)
            .register(&mut tree)?;

        printer.print_tree(&mut tree, ctx, root)?;
        printer.feed()?;
        Ok(())
    }
}
