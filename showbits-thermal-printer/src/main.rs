mod command;
mod printer;
mod server;
mod util;

use std::path::PathBuf;

use clap::Parser;
use printer::Printer;
use tokio::{runtime::Runtime, sync::mpsc};

#[derive(Parser)]
struct Args {
    path: PathBuf,
    addr: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let (tx, rx) = mpsc::channel(3);
    let mut printer = Printer::new(rx, &args.path)?;

    let runtime = Runtime::new()?;
    runtime.spawn(server::run(tx, args.addr));

    println!("Running");
    printer.run()?;

    Ok(())
}
