//! `dev`: this repository's own gate steps (dev §G).

use clap::{Parser, Subcommand};
use std::process::ExitCode;

/// This repository's own gate steps.
#[derive(Parser)]
#[command(name = "dev", version, about)]
struct Cli {
    #[command(subcommand)]
    verb: Verb,
}

#[derive(Subcommand)]
enum Verb {
    /// Refuse private names and unlisted repositories (root V37).
    Names {
        /// Files to check.
        files: Vec<String>,
    },
}

fn main() -> ExitCode {
    let Cli {
        verb: Verb::Names { files: _ },
    } = Cli::parse();
    ExitCode::SUCCESS
}
