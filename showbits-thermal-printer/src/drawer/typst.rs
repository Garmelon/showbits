use showbits_common::{Node, Tree, WidgetExt, color::WHITE, widgets::Typst};
use taffy::{prelude::length, style_helpers::percent};

use crate::persistent_printer::PersistentPrinter;

use super::{Context, Drawing, FEED};

pub struct TypstDrawing(pub String);

impl Drawing for TypstDrawing {
    fn draw(&self, printer: &mut PersistentPrinter, ctx: &mut Context) -> anyhow::Result<()> {
        let mut tree = Tree::<Context>::new(WHITE);

        let typst = Typst::new(self.0.clone()).node().register(&mut tree)?;

        let root = Node::empty()
            .with_size_width(percent(1.0))
            .with_padding_bottom(length(FEED))
            .and_child(typst)
            .register(&mut tree)?;

        printer.print_tree(&mut tree, ctx, root)?;
        Ok(())
    }
}
