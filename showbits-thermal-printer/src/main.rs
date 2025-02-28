mod drawer;
mod persistent_printer;
mod printer;
mod server;
mod documents;

use std::{path::PathBuf, time::Duration};

use clap::Parser;
use drawer::{BacklogDrawing, Command};
use tokio::{runtime::Runtime, sync::mpsc};

use self::{drawer::Drawer, persistent_printer::PersistentPrinter};

#[derive(Parser)]
struct Args {
    /// Address the web server will listen at.
    addr: String,

    /// Path to the queue directory.
    queue: PathBuf,

    /// Path to the printer's USB device file.
    ///
    /// Usually, this is located at `/dev/usb/lp0` or a similar location.
    #[arg(long, short)]
    printer: Option<PathBuf>,

    /// Export an image of whatever is printed here.
    #[arg(long, short)]
    export: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let (tx, rx) = mpsc::channel(3);

    let printer = PersistentPrinter::new(args.printer, args.export, args.queue);
    let mut drawer = Drawer::new(rx, printer);

    let runtime = Runtime::new()?;
    runtime.spawn(server::run(tx.clone(), args.addr));
    runtime.spawn(async move {
        loop {
            let _ = tx.send(Command::draw(BacklogDrawing)).await;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    println!("Running");
    drawer.run()?;

    Ok(())
}
