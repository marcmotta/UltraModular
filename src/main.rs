// src/main.rs
/*
 * Main executable for UltraModular
 */

use clap::Parser;
use ultramodular::{Result, run};

#[derive(Parser)]
#[command(version, about = "UltraModular - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
