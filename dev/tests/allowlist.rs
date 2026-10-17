use paradigm_dev::allowlist::Allowlist;

const LIST: &str = r#"# public repositories
self = "pr0d1r2/paradigm"

[[repo]]
name = "pr0d1r2/pklith"
public = "2026-10-16"
"#;

#[test]
fn holds_listed_repos_and_self() {
    let list = Allowlist::parse(LIST).unwrap_or_default();
    assert!(list.allows("pr0d1r2/pklith"));
    assert!(list.allows("pr0d1r2/paradigm"));
    assert!(!list.allows("pr0d1r2/hidden"));
}

#[test]
fn compares_case_insensitively() {
    let list = Allowlist::parse(LIST).unwrap_or_default();
    assert!(list.allows("PR0D1R2/PKLITH"));
}

#[test]
fn refuses_a_row_without_a_public_date() {
    let text = "[[repo]]\nname = \"pr0d1r2/pklith\"\n";
    assert!(Allowlist::parse(text).is_err());
}

#[test]
fn refuses_a_date_of_the_wrong_shape() {
    let text = "[[repo]]\nname = \"a/b\"\npublic = \"16.10.2026\"\n";
    assert!(Allowlist::parse(text).is_err());
}

#[test]
fn refuses_an_unknown_key() {
    assert!(Allowlist::parse("owner = \"x\"\n").is_err());
}
