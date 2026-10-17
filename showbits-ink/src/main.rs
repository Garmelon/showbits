mod display;
mod prepare;
mod queue;
mod server;

use std::{path::PathBuf, thread, time::Duration};

use clap::Parser;
use tokio::runtime::Runtime;

use crate::queue::Queue;

#[derive(Debug, Parser)]
struct Args {
    /// Address for the web server to listen at.
    #[arg(long, short, default_value = "localhost:8080")]
    address: String,

    /// Directory to maintain image queue in.
    #[arg(short = 'Q', long)]
    queue: PathBuf,

    /// Directory to archive displayed images to.
    #[arg(short = 'A', long)]
    archive: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let queue = Queue::new(args.queue, args.archive)?;

    let runtime = Runtime::new()?;
    runtime.spawn(server::run(args.address, queue.clone()));

    loop {
        if let Some(entry) = queue.next_entry()? {
            display::display(entry.image())?;
            entry.dequeue()?;
        } else {
            thread::sleep(Duration::from_millis(100));
        }
    }
}
