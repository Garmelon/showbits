mod color;
mod display;

use std::{path::PathBuf, thread, time::Duration};

use anyhow::bail;
use clap::Parser;
use image::RgbImage;
use showbits_common::{Queue, Server, run_server};
use tokio::runtime::Runtime;

use crate::color::Color;

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

#[derive(Clone)]
struct InkServer(Queue);

impl Server for InkServer {
    fn queue(&self) -> &Queue {
        &self.0
    }

    fn prepare(&self, mut image: RgbImage) -> anyhow::Result<RgbImage> {
        if image.width() != display::WIDTH || image.height() != display::HEIGHT {
            bail!("image must be {}x{}", display::WIDTH, display::HEIGHT);
        }

        for pixel in image.pixels_mut() {
            *pixel = Color::from_rgb(*pixel).into_rgb();
        }

        Ok(image)
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let queue = Queue::new(args.queue, args.archive)?;
    let server = InkServer(queue.clone());

    let runtime = Runtime::new()?;
    runtime.spawn(run_server(server, args.address));

    loop {
        if let Some(entry) = queue.next_entry()? {
            display::display(entry.image())?;
            entry.dequeue()?;
        }
        thread::sleep(Duration::from_millis(100));
    }
}
