//! Repository references in text: links, flake refs and owner paths.

/// Every `owner/name` referenced in `text`, with its line (from 1).
#[must_use]
pub fn repo_refs(_text: &str) -> Vec<(usize, String)> {
    Vec::new()
}
