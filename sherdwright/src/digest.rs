//! The digest: what changed in the specs' §V, §I and §F between two
//! reconciliations, on one page (sherdwright T9). The human signs off on
//! it and a green report instead of reading the code diff (root §G).

use std::path::Path;

/// The sections a digest reads, in the order it prints them.
pub const SECTIONS: [&str; 3] = ["§V", "§I", "§F"];

/// One line added to or removed from one section of one spec.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    /// The spec's path in the tree, `SPEC.md` for the root.
    pub path: String,
    /// One of [`SECTIONS`].
    pub section: &'static str,
    /// Added in `<b>`, or removed from `<a>`.
    pub added: bool,
    /// The line as the spec holds it.
    pub line: String,
}

/// The lines of each section in [`SECTIONS`] that `text` holds, in order,
/// headings and blank lines left out.
#[must_use]
pub fn sections(text: &str) -> Vec<(&'static str, String)> {
    let _ = text;
    Vec::new()
}

/// What changed in one spec between `old` and `new`: per section, the
/// removed lines, then the added ones, each in file order.
#[must_use]
pub fn delta(path: &str, old: &str, new: &str) -> Vec<Change> {
    let _ = (path, old, new);
    Vec::new()
}

/// Every change between the last commit carrying `reconcile-<a>` and the
/// last carrying `reconcile-<b>`, specs in path order.
///
/// # Errors
/// When git cannot read `repo`, or no commit carries one of the stages.
pub fn digest(repo: &Path, a: u32, b: u32) -> Result<Vec<Change>, String> {
    let _ = (repo, a, b);
    Ok(Vec::new())
}

/// The digest for a terminal: a heading per spec section, then its lines.
#[must_use]
pub fn human(a: u32, b: u32, changes: &[Change]) -> String {
    let _ = (a, b, changes);
    String::new()
}

/// The digest as one JSON object and a newline.
#[must_use]
pub fn json(a: u32, b: u32, changes: &[Change]) -> String {
    let _ = (a, b, changes);
    String::new()
}
