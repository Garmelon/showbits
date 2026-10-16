use clap::Parser;

#[derive(Debug, Parser)]
struct Args {}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("{:?}", args);
    Ok(())
}
