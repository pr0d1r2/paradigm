//! A member's history, as git reads it.

use std::path::Path;
use std::process::Command;

/// One commit and the stages its trailers name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    /// The full commit hash.
    pub sha: String,
    /// Each `Paradigm-Stage:` trailer value git parses, in order.
    pub stages: Vec<String>,
}

/// Separates the hash from the trailers, the trailers from each other,
/// and one commit from the next. None can occur in a hash or a trailer.
const FIELD: char = '\u{1f}';
const VALUE: char = '\u{1e}';
const RECORD: char = '\u{1d}';

/// Every commit reachable from `HEAD` in `repo`, oldest first, with the
/// stage trailers git itself parses (root V43).
///
/// # Errors
/// When git cannot read `repo`.
pub fn trailers(repo: &Path) -> Result<Vec<Commit>, String> {
    let format = "--format=%H%x1f%(trailers:key=Paradigm-Stage,valueonly,separator=%x1e)%x1d";
    let out = git(repo, &["log", "--reverse", format])?;
    Ok(out.split(RECORD).filter_map(commit).collect())
}

fn commit(record: &str) -> Option<Commit> {
    let (sha, values) = record.trim_start().split_once(FIELD)?;
    let stages = values.split(VALUE).map(str::trim).filter(|v| !v.is_empty());
    let stages = stages.map(String::from).collect();
    Some(Commit {
        sha: sha.to_string(),
        stages,
    })
}

/// One git run against `repo`, with the caller's `GIT_*` variables dropped
/// so a hook's environment cannot redirect it (root V15).
fn git(repo: &Path, args: &[&str]) -> Result<String, String> {
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(repo).args(args);
    for (key, _) in std::env::vars().filter(|(k, _)| k.starts_with("GIT_")) {
        cmd.env_remove(key);
    }
    let out = cmd.output().map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    String::from_utf8(out.stdout).map_err(|e| e.to_string())
}
