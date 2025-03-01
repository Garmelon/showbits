mod backlog;
mod calendar;
mod cells;
mod chat_message;
mod new_typst;
mod photo;
mod tictactoe;
mod typst;

use showbits_common::widgets::{FontStuff, HasFontStuff};
use tokio::sync::mpsc;

use crate::persistent_printer::PersistentPrinter;

pub use self::{
    backlog::BacklogDrawing, calendar::CalendarDrawing, cells::CellsDrawing,
    chat_message::ChatMessageDrawing, new_typst::NewTypstDrawing, photo::PhotoDrawing,
    tictactoe::TicTacToeDrawing, typst::TypstDrawing,
};

pub const FEED: f32 = 96.0;

#[derive(Default)]
pub struct Context {
    font_stuff: FontStuff,
}

pub trait Drawing {
    fn draw(&self, printer: &mut PersistentPrinter, ctx: &mut Context) -> anyhow::Result<()>;
}

pub struct Command(Box<dyn Drawing + Send>);

impl Command {
    pub fn draw<D: Drawing + Send + 'static>(drawing: D) -> Self {
        Self(Box::new(drawing))
    }
}

impl HasFontStuff for Context {
    fn font_stuff(&mut self) -> &mut FontStuff {
        &mut self.font_stuff
    }
}

pub struct Drawer {
    rx: mpsc::Receiver<Command>,
    printer: PersistentPrinter,
    ctx: Context,
}

impl Drawer {
    pub fn new(rx: mpsc::Receiver<Command>, printer: PersistentPrinter) -> Self {
        Self {
            rx,
            printer,
            ctx: Context::default(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        while let Some(command) = self.rx.blocking_recv() {
            command.0.draw(&mut self.printer, &mut self.ctx)?;
        }
        Ok(())
    }
}
