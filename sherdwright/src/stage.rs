//! The `Paradigm-Stage:` vocabulary (root §I.trailer).

use std::fmt;
use std::str::FromStr;

/// One lifecycle stage, as a commit trailer names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Intent,
    GapRound(u32),
    SpecRun,
    CkCheckRound(u32),
    DevShell,
    Guardrails,
    Wave(u32),
    Reconcile(u32),
    Gate,
    Release,
    Backprop,
    Propagation,
}

/// Stages that take no round number.
const PLAIN: [(&str, Stage); 8] = [
    ("intent", Stage::Intent),
    ("spec-run", Stage::SpecRun),
    ("dev-shell", Stage::DevShell),
    ("guardrails", Stage::Guardrails),
    ("gate", Stage::Gate),
    ("release", Stage::Release),
    ("backprop", Stage::Backprop),
    ("propagation", Stage::Propagation),
];

/// Builds a numbered stage from its round.
type Make = fn(u32) -> Stage;

/// Stages numbered by round: the prefix, then the round.
const ROUNDS: [(&str, Make); 4] = [
    ("gap-round-", Stage::GapRound),
    ("ck-check-round-", Stage::CkCheckRound),
    ("wave-", Stage::Wave),
    ("reconcile-", Stage::Reconcile),
];

impl FromStr for Stage {
    type Err = String;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        if let Some((_, stage)) = PLAIN.iter().find(|(n, _)| *n == name) {
            return Ok(*stage);
        }
        let numbered = |(prefix, make): &(&str, Make)| round(name, prefix).map(make);
        let stage = ROUNDS.iter().find_map(numbered);
        stage.ok_or_else(|| format!("`{name}` names no stage of root §I.trailer"))
    }
}

/// The round after `prefix`, when the rest of `name` is all digits.
fn round(name: &str, prefix: &str) -> Option<u32> {
    let rest = name.strip_prefix(prefix)?;
    let digits = !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit());
    digits.then(|| rest.parse().ok()).flatten()
}

impl Stage {
    /// The prefix and round of a numbered stage.
    fn numbered(self) -> Option<(&'static str, u32)> {
        match self {
            Self::GapRound(n) => Some(("gap-round-", n)),
            Self::CkCheckRound(n) => Some(("ck-check-round-", n)),
            Self::Wave(n) => Some(("wave-", n)),
            Self::Reconcile(n) => Some(("reconcile-", n)),
            _ => None,
        }
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((prefix, n)) = self.numbered() {
            return write!(f, "{prefix}{n}");
        }
        let name = PLAIN.iter().find(|(_, s)| s == self).map_or("", |(n, _)| n);
        f.write_str(name)
    }
}
