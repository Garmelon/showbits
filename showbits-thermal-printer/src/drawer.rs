use showbits_typst::Typst;
use tokio::sync::{mpsc, oneshot};

use crate::persistent_printer::PersistentPrinter;

pub enum Command {
    Backlog,
    Typst(Typst, oneshot::Sender<anyhow::Result<()>>),
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
            self.run_cmd(command)?;
        }
        Ok(())
    }

    fn run_cmd(&mut self, command: Command) -> anyhow::Result<()> {
        match command {
            Command::Backlog => {
                self.printer.print_backlog()?;
            }
            Command::Typst(typst, tx) => {
                let _ = tx.send(self.run_cmd_typst(typst));
            }
        }
        Ok(())
    }

    fn run_cmd_typst(&mut self, typst: Typst) -> anyhow::Result<()> {
        let image = typst.render()?;
        self.printer.print_image(&image)?;
        Ok(())
    }
}
