// src/main.rs
/*
 * Main executable for NftMarketplaceEngineSDKUltra
 */

use clap::Parser;
use nftmarketplaceenginesdkultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMarketplaceEngineSDKUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
