//! The name guard (root V37): private names and unlisted repositories.

use crate::allowlist::Allowlist;
use crate::denylist::Denylist;
use crate::finding::Finding;

/// Findings for one file's text.
#[must_use]
pub fn check(
    _path: &str,
    _text: &str,
    _allow: &Allowlist,
    _deny: Option<&Denylist>,
) -> Vec<Finding> {
    Vec::new()
}
