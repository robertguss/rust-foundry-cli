//! Clap wiring only (I/O boundary). Domain logic must not live only here.

use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};

use crate::VERSION;
use crate::spec::{
    CliOverrides, EffectiveInputs, SpecError, VerifyMode, load_spec, normalize_effective_inputs,
};

/// Foundry — validate / plan / generate (PHASE-01).
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
    /// Validate a Project Spec (write-free w.r.t. destination).
    Validate {
        /// Project Spec path, or `-` for stdin (REQ-031).
        #[arg(long = "spec", value_name = "PATH")]
        spec: String,
        /// Override TOML `name` (REQ-034).
        #[arg(long = "name", value_name = "NAME")]
        name: Option<String>,
        /// Override TOML `destination` (REQ-034).
        #[arg(long = "dest", value_name = "PATH")]
        dest: Option<String>,
        /// Override TOML `verify` (REQ-034).
        #[arg(long = "verify", value_enum)]
        verify: Option<CliVerifyMode>,
    },
    // plan / generate / catalog land in later milestones
}

/// CLI verify override values (matches TOML set).
#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliVerifyMode {
    None,
    Default,
    Strict,
}

impl From<CliVerifyMode> for VerifyMode {
    fn from(value: CliVerifyMode) -> Self {
        match value {
            CliVerifyMode::None => VerifyMode::None,
            CliVerifyMode::Default => VerifyMode::Default,
            CliVerifyMode::Strict => VerifyMode::Strict,
        }
    }
}

/// Parse argv and execute the selected command.
pub fn run() -> ExitCode {
    let cli = Cli::parse();
    match execute(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(code) => code,
    }
}

fn execute(cli: Cli) -> Result<(), ExitCode> {
    match cli.command {
        Commands::Version => {
            println!("foundry {VERSION}");
            Ok(())
        }
        Commands::Validate {
            spec,
            name,
            dest,
            verify,
        } => {
            let project = load_and_effective(spec, name, dest, verify)?;
            print_validate_ok(&project);
            Ok(())
        }
    }
}

fn load_and_effective(
    spec_path: String,
    name: Option<String>,
    dest: Option<String>,
    verify: Option<CliVerifyMode>,
) -> Result<EffectiveInputs, ExitCode> {
    let project = load_spec(&spec_path).map_err(emit_spec_error)?;
    normalize_effective_inputs(
        project,
        CliOverrides {
            name,
            dest,
            verify: verify.map(Into::into),
        },
    )
    .map_err(emit_spec_error)
}

fn print_validate_ok(inputs: &EffectiveInputs) {
    println!("foundry validate: ok");
    println!("  source: {}", inputs.source);
    println!("  schema: {}", inputs.schema);
    println!("  name: {}", inputs.name);
    println!("  archetype: {}", inputs.archetype);
    println!("  destination: {}", inputs.destination);
    if inputs.profiles.is_empty() {
        println!("  profiles: []");
    } else {
        println!("  profiles: [{}]", inputs.profiles.join(", "));
    }
    // Always show effective verify (defaulted when TOML/CLI omit).
    println!("  verify: {}", inputs.verify.as_str());
    if let Some(desc) = &inputs.description {
        println!("  description: {desc}");
    }
}

fn emit_spec_error(err: SpecError) -> ExitCode {
    eprintln!("error[{}]: {}", err.code, err.message);
    ExitCode::from(1)
}
