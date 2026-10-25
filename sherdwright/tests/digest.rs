use sherdwright::digest::{Change, delta, digest, human, json, sections};
use std::path::{Path, PathBuf};
use std::process::Command;

const OLD: &str = "# SPEC\n\n## §G GOAL\n\ngoal\n\n## §F FEDERATION\n\ndir|owns\na|x\n\n\
## §I INTERFACES\n\n- cmd: `a`\n\n## §V INVARIANTS\n\nV1: one.\nV2: two.\n\n## §T TASKS\n\nT1|.|t|-\n";

const NEW: &str = "# SPEC\n\n## §G GOAL\n\nnew goal\n\n## §F FEDERATION\n\ndir|owns\na|x\n\n\
## §I INTERFACES\n\n- cmd: `a`\n- cmd: `b`\n\n## §V INVARIANTS\n\nV1: one.\nV2: two, sharper.\n\n\
## §T TASKS\n\nT1|x|t|-\n";

fn change(path: &str, section: &'static str, added: bool, line: &str) -> Change {
    let (path, line) = (path.to_string(), line.to_string());
    Change {
        path,
        section,
        added,
        line,
    }
}

#[test]
fn reads_only_the_signed_off_sections() {
    let found = sections(OLD);
    let names: Vec<&str> = found.iter().map(|(s, _)| *s).collect();
    assert_eq!(names, ["§F", "§F", "§I", "§V", "§V"]);
    assert_eq!(found.last().map(|(_, l)| l.as_str()), Some("V2: two."));
}

#[test]
fn lists_removed_then_added_lines_in_section_order() {
    let expected = [
        change("SPEC.md", "§V", false, "V2: two."),
        change("SPEC.md", "§V", true, "V2: two, sharper."),
        change("SPEC.md", "§I", true, "- cmd: `b`"),
    ];
    assert_eq!(delta("SPEC.md", OLD, NEW), expected);
}

#[test]
fn ignores_goal_and_task_changes() {
    let only = OLD.replace("goal", "other").replace("T1|.|", "T1|x|");
    assert!(delta("SPEC.md", OLD, &only).is_empty());
}

#[test]
fn a_new_spec_adds_every_line_and_a_gone_one_removes_them() {
    assert_eq!(delta("a/SPEC.md", "", OLD).len(), 5);
    assert!(delta("a/SPEC.md", "", OLD).iter().all(|c| c.added));
    assert!(delta("a/SPEC.md", OLD, "").iter().all(|c| !c.added));
}

#[test]
fn prints_a_heading_per_spec_section() {
    let text = human(1, 2, &delta("SPEC.md", OLD, NEW));
    let expected = "reconcile-1..reconcile-2\n\nSPEC.md §V\n- V2: two.\n+ V2: two, sharper.\n\n\
SPEC.md §I\n+ - cmd: `b`\n\n3 changes\n";
    assert_eq!(text, expected);
}

#[test]
fn says_so_when_nothing_changed() {
    assert_eq!(human(1, 2, &[]), "reconcile-1..reconcile-2\n\n0 changes\n");
}

