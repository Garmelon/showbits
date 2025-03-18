mod color;
mod documents;
mod drawer;
mod persistent_printer;
mod printer;
mod server;

use std::{path::PathBuf, time::Duration};

use clap::Parser;
use drawer::Command;
use tokio::{runtime::Runtime, sync::mpsc};

use self::{drawer::Drawer, persistent_printer::PersistentPrinter};

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Parser)]
struct Args {
    /// Path to the queue directory.
    queue: PathBuf,

    /// Address the web server will listen at.
    #[arg(long, short, default_value = "localhost:8080")]
    address: String,

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
    runtime.spawn(server::run(tx.clone(), args.address));
    runtime.spawn(async move {
        loop {
            let _ = tx.send(Command::Backlog).await;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    println!("Running");
    drawer.run()?;

    Ok(())
}
