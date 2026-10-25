use sherdwright::json::string;

#[test]
fn quotes_plain_text() {
    assert_eq!(string("V38 waived"), "\"V38 waived\"");
}

#[test]
fn escapes_quotes_and_backslashes() {
    assert_eq!(string(r#"a "b" \c"#), r#""a \"b\" \\c""#);
}

#[test]
fn escapes_control_characters() {
    assert_eq!(string("a\nb\tc\rd"), r#""a\nb\tc\rd""#);
    assert_eq!(string("\u{1f}"), r#""\u001f""#);
}

#[test]
fn keeps_unicode_as_it_is() {
    assert_eq!(string("§V ⊥ ∀"), "\"§V ⊥ ∀\"");
}
