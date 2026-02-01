mod args;
mod logic;

use anyhow::Result;
use args::Cli;
use clap::Parser;

fn main() -> Result<()> {
    let args = Cli::parse();

    // Handle the Result - if it fails, convert error and propagate with ?
    logic::process_directory(&args.path, args.dry_run)
        .map_err(|_| anyhow::anyhow!("Failed to process directory"))?;

    Ok(())
}
