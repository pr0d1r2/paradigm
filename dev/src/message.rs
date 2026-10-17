//! The commit-message rules (dev §I `dev commit-msg`): a stage trailer git
//! can parse (root V43) and a cited spec id (root V27).

use sherdwright::stage::Stage;

/// git's scissors line: `git commit -v` appends the diff below it.
const SCISSORS: &str = "# ------------------------ >8 ------------------------";

/// Subjects git writes itself; they carry no stage or id.
const GIT_SUBJECTS: [&str; 4] = ["Merge ", "Revert ", "fixup! ", "squash! "];

const KEY: &str = "Paradigm-Stage:";

/// The stage named in `trailers`, the output of
/// `git interpret-trailers --parse`.
///
/// # Errors
/// Says what is missing or wrong.
pub fn stage(trailers: &str) -> Result<String, String> {
    let line = trailers.lines().find_map(|l| l.strip_prefix(KEY));
    let Some(name) = line.map(str::trim) else {
        return Err(format!(
            "no `{KEY}` trailer git can parse: put it in the final paragraph, directly above \
             Co-Authored-By, with no blank line between (root V43)"
        ));
    };
    let stage = name.parse::<Stage>().map_err(|e| format!("`{KEY}` {e}"))?;
    Ok(stage.to_string())
}

/// Whether `text` cites a `T`, `V` or `B` id, bare or node-qualified.
#[must_use]
pub fn cites(text: &str) -> bool {
    text.split(|c: char| !c.is_ascii_alphanumeric()).any(is_id)
}

fn is_id(word: &str) -> bool {
    let mut chars = word.chars();
    let letter = chars.next().is_some_and(|c| matches!(c, 'T' | 'V' | 'B'));
    let rest = chars.as_str();
    let digits = rest.trim_end_matches(|c: char| c.is_ascii_lowercase());
    letter && !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit())
}

/// Both rules over a message and its parsed trailers.
///
/// # Errors
/// The rule broken, with the fix.
pub fn check(text: &str, trailers: &str) -> Result<(), String> {
    let kept = kept(text);
    let subject = kept.lines().next().unwrap_or_default();
    if GIT_SUBJECTS.iter().any(|p| subject.starts_with(p)) {
        return Ok(());
    }
    stage(trailers)?;
    if cites(&kept) {
        Ok(())
    } else {
        Err("cite the T, V or B id this commit serves, in the subject or body (root V27)".into())
    }
}

/// The lines git keeps: up to the scissors, `#` comment lines dropped.
fn kept(text: &str) -> String {
    let lines = text.lines().take_while(|l| *l != SCISSORS);
    let kept: Vec<&str> = lines.filter(|l| !l.starts_with('#')).collect();
    kept.join("\n")
}
