use paradigm_dev::allowlist::Allowlist;
use paradigm_dev::denylist::Denylist;
use paradigm_dev::names::check;

fn allow() -> Allowlist {
    Allowlist::parse("[[repo]]\nname = \"pr0d1r2/pklith\"\npublic = \"2026-10-16\"\n")
        .unwrap_or_default()
}

#[test]
fn passes_a_listed_repository() {
    assert!(check("a.md", "uses pr0d1r2/pklith", &allow(), None).is_empty());
}

#[test]
fn flags_an_unlisted_repository_by_line() {
    let found = check(
        "a.md",
        concat!("ok\nsee pr0d1r2", "/elsewhere"),
        &allow(),
        None,
    );
    let lines: Vec<String> = found.iter().map(ToString::to_string).collect();
    assert_eq!(
        lines,
        [concat!(
            "a.md:2: repository `pr0d1r2",
            "/elsewhere` is not in dev/public-repos.toml"
        )]
    );
}

#[test]
fn flags_a_denied_name_without_echoing_it() {
    let deny = Denylist::parse("hushhush\n");
    let found = check("a.md", "x\n\nthe Hushhush tool", &allow(), Some(&deny));
    let lines: Vec<String> = found.iter().map(ToString::to_string).collect();
    assert_eq!(lines, ["a.md:3: private name (denylist entry 1)"]);
    assert!(!lines.iter().any(|l| l.to_lowercase().contains("hushhush")));
}
