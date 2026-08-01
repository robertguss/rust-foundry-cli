//! Clap wiring only (I/O boundary). Domain logic must not live only here.

use clap::{Parser, Subcommand};

use crate::VERSION;

/// Foundry — validate / plan / generate (PHASE-01 scaffold).
#[derive(Debug, Parser)]
#[command(
    name = "foundry",
    version = VERSION,
    about = "AI-native hybrid foundry for modern Rust projects",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Print package version (catalog digest lands in PHASE-02 / MS-007).
    Version,
    // validate / plan / generate / catalog land in MS-002+
}

/// Parse argv and execute the selected command.
pub fn run() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Version => {
            println!("foundry {VERSION}");
        }
    }
}
