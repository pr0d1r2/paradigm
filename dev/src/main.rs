//! `dev`: this repository's own gate steps (dev §G).

use clap::Parser;

/// This repository's own gate steps.
#[derive(Parser)]
#[command(name = "dev", version, about)]
struct Cli {}

fn main() {
    Cli::parse();
}
