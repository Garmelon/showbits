use showbits_common::{Node, Tree, WidgetExt, color::WHITE, widgets::Text};
use taffy::style_helpers::percent;

use crate::persistent_printer::PersistentPrinter;

use super::{Context, Drawing};

pub struct TextDrawing(pub String);

impl Drawing for TextDrawing {
    fn draw(&self, printer: &mut PersistentPrinter, ctx: &mut Context) -> anyhow::Result<()> {
        let mut tree = Tree::<Context>::new(WHITE);

        let text = Text::new()
            .with_metrics(Text::default_metrics().scale(2.0))
            .and_plain(&self.0)
            .widget(&mut ctx.font_stuff)
            .node()
            .register(&mut tree)?;

        let root = Node::empty()
            .with_size_width(percent(1.0))
            .and_child(text)
            .register(&mut tree)?;

        printer.print_tree(&mut tree, ctx, root)?;
        Ok(())
    }
}
