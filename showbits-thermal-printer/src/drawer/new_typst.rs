use showbits_typst::Typst;

use crate::persistent_printer::PersistentPrinter;

use super::{Context, Drawing};

pub struct NewTypstDrawing(pub Typst);

impl NewTypstDrawing {
    pub fn new(typst: impl Into<Typst>) -> Self {
        Self(typst.into())
    }
}

impl Drawing for NewTypstDrawing {
    fn draw(&self, printer: &mut PersistentPrinter, _ctx: &mut Context) -> anyhow::Result<()> {
        let image = self.0.render()?;
        printer.print_image(&image)?;
        Ok(())
    }
}
