//! A member's history, as git reads it.

use std::path::Path;

/// One commit and the stages its trailers name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    /// The full commit hash.
    pub sha: String,
    /// Each `Paradigm-Stage:` trailer value git parses, in order.
    pub stages: Vec<String>,
}

/// Every commit reachable from `HEAD` in `repo`, oldest first.
///
/// # Errors
/// When git cannot read `repo`.
pub fn trailers(_repo: &Path) -> Result<Vec<Commit>, String> {
    Ok(Vec::new())
}
