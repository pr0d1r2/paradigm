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

/// A repository whose root spec changes V1 between `reconcile-1` and
/// `reconcile-2`.
fn reconciled(name: &str) -> PathBuf {
    let dir = repo(name);
    let who = ["-c", "user.email=t@example.com", "-c", "user.name=t"];
    for (round, v) in [(1, "V1: one."), (2, "V1: two.")] {
        let spec = format!("# SPEC\n\n## §V INVARIANTS\n\n{v}\n");
        std::fs::write(dir.join("SPEC.md"), spec).unwrap_or_default();
        git(&dir, &["add", "SPEC.md"]);
        let message = format!("r (T1)\n\nParadigm-Stage: reconcile-{round}");
        let commit = ["commit", "-q", "-m", &message];
        git(&dir, &[who.as_slice(), commit.as_slice()].concat());
    }
    dir
}

#[test]
fn digest_prints_the_change_between_two_reconciliations() {
    let dir = path(&reconciled("digest"));
    let (code, text) = run(&["digest", "1", "2", "--dir", &dir]);
    assert_eq!(code, Some(0));
    assert!(
        text.contains("SPEC.md §V\n- V1: one.\n+ V1: two.\n"),
        "{text}"
    );
    let (_, json) = run(&["digest", "1", "2", "--dir", &dir, "--format", "json"]);
    assert!(json.starts_with("{\"from\":\"reconcile-1\""), "{json}");
}

#[test]
fn digest_exits_2_without_the_reconciliation() {
    let dir = repo("nodigest");
    assert_eq!(run(&["digest", "1", "2", "--dir", &path(&dir)]).0, Some(2));
}
