//! Repository text made safe for a terminal: a spec or `.pklith` row is
//! member data, and an escape sequence in it must not rewrite the page a
//! human signs off on (root §G).

/// `text` with every control and bidirectional formatting character
/// written as `\u{..}`.
#[must_use]
pub fn visible(text: &str) -> String {
    let shown = |c: char| {
        if c.is_control() || bidi(c) {
            c.escape_unicode().to_string()
        } else {
            c.to_string()
        }
    };
    text.chars().map(shown).collect()
}

/// Unicode's bidirectional formatting characters: marks, embeddings,
/// overrides and isolates. They reorder how a line displays, so a spec
/// could show a reviewer a different line than it holds.
fn bidi(c: char) -> bool {
    matches!(c, '\u{61c}' | '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}
