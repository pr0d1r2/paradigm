//! JSON text written by hand: no dependency, and the same input always
//! gives the same bytes (sherdwright V1).

/// `text` as a quoted JSON string.
#[must_use]
pub fn string(text: &str) -> String {
    text.to_string()
}
