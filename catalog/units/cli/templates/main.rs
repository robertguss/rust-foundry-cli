//! {{name}} — generated pure CLI.

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "{{name}}", version, about = "Generated CLI")]
struct Cli {
    /// Optional message to print
    #[arg(long)]
    message: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    match cli.message {
        Some(m) => println!("{m}"),
        None => println!("{{name}}"),
    }
}
