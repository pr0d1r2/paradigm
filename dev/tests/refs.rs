use paradigm_dev::refs::repo_refs;

const MIXED: &str = "see https://github.com/panva/jose.\n\
                     url = \"git+https://github.com/pr0d1r2/nix-hk?ref=main\";\n\
                     nix run github:pr0d1r2/paradigm/v1#seed\n\
                     and pr0d1r2/sherd too";

#[test]
fn finds_links_flake_refs_and_owner_paths() {
    let found = repo_refs(MIXED);
    let names: Vec<&str> = found.iter().map(|(_, n)| n.as_str()).collect();
    assert_eq!(
        names,
        [
            "panva/jose",
            "pr0d1r2/nix-hk",
            "pr0d1r2/paradigm",
            "pr0d1r2/sherd"
        ]
    );
}

#[test]
fn reports_the_line_of_each_reference() {
    let lines: Vec<usize> = repo_refs(MIXED).iter().map(|(l, _)| *l).collect();
    assert_eq!(lines, [1, 2, 3, 4]);
}

#[test]
fn strips_a_git_suffix() {
    let found = repo_refs("git clone https://github.com/pr0d1r2/itok.git");
    assert_eq!(found, [(1, "pr0d1r2/itok".to_string())]);
}

#[test]
fn counts_an_owner_path_once_inside_a_link() {
    assert_eq!(repo_refs("https://github.com/pr0d1r2/itok").len(), 1);
}

#[test]
fn ignores_text_with_no_repository() {
    assert!(repo_refs("plain words / and a slash").is_empty());
}
