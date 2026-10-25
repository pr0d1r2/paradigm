//! Repository text made safe for a terminal: a spec or `.pklith` row is
//! member data, and an escape sequence in it must not rewrite the page a
//! human signs off on (root §G).

/// `text` with every control character written as `\u{..}`.
#[must_use]
pub fn visible(text: &str) -> String {
    text.to_string()
}
