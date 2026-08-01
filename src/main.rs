//! Process boundary for the `foundry` binary.

use std::process::ExitCode;

fn main() -> ExitCode {
    foundry::cli::run()
}
