mod backlog;
mod new_typst;

use tokio::sync::mpsc;

use crate::persistent_printer::PersistentPrinter;

pub use self::{backlog::BacklogDrawing, new_typst::NewTypstDrawing};

pub trait Drawing {
    fn draw(&self, printer: &mut PersistentPrinter) -> anyhow::Result<()>;
}

pub struct Command(Box<dyn Drawing + Send>);

impl Command {
    pub fn draw<D: Drawing + Send + 'static>(drawing: D) -> Self {
        Self(Box::new(drawing))
    }
}

pub struct Drawer {
    rx: mpsc::Receiver<Command>,
    printer: PersistentPrinter,
}

impl Drawer {
    pub fn new(rx: mpsc::Receiver<Command>, printer: PersistentPrinter) -> Self {
        Self { rx, printer }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        while let Some(command) = self.rx.blocking_recv() {
            command.0.draw(&mut self.printer)?;
        }
        Ok(())
    }
}
