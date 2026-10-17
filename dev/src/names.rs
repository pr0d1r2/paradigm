//! The name guard (root V37): private names and unlisted repositories.

use crate::allowlist::Allowlist;
use crate::denylist::Denylist;
use crate::finding::Finding;
use crate::refs::repo_refs;

/// Findings for one file's text. A line holding a private name reports
/// only that, so no other finding can echo the name (dev V2).
#[must_use]
pub fn check(path: &str, text: &str, allow: &Allowlist, deny: Option<&Denylist>) -> Vec<Finding> {
    let denied = deny
        .map(|deny| denied_lines(text, deny))
        .unwrap_or_default();
    let private = denied
        .iter()
        .map(|&(line, entry)| private(path, line, entry));
    let unknown = unlisted(text, allow, &denied).map(|(line, repo)| unknown(path, line, &repo));
    private.chain(unknown).collect()
}

fn private(path: &str, line: usize, entry: usize) -> Finding {
    Finding::new(
        path,
        line,
        &format!("private name (denylist entry {entry})"),
    )
}

fn unknown(path: &str, line: usize, repo: &str) -> Finding {
    let what = format!("repository `{repo}` is not in dev/public-repos.toml");
    Finding::new(path, line, &what)
}

/// `(line, entry)` for each line holding a denied name.
fn denied_lines(text: &str, deny: &Denylist) -> Vec<(usize, usize)> {
    let hit = |(index, line): (usize, &str)| deny.hit(line).map(|entry| (index + 1, entry));
    text.lines().enumerate().filter_map(hit).collect()
}

/// References the allowlist lacks, on lines no private name holds.
fn unlisted<'a>(
    text: &str,
    allow: &'a Allowlist,
    denied: &'a [(usize, usize)],
) -> impl Iterator<Item = (usize, String)> + 'a {
    let quiet = move |line: &usize| !denied.iter().any(|(d, _)| d == line);
    repo_refs(text)
        .into_iter()
        .filter(move |(line, repo)| !allow.allows(repo) && quiet(line))
}
