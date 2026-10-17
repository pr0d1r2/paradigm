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

impl FromStr for Stage {
    type Err = String;

    fn from_str(_name: &str) -> Result<Self, Self::Err> {
        Ok(Self::Intent)
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "intent")
    }
}
