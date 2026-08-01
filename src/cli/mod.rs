//! Clap wiring only (I/O boundary). Domain logic must not live only here.

use std::fs;
use std::path::Path;
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};

use crate::VERSION;
use crate::catalog::stub_catalog;
use crate::plan::{ConstructError, construct};
use crate::report::{ReportFormat, format_error_json, format_plan};
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
        /// Override TOML `name` (REQ-034; public CLI surface).
        #[arg(long = "name", value_name = "NAME")]
        name: Option<String>,
        /// Override TOML `destination` (REQ-034; public CLI surface).
        #[arg(long = "dest", value_name = "PATH")]
        dest: Option<String>,
        /// Override TOML `verify` (REQ-034; public CLI surface).
        #[arg(long = "verify", value_enum)]
        verify: Option<CliVerifyMode>,
    },
    /// Emit Generation Plan (write-free w.r.t. destination; REQ-040..043).
    ///
    /// Non-interactive. Exit 0 on success; exit 1 on spec/resolve/construct
    /// failure. Does not prompt (REQ-021/023).
    Plan {
        /// Project Spec path, or `-` for stdin (REQ-031).
        #[arg(long = "spec", value_name = "PATH")]
        spec: String,
        /// Override TOML `name` (REQ-034; public CLI surface).
        #[arg(long = "name", value_name = "NAME")]
        name: Option<String>,
        /// Override TOML `destination` (REQ-034; public CLI surface).
        #[arg(long = "dest", value_name = "PATH")]
        dest: Option<String>,
        /// Override TOML `verify` (REQ-034; public CLI surface).
        #[arg(long = "verify", value_enum)]
        verify: Option<CliVerifyMode>,
        /// Report format: `text` (default) or `json` (REQ-042).
        #[arg(long = "format", value_enum, default_value_t = CliReportFormat::Text)]
        format: CliReportFormat,
        /// Write plan report to FILE (not destination place; REQ-043 exception).
        #[arg(long = "out", value_name = "FILE")]
        out: Option<String>,
    },
    // generate / catalog land in later milestones
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

/// CLI plan report format.
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
enum CliReportFormat {
    #[default]
    Text,
    Json,
}

impl From<CliReportFormat> for ReportFormat {
    fn from(value: CliReportFormat) -> Self {
        match value {
            CliReportFormat::Text => ReportFormat::Text,
            CliReportFormat::Json => ReportFormat::Json,
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
        Commands::Plan {
            spec,
            name,
            dest,
            verify,
            format,
            out,
        } => {
            let report_format = ReportFormat::from(format);
            let inputs = match load_and_effective(spec, name, dest, verify) {
                Ok(i) => i,
                Err(code) => {
                    // load_and_effective already printed text error; for JSON
                    // format re-emit is harder without the SpecError — text path
                    // is fine for validate-style errors on plan too.
                    return Err(code);
                }
            };
            let plan = construct(&inputs, &stub_catalog())
                .map_err(|e| emit_construct_error(&e, report_format))?;
            let body = format_plan(&plan, report_format);
            emit_report(&body, out.as_deref())?;
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

fn emit_report(body: &str, out: Option<&str>) -> Result<(), ExitCode> {
    match out {
        None => {
            print!("{body}");
            if !body.ends_with('\n') {
                println!();
            }
            Ok(())
        }
        Some(path) => {
            if let Some(parent) = Path::new(path).parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent).map_err(|e| {
                        eprintln!("error[report.write]: cannot create parent for {path}: {e}");
                        ExitCode::from(1)
                    })?;
                }
            }
            fs::write(path, body).map_err(|e| {
                eprintln!("error[report.write]: cannot write {path}: {e}");
                ExitCode::from(1)
            })?;
            Ok(())
        }
    }
}

fn emit_spec_error(err: SpecError) -> ExitCode {
    eprintln!("error[{}]: {}", err.code, err.message);
    ExitCode::from(1)
}

fn emit_construct_error(err: &ConstructError, format: ReportFormat) -> ExitCode {
    match format {
        ReportFormat::Text => {
            eprintln!("error[{}]: {}", err.code, err.message);
        }
        ReportFormat::Json => {
            // Errors go to stderr as JSON so stdout stays clean for piping.
            eprintln!("{}", format_error_json(err.code, &err.message));
        }
    }
    ExitCode::from(1)
}
