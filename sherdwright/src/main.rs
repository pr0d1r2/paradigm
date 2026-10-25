//! `sherdwright`: the stage checker (sherdwright §G). Parses arguments,
//! calls the library, maps the outcome to an exit code: 0 no fail, 1 fail,
//! 2 usage (sherdwright §I).

use clap::{Parser, Subcommand, ValueEnum};
use sherdwright::report::Report;
use sherdwright::{digest, history};
use std::path::{Path, PathBuf};
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
    /// Print what changed in every spec's §V, §I and §F between the last
    /// commits carrying `reconcile-<A>` and `reconcile-<B>`.
    Digest {
        /// The earlier reconciliation's round.
        a: u32,
        /// The later reconciliation's round.
        b: u32,
        /// The member's checkout.
        #[arg(long, default_value = ".")]
        dir: PathBuf,
        /// How to print the digest.
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
    match Cli::parse().verb {
        Verb::Check { dir, format } => check(&dir, format),
        Verb::Digest { a, b, dir, format } => digest(&dir, a, b, format),
    }
}

/// The report for the member at `dir`. No stage check runs yet, so every
/// report is empty until sherdwright T4 to T7 land (root V41).
fn check(dir: &Path, format: Format) -> ExitCode {
    if let Err(e) = history::trailers(dir) {
        eprintln!("sherdwright check: {}: {e}", dir.display());
        return ExitCode::from(2);
    }
    let report = Report::default();
    print!("{}", render(&report, format));
    ExitCode::from(report.exit_code())
}

fn render(report: &Report, format: Format) -> String {
    match format {
        Format::Human => report.human(),
        Format::Json => report.json(),
    }
}

/// What changed in the specs between two reconciliations (sherdwright T9).
fn digest(dir: &Path, a: u32, b: u32, format: Format) -> ExitCode {
    match digest::digest(dir, a, b) {
        Ok(changes) => {
            let text = match format {
                Format::Human => digest::human(a, b, &changes),
                Format::Json => digest::json(a, b, &changes),
            };
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("sherdwright digest: {}: {e}", dir.display());
            ExitCode::from(2)
        }
    }
}
