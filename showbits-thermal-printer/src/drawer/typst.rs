use showbits_common::{color::WHITE, widgets::Typst, Node, Tree, WidgetExt};
use taffy::style_helpers::percent;

use crate::printer::Printer;

use super::{Context, Drawing};

pub struct TypstDrawing(pub String);

impl Drawing for TypstDrawing {
    fn draw(&self, printer: &mut Printer, ctx: &mut Context) -> anyhow::Result<()> {
        let mut tree = Tree::<Context>::new(WHITE);

        let typst = Typst::new(self.0.clone()).node().register(&mut tree)?;

        let root = Node::empty()
            .with_size_width(percent(1.0))
            .and_child(typst)
            .register(&mut tree)?;

        printer.print_tree(&mut tree, ctx, root)?;
        printer.feed()?;
        Ok(())
    }
}
