use showbits_common::{
    color::{BLACK, WHITE},
    widgets::{Block, Text},
    Node, Tree, WidgetExt,
};
use taffy::{
    style_helpers::{length, percent},
    AlignItems, Display, FlexDirection,
};

use crate::printer::Printer;

use super::{Context, Drawing};

pub struct ChatMessageDrawing {
    pub username: String,
    pub content: String,
}

impl Drawing for ChatMessageDrawing {
    fn draw(&self, printer: &mut Printer, ctx: &mut Context) -> anyhow::Result<()> {
        let mut tree = Tree::<Context>::new(WHITE);

        let max_username_width_in_chars = 32.0;
        let max_username_height_in_lines = 3.0;
        let max_content_height_in_lines = 16.0;

        let username = Text::new()
            .and_plain(&self.username)
            .widget(&mut ctx.font_stuff)
            .node()
            .with_max_size_width(length(max_username_width_in_chars * 8.0))
            .with_max_size_height(length(max_username_height_in_lines * 16.0))
            .register(&mut tree)?;

        let username = Block::new()
            .with_border(BLACK)
            .node()
            .with_border_all(length(1.0))
            .with_padding_horiz(length(1.0))
            .with_flex_shrink(0.0) // Avoid wrapping
            .and_child(username)
            .register(&mut tree)?;

        let content = if let Some(content) = self.content.strip_prefix("/me") {
            let content = content.trim_start();

            let content = Text::new()
                .and_plain(content)
                .widget(&mut ctx.font_stuff)
                .node()
                .with_max_size_height(length(max_content_height_in_lines * 16.0))
                .register(&mut tree)?;

            Block::new()
                .with_border(BLACK)
                .node()
                .with_border_all(length(1.0))
                .with_padding_horiz(length(1.0))
                .and_child(content)
                .register(&mut tree)?
        } else {
            let content = Text::new()
                .and_plain(&self.content)
                .widget(&mut ctx.font_stuff)
                .node()
                .with_max_size_height(length(max_content_height_in_lines * 16.0))
                .register(&mut tree)?;

            Node::empty()
                .with_padding_vert(length(1.0))
                .and_child(content)
                .register(&mut tree)?
        };

        let root = Node::empty()
            .with_size_width(percent(1.0))
            .with_padding_all(length(1.0))
            .with_display(Display::Flex)
            .with_flex_direction(FlexDirection::Row)
            .with_align_items(Some(AlignItems::Start))
            .with_gap_width(length(2.0))
            .and_child(username)
            .and_child(content)
            .register(&mut tree)?;

        printer.print_tree(&mut tree, ctx, root)?;
        Ok(())
    }
}
