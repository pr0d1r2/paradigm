use sherdwright::text::visible;

#[test]
fn keeps_plain_text() {
    assert_eq!(visible("V1: one ⊥ two."), "V1: one ⊥ two.");
}

#[test]
fn writes_control_characters_as_escapes() {
    assert_eq!(visible("a\u{1b}[2Kb"), "a\\u{1b}[2Kb");
    assert_eq!(visible("x\ry\u{7}"), "x\\u{d}y\\u{7}");
}

#[test]
fn writes_bidirectional_formatting_as_escapes() {
    assert_eq!(visible("a\u{202e}b"), "a\\u{202e}b");
    for c in [
        '\u{61c}', '\u{200e}', '\u{200f}', '\u{202a}', '\u{2066}', '\u{2069}',
    ] {
        assert_eq!(visible(&c.to_string()), c.escape_unicode().to_string());
    }
    assert_eq!(visible("שלום"), "שלום");
}
