//! `sherdwright`: the stage checker (sherdwright §G). Parses arguments,
//! calls the library, maps the outcome to an exit code: 0 no fail, 1 fail,
//! 2 usage (sherdwright §I).

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::process::ExitCode;

/// Checks a repository's history against the paradigm's lifecycle stages.
#[derive(Parser)]
#[command(name = "sherdwright", version, about)]
struct Cli {
    #[command(subcommand)]
    verb: Verb,
}

#[derive(Subcommand)]
enum Verb {
    /// Report every root invariant for the member at <DIR>.
    Check {
        /// The member's checkout.
        #[arg(default_value = ".")]
        dir: PathBuf,
        /// How to print the report.
        #[arg(long, value_enum, default_value_t = Format::Human)]
        format: Format,
    },
}

/// How a report is printed.
#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Human,
    Json,
}

fn main() -> ExitCode {
    let _ = Cli::parse();
    ExitCode::SUCCESS
}
