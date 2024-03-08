mod drawer;
mod printer;
mod server;

use std::path::PathBuf;

use clap::Parser;
use drawer::Drawer;
use printer::Printer;
use tokio::{runtime::Runtime, sync::mpsc};

#[derive(Parser)]
struct Args {
    /// Address the web server will listen at.
    addr: String,

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

    let printer = Printer::new(args.printer, args.export)?;
    let mut drawer = Drawer::new(rx, printer);

    let runtime = Runtime::new()?;
    runtime.spawn(server::run(tx, args.addr));

    println!("Running");
    drawer.run()?;

    Ok(())
}
