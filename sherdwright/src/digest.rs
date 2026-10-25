//! The digest: what changed in the specs' §V, §I and §F between two
//! reconciliations, on one page (sherdwright T9). The human signs off on
//! it and a green report instead of reading the code diff (root §G).
//!
//! Specs are read as committed: the gate runs `mth fmt --check` on every
//! spec commit (root V9), so a committed spec is already in `mth fmt` form
//! and the same two reconciliations always give the same digest.

use crate::history::{self, Commit};
use crate::json::string;
use crate::stage::Stage;
use crate::text::visible;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::Path;

/// The sections a digest reads, in the order it prints them.
pub const SECTIONS: [&str; 3] = ["§V", "§I", "§F"];

/// One line added to or removed from one section of one spec.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    /// The spec's path in the tree, `SPEC.md` for the root.
    pub path: String,
    /// One of [`SECTIONS`].
    pub section: &'static str,
    /// Added in `<b>`, or removed from `<a>`.
    pub added: bool,
    /// The line as the spec holds it.
    pub line: String,
}

/// The lines of each section in [`SECTIONS`] that `text` holds, in order,
/// headings and blank lines left out.
#[must_use]
pub fn sections(text: &str) -> Vec<(&'static str, String)> {
    let mut current = None;
    let mut found = Vec::new();
    for line in text.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            current = SECTIONS.into_iter().find(|s| names(heading, s));
        } else if let Some(section) = current.filter(|_| !line.trim().is_empty()) {
            found.push((section, line.to_string()));
        }
    }
    found
}

/// Whether `heading` is the one for `section`: its name, then a space
/// or nothing, so `§V2 NOTES` is not `§V`.
fn names(heading: &str, section: &str) -> bool {
    let rest = heading.strip_prefix(section);
    rest.is_some_and(|r| r.is_empty() || r.starts_with(' '))
}

/// What changed in one spec between `old` and `new`: per section, the
/// removed lines, then the added ones, each in file order.
#[must_use]
pub fn delta(path: &str, old: &str, new: &str) -> Vec<Change> {
    let (old, new) = (sections(old), sections(new));
    let per = |section| moved(path, section, &of(&old, section), &of(&new, section));
    SECTIONS.into_iter().flat_map(per).collect()
}

/// One section's removed lines, then its added ones. Each copy of a
/// repeated line counts on its own.
fn moved(path: &str, section: &'static str, before: &[&str], after: &[&str]) -> Vec<Change> {
    let change = |added, line: &str| Change {
        path: path.to_string(),
        section,
        added,
        line: line.to_string(),
    };
    let gone = unmatched(before, after)
        .into_iter()
        .map(|l| change(false, l));
    let came = unmatched(after, before)
        .into_iter()
        .map(|l| change(true, l));
    gone.chain(came).collect()
}

/// The lines of `kept`, in order, left once each line of `against` has
/// cancelled one equal copy.
fn unmatched<'a>(kept: &[&'a str], against: &[&str]) -> Vec<&'a str> {
    let mut left: BTreeMap<&str, usize> = BTreeMap::new();
    for line in against {
        *left.entry(line).or_default() += 1;
    }
    let mut unmatched = Vec::new();
    for line in kept {
        match left.get_mut(line) {
            Some(n) if *n > 0 => *n -= 1,
            _ => unmatched.push(*line),
        }
    }
    unmatched
}

/// The lines of `all` that sit in `section`.
fn of<'a>(all: &'a [(&str, String)], section: &str) -> Vec<&'a str> {
    let mine = all.iter().filter(|(s, _)| *s == section);
    mine.map(|(_, l)| l.as_str()).collect()
}

/// Every change between the last commit carrying `reconcile-<a>` and the
/// last carrying `reconcile-<b>`, specs in path order.
///
/// # Errors
/// When git cannot read `repo`, or no commit carries one of the stages.
pub fn digest(repo: &Path, a: u32, b: u32) -> Result<Vec<Change>, String> {
    let commits = history::trailers(repo)?;
    let (from, to) = (last(&commits, a)?, last(&commits, b)?);
    let (old, new) = (history::specs(repo, from)?, history::specs(repo, to)?);
    let paths: BTreeSet<&String> = old.iter().chain(&new).collect();
    let mut changes = Vec::new();
    for path in paths {
        let before = text(repo, from, path, &old)?;
        let after = text(repo, to, path, &new)?;
        changes.extend(delta(path, &before, &after));
    }
    Ok(changes)
}

/// The hash of the last commit carrying `reconcile-<n>`.
fn last(commits: &[Commit], n: u32) -> Result<&str, String> {
    let stage = Stage::Reconcile(n).to_string();
    let carries = |c: &&Commit| c.stages.contains(&stage);
    let commit = commits.iter().rev().find(carries);
    commit
        .map(|c| c.sha.as_str())
        .ok_or_else(|| format!("no commit carries `Paradigm-Stage: {stage}`"))
}

/// `path` at `sha`, or nothing when that commit holds no such spec.
fn text(repo: &Path, sha: &str, path: &str, held: &[String]) -> Result<String, String> {
    if held.iter().any(|p| p == path) {
        history::show(repo, sha, path)
    } else {
        Ok(String::new())
    }
}

/// The digest for a terminal: a heading per spec section, then its lines.
#[must_use]
pub fn human(a: u32, b: u32, changes: &[Change]) -> String {
    let mut out = format!("{}..{}\n\n", Stage::Reconcile(a), Stage::Reconcile(b));
    let mut heading = None;
    for c in changes {
        if heading != Some((&c.path, c.section)) {
            let gap = if heading.is_some() { "\n" } else { "" };
            let _ = writeln!(out, "{gap}{} {}", visible(&c.path), c.section);
            heading = Some((&c.path, c.section));
        }
        let _ = writeln!(out, "{} {}", sign(c), visible(&c.line));
    }
    let gap = if changes.is_empty() { "" } else { "\n" };
    let _ = writeln!(out, "{gap}{} changes", changes.len());
    out
}

/// The digest as one JSON object and a newline.
#[must_use]
pub fn json(a: u32, b: u32, changes: &[Change]) -> String {
    let (from, to) = (Stage::Reconcile(a), Stage::Reconcile(b));
    let changes: Vec<String> = changes.iter().map(change_json).collect();
    let changes = changes.join(",");
    format!("{{\"from\":\"{from}\",\"to\":\"{to}\",\"changes\":[{changes}]}}\n")
}

fn change_json(c: &Change) -> String {
    let (path, section, line) = (string(&c.path), string(c.section), string(&c.line));
    let op = string(sign(c));
    format!("{{\"path\":{path},\"section\":{section},\"op\":{op},\"line\":{line}}}")
}

fn sign(c: &Change) -> &'static str {
    if c.added { "+" } else { "-" }
}
