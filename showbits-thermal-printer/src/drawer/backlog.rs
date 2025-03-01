use crate::persistent_printer::PersistentPrinter;

use super::Drawing;

pub struct BacklogDrawing;

impl Drawing for BacklogDrawing {
    fn draw(&self, printer: &mut PersistentPrinter) -> anyhow::Result<()> {
        printer.print_backlog()?;
        Ok(())
    }
}
