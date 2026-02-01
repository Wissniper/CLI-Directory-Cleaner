use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub path: String,

    #[arg(short, long, default_value_t = false)]
    pub dry_run: bool,
    // dry run = making all the calculations, then stopping before execution and printing what would have been done normally
}
