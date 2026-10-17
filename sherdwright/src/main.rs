//! `sherdwright`: the stage checker (sherdwright §G).

use clap::Parser;

/// Checks a repository's history against the paradigm's lifecycle stages.
#[derive(Parser)]
#[command(name = "sherdwright", version, about)]
struct Cli {}

fn main() {
    Cli::parse();
}
