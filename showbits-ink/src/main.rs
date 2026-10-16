mod display;
mod prepare;

use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    image: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let image = image::open(&args.image)?.into_rgba8();
    let image = prepare::prepare(image);
    display::display(&image)?;
    Ok(())
}
