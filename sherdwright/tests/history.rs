use sherdwright::history::{Commit, trailers};
use std::path::{Path, PathBuf};
use std::process::Command;

fn git(dir: &Path, args: &[&str]) {
    let mut cmd = Command::new("git");
    cmd.args(args).current_dir(dir);
    for (key, _) in std::env::vars().filter(|(k, _)| k.starts_with("GIT_")) {
        cmd.env_remove(key);
    }
    let ok = cmd.output().is_ok_and(|o| o.status.success());
    assert!(ok, "git {args:?}");
}

fn repo(name: &str, messages: &[&str]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sherdwright-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap_or_default();
    git(&dir, &["init", "-q", "-b", "main"]);
    git(&dir, &["config", "user.email", "t@example.com"]);
    git(&dir, &["config", "user.name", "t"]);
    for message in messages {
        git(&dir, &["commit", "-q", "--allow-empty", "-m", message]);
    }
    dir
}

fn stages_of(dir: &Path) -> Vec<Vec<String>> {
    let commits = trailers(dir).unwrap_or_default();
    commits.into_iter().map(|c| c.stages).collect()
}

#[test]
fn reads_stage_trailers_oldest_first() {
    let first = "a (T1)\n\nParadigm-Stage: intent";
    let dir = repo("order", &[first, "b (T2)\n\nParadigm-Stage: spec-run"]);
    let expected = [vec!["intent".to_string()], vec!["spec-run".to_string()]];
    assert_eq!(stages_of(&dir), expected);
}

#[test]
fn sees_no_stage_where_git_parses_none() {
    let split = "c (T3)\n\nParadigm-Stage: gate\n\nCo-Authored-By: a <b@c>";
    let dir = repo("split", &[split]);
    let commits: Vec<Commit> = trailers(&dir).unwrap_or_default();
    assert_eq!(commits.len(), 1);
    assert!(commits.iter().all(|c| c.stages.is_empty()));
}

#[test]
fn names_each_commit_by_its_full_hash() {
    let dir = repo("hash", &["d (T4)\n\nParadigm-Stage: gate"]);
    let commits = trailers(&dir).unwrap_or_default();
    assert!(commits.iter().all(|c| c.sha.len() == 40));
}

#[test]
fn fails_outside_a_repository() {
    let dir = std::env::temp_dir().join(format!("sherdwright-none-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap_or_default();
    assert!(trailers(&dir).is_err());
}
