//! Repository references in text: links, flake refs and owner paths.

/// Prefixes after which an `owner/name` follows.
const PREFIXES: [&str; 2] = ["github.com/", "github:"];

/// The fleet's owner, named in paths with no host in front.
const OWNER: &str = "pr0d1r2/";

/// Every `owner/name` referenced in `text`, with its line (from 1).
#[must_use]
pub fn repo_refs(text: &str) -> Vec<(usize, String)> {
    text.lines()
        .enumerate()
        .flat_map(|(index, line)| in_line(line).into_iter().map(move |r| (index + 1, r)))
        .collect()
}

/// References in one line, each counted once even when an owner path sits
/// inside a link.
fn in_line(line: &str) -> Vec<String> {
    let after_prefix = PREFIXES
        .iter()
        .flat_map(|p| line.match_indices(p).map(move |(at, _)| at + p.len()));
    let mut starts: Vec<usize> = after_prefix
        .chain(line.match_indices(OWNER).map(|(at, _)| at))
        .collect();
    starts.sort_unstable();
    starts.dedup();
    starts
        .into_iter()
        .filter_map(|at| line.get(at..).and_then(owner_name))
        .collect()
}

fn owner_name(rest: &str) -> Option<String> {
    let mut parts = rest.splitn(3, '/');
    let owner = word(parts.next()?)?;
    let name = word(parts.next()?)?;
    Some(format!("{owner}/{name}"))
}

/// The leading run of name characters, without a trailing `.` or `.git`.
fn word(text: &str) -> Option<&str> {
    let allowed = |c: char| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-');
    let end = text.find(|c: char| !allowed(c)).unwrap_or(text.len());
    let run = text.get(..end)?.trim_end_matches('.');
    let run = run.strip_suffix(".git").unwrap_or(run);
    (!run.is_empty()).then_some(run)
}
