use std::path::{Path, PathBuf};
use std::process::Command;

fn run(args: &[&str]) -> (Option<i32>, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_sherdwright"))
        .args(args)
        .output();
    out.map(|o| {
        (
            o.status.code(),
            String::from_utf8_lossy(&o.stdout).into_owned(),
        )
    })
    .unwrap_or_default()
}

fn repo(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sherdwright-main-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).unwrap_or_default();
    std::fs::create_dir_all(&dir).unwrap_or_default();
    git(&dir, &["init", "-q", "-b", "main"]);
    let who = ["-c", "user.email=t@example.com", "-c", "user.name=t"];
    let commit = ["commit", "-q", "--allow-empty", "-m", "a (T1)"];
    git(&dir, &[who.as_slice(), commit.as_slice()].concat());
    dir
}

fn git(dir: &Path, args: &[&str]) {
    let mut cmd = Command::new("git");
    cmd.args(args).current_dir(dir);
    for (key, _) in std::env::vars().filter(|(k, _)| k.starts_with("GIT_")) {
        cmd.env_remove(key);
    }
    let ok = cmd.output().is_ok_and(|o| o.status.success());
    assert!(ok, "git {args:?}");
}

fn path(dir: &Path) -> String {
    dir.to_string_lossy().into_owned()
}

#[test]
fn prints_its_version() {
    let (_, text) = run(&["--version"]);
    assert!(text.starts_with("sherdwright "));
}

#[test]
fn check_reports_a_member_with_no_check_run() {
    let dir = repo("human");
    let (code, text) = run(&["check", &path(&dir)]);
    assert_eq!(code, Some(0));
    assert_eq!(text, "0 invariants · 0 fail · 0 review\n");
}

#[test]
fn check_writes_json_on_request() {
    let dir = repo("json");
    let (code, text) = run(&["check", &path(&dir), "--format", "json"]);
    assert_eq!(code, Some(0));
    assert_eq!(text, "{\"findings\":[],\"review\":0}\n");
}

#[test]
fn check_exits_2_outside_a_repository() {
    let dir = std::env::temp_dir().join(format!("sherdwright-main-none-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap_or_default();
    assert_eq!(run(&["check", &path(&dir)]).0, Some(2));
}

#[test]
fn exits_2_on_a_usage_error() {
    assert_eq!(run(&["check", "--format", "xml"]).0, Some(2));
    assert_eq!(run(&["no-such-verb"]).0, Some(2));
}
