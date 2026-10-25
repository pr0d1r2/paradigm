//! `dev`: this repository's own gate steps (dev §G). Reads files, calls the
//! library, maps verdicts to exit codes: 0 clean, 1 violation, 2 usage.

use clap::{Parser, Subcommand};
use paradigm_dev::allowlist::Allowlist;
use paradigm_dev::denylist::Denylist;
use paradigm_dev::finding::Finding;
use paradigm_dev::message;
use paradigm_dev::names::check;
use sherdwright::history::drop_git_vars;
use std::fs;
use std::process::{Command, ExitCode};

const ALLOW: &str = "dev/public-repos.toml";
const DENY: &str = ".private/denylist";

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
    /// Refuse a message without a parsable stage trailer or a spec id.
    CommitMsg {
        /// The message file git passes the hook.
        file: String,
    },
}

fn main() -> ExitCode {
    match Cli::parse().verb {
        Verb::Names { files } => names(&files),
        Verb::CommitMsg { file } => commit_msg(&file),
    }
}

fn names(files: &[String]) -> ExitCode {
    let allow = match allowlist() {
        Ok(allow) => allow,
        Err(e) => {
            eprintln!("dev names: {ALLOW}: {e}");
            return ExitCode::from(2);
        }
    };
    let deny = denylist();
    let found: Vec<Finding> = files
        .iter()
        .flat_map(|f| scan(f, &allow, deny.as_ref()))
        .collect();
    verdict(&found)
}

fn allowlist() -> Result<Allowlist, String> {
    let text = fs::read_to_string(ALLOW).map_err(|e| e.to_string())?;
    Allowlist::parse(&text)
}

/// Prints each finding; exit 1 when there is any.
fn verdict(found: &[Finding]) -> ExitCode {
    for finding in found {
        println!("{finding}");
    }
    if found.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

/// The denylist when present; when absent, says so loudly and checks the
/// allowlist only, as CI does (root V37).
fn denylist() -> Option<Denylist> {
    if let Ok(text) = fs::read_to_string(DENY) {
        return Some(Denylist::parse(&text));
    }
    eprintln!("dev names: no {DENY}; private names unchecked, allowlist only");
    None
}

/// Findings for one file; a file that is gone or not UTF-8 holds no names.
fn scan(path: &str, allow: &Allowlist, deny: Option<&Denylist>) -> Vec<Finding> {
    let text = fs::read(path)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok());
    text.map(|text| check(path, &text, allow, deny))
        .unwrap_or_default()
}

fn commit_msg(file: &str) -> ExitCode {
    let read = fs::read_to_string(file).map_err(|e| e.to_string());
    let input = read.and_then(|text| trailers(file).map(|t| (text, t)));
    let (text, parsed) = match input {
        Ok(input) => input,
        Err(e) => {
            eprintln!("dev commit-msg: {file}: {e}");
            return ExitCode::from(2);
        }
    };
    let Err(e) = message::check(&text, &parsed) else {
        return ExitCode::SUCCESS;
    };
    eprintln!("dev commit-msg: {e}");
    ExitCode::from(1)
}

/// git's own reading of the message's trailers. The hook's `GIT_*`
/// variables are dropped so this child git acts on nothing else (root V15).
fn trailers(file: &str) -> Result<String, String> {
    let mut git = Command::new("git");
    git.args(["interpret-trailers", "--parse", file]);
    drop_git_vars(&mut git, std::env::vars().map(|(k, _)| k));
    let out = git.output().map_err(|e| e.to_string())?;
    String::from_utf8(out.stdout).map_err(|e| e.to_string())
}
