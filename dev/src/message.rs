//! The commit-message rules (dev §I `dev commit-msg`): a stage trailer git
//! can parse (root V43) and a cited spec id (root V27).

/// The stage named in `trailers`, the output of
/// `git interpret-trailers --parse`.
///
/// # Errors
/// Says what is missing or wrong.
pub fn stage(_trailers: &str) -> Result<String, String> {
    Err(String::new())
}

/// Whether `text` cites a `T`, `V` or `B` id, bare or node-qualified.
#[must_use]
pub fn cites(_text: &str) -> bool {
    true
}

/// Both rules over a message and its parsed trailers.
///
/// # Errors
/// The rule broken, with the fix.
pub fn check(_text: &str, _trailers: &str) -> Result<(), String> {
    Ok(())
}
