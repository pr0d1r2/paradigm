//! JSON text written by hand: no dependency, and the same input always
//! gives the same bytes (sherdwright V1).

use std::fmt::Write;

/// `text` as a quoted JSON string.
#[must_use]
pub fn string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        escape(&mut out, c);
    }
    out.push('"');
    out
}

/// `c` as JSON writes it inside a string.
fn escape(out: &mut String, c: char) {
    match c {
        '"' => out.push_str("\\\""),
        '\\' => out.push_str("\\\\"),
        '\n' => out.push_str("\\n"),
        '\r' => out.push_str("\\r"),
        '\t' => out.push_str("\\t"),
        c if c.is_control() => {
            let _ = write!(out, "\\u{:04x}", u32::from(c));
        }
        c => out.push(c),
    }
}
