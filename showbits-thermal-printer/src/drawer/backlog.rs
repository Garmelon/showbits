use crate::persistent_printer::PersistentPrinter;

use super::{Context, Drawing};

pub struct BacklogDrawing;

impl Drawing for BacklogDrawing {
    fn draw(&self, printer: &mut PersistentPrinter, _ctx: &mut Context) -> anyhow::Result<()> {
        printer.print_backlog()?;
        Ok(())
    }
}
