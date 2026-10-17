use std::path::Path;
use std::process::Command;

fn dev(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_dev"))
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap_or_else(|e| unreachable!("dev did not start: {e}"))
}

fn tree(name: &str, files: &[(&str, &str)]) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("paradigm-dev-{name}-{}", std::process::id()));
    for (path, text) in files {
        let file = dir.join(path);
        let parent = file.parent().unwrap_or(&dir);
        std::fs::create_dir_all(parent).unwrap_or_default();
        std::fs::write(file, text).unwrap_or_default();
    }
    dir
}

const ALLOW: (&str, &str) = (
    "dev/public-repos.toml",
    "[[repo]]\nname = \"pr0d1r2/pklith\"\npublic = \"2026-10-16\"\n",
);

#[test]
fn names_exits_zero_on_clean_files() {
    let dir = tree("clean", &[ALLOW, ("a.md", "pr0d1r2/pklith\n")]);
    let out = dev(&dir, &["names", "a.md"]);
    assert_eq!(out.status.code(), Some(0));
}

#[test]
fn names_exits_one_and_prints_findings() {
    let dir = tree("dirty", &[ALLOW, ("a.md", concat!("pr0d1r2", "/other\n"))]);
    let out = dev(&dir, &["names", "a.md"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stdout).contains("a.md:1:"));
}

#[test]
fn names_says_when_the_denylist_is_absent() {
    let dir = tree("nodeny", &[ALLOW, ("a.md", "fine\n")]);
    let out = dev(&dir, &["names", "a.md"]);
    assert!(String::from_utf8_lossy(&out.stderr).contains("no .private/denylist"));
}

#[test]
fn names_uses_the_denylist_when_present() {
    let dir = tree(
        "deny",
        &[
            ALLOW,
            (".private/denylist", "zzhidden\n"),
            ("a.md", "ZZHIDDEN\n"),
        ],
    );
    let out = dev(&dir, &["names", "a.md"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        !String::from_utf8_lossy(&out.stdout)
            .to_lowercase()
            .contains("zzhidden")
    );
}

#[test]
fn names_exits_two_without_an_allowlist() {
    let dir = tree("noallow", &[("a.md", "x\n")]);
    let out = dev(&dir, &["names", "a.md"]);
    assert_eq!(out.status.code(), Some(2));
}

fn message(dir: &Path, text: &str) -> std::process::Output {
    std::fs::write(dir.join("MSG"), text).unwrap_or_default();
    dev(dir, &["commit-msg", "MSG"])
}

#[test]
fn commit_msg_reads_the_trailer_the_way_git_does() {
    let dir = tree("msg-ok", &[]);
    std::fs::create_dir_all(&dir).unwrap_or_default();
    let ok = "ci(x): y (T4)\n\nWhy.\n\nParadigm-Stage: gate\nCo-Authored-By: a <b@c>\n";
    assert_eq!(message(&dir, ok).status.code(), Some(0));
}

#[test]
fn commit_msg_refuses_a_stage_line_git_does_not_parse() {
    let dir = tree("msg-split", &[]);
    std::fs::create_dir_all(&dir).unwrap_or_default();
    let split = "ci(x): y (T4)\n\nWhy.\n\nParadigm-Stage: gate\n\nCo-Authored-By: a <b@c>\n";
    let out = message(&dir, split);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("Paradigm-Stage"));
}