#[test]
fn writes_json() {
    let changes = [change("SPEC.md", "§V", true, "V3: \"q\".")];
    let expected = concat!(
        r#"{"from":"reconcile-1","to":"reconcile-2","changes":[{"path":"SPEC.md","#,
        r#""section":"§V","op":"+","line":"V3: \"q\"."}]}"#,
        "\n"
    );
    assert_eq!(json(1, 2, &changes), expected);
    let empty = "{\"from\":\"reconcile-1\",\"to\":\"reconcile-2\",\"changes\":[]}\n";
    assert_eq!(json(1, 2, &[]), empty);
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

/// Writes `files`, then commits them with `stage` as the trailer.
fn commit(dir: &Path, files: &[(&str, &str)], stage: &str) {
    for (path, text) in files {
        let file = dir.join(path);
        std::fs::create_dir_all(file.parent().unwrap_or(dir)).unwrap_or_default();
        std::fs::write(file, text).unwrap_or_default();
    }
    git(dir, &["add", "-A"]);
    let who = ["-c", "user.email=t@example.com", "-c", "user.name=t"];
    let message = format!("c (T1)\n\nParadigm-Stage: {stage}");
    let args = ["commit", "-q", "--allow-empty", "-m", &message];
    git(dir, &[who.as_slice(), args.as_slice()].concat());
}

fn repo(name: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("sherdwright-digest-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).unwrap_or_default();
    std::fs::create_dir_all(&dir).unwrap_or_default();
    git(&dir, &["init", "-q", "-b", "main"]);
    dir
}

#[test]
fn compares_the_last_commit_of_each_reconciliation() {
    let dir = repo("last");
    commit(&dir, &[("SPEC.md", OLD)], "reconcile-1");
    commit(
        &dir,
        &[("SPEC.md", &OLD.replace("V1: one.", "V1: first."))],
        "reconcile-1",
    );
    commit(&dir, &[("SPEC.md", NEW), ("node/SPEC.md", OLD)], "wave-2");
    commit(&dir, &[("SPEC.md", NEW)], "reconcile-2");
    commit(&dir, &[(".coverage", "lines 99\n")], "reconcile-2");
    let changes = digest(&dir, 1, 2).unwrap_or_default();
    let paths: Vec<&str> = changes.iter().map(|c| c.path.as_str()).collect();
    assert_eq!(paths.iter().filter(|p| **p == "node/SPEC.md").count(), 5);
    assert_eq!(paths.first(), Some(&"SPEC.md"));
    assert!(changes.contains(&change("SPEC.md", "§V", false, "V1: first.")));
}

#[test]
fn refuses_a_reconciliation_no_commit_carries() {
    let dir = repo("missing");
    commit(&dir, &[("SPEC.md", OLD)], "reconcile-1");
    let err = digest(&dir, 1, 3).err().unwrap_or_default();
    assert!(err.contains("reconcile-3"), "{err}");
}

#[test]
fn fails_outside_a_repository() {
    let dir = std::env::temp_dir().join(format!("sherdwright-digest-none-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap_or_default();
    assert!(!inside_a_repository(&dir), "{}", dir.display());
    assert!(digest(&dir, 1, 2).is_err());
}

#[test]
fn counts_each_copy_of_a_repeated_line() {
    let twice = OLD.replace("- cmd: `a`\n", "- cmd: `a`\n- cmd: `a`\n");
    let expected = [change("SPEC.md", "§I", false, "- cmd: `a`")];
    assert_eq!(delta("SPEC.md", &twice, OLD), expected);
    let back = [change("SPEC.md", "§I", true, "- cmd: `a`")];
    assert_eq!(delta("SPEC.md", OLD, &twice), back);
}

/// Whether git finds a repository at or above `dir`.
fn inside_a_repository(dir: &Path) -> bool {
    let mut probe = Command::new("git");
    probe.arg("-C").arg(dir).args(["rev-parse", "--git-dir"]);
    for (key, _) in std::env::vars().filter(|(k, _)| k.starts_with("GIT_")) {
        probe.env_remove(key);
    }
    probe.output().is_ok_and(|o| o.status.success())
}

#[test]
fn reads_a_section_only_under_its_own_heading() {
    let notes = OLD.replace(
        "## §T TASKS",
        "## §V2 NOTES\n\nnot an invariant\n\n## §T TASKS",
    );
    assert!(delta("SPEC.md", OLD, &notes).is_empty());
    assert_eq!(sections("## §V\nV1: bare heading.\n").len(), 1);
}

#[test]
fn human_digest_escapes_terminal_controls() {
    let changes = [change(
        "a\u{1b}/SPEC.md",
        "§V",
        true,
        "V9: hi\u{1b}[1A\u{1b}[2K",
    )];
    let text = human(1, 2, &changes);
    assert!(!text.contains('\u{1b}'), "{text:?}");
    assert!(text.contains("+ V9: hi\\u{1b}[1A\\u{1b}[2K\n"), "{text:?}");
    assert!(text.contains("a\\u{1b}/SPEC.md §V\n"), "{text:?}");
}
