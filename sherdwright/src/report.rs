//! The report: one verdict per root invariant (sherdwright §I json).

/// What the report says about one invariant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// The member keeps it.
    Hold,
    /// An `exempt` row with a reason waives it (root V38).
    Waived,
    /// The member breaks it.
    Fail,
    /// No runner checks it yet (root V41).
    Review,
    /// Newer than the member's pin (root V40).
    Newer,
}

impl Verdict {
    /// The name the json and human reports print.
    #[must_use]
    pub fn name(self) -> &'static str {
        ""
    }
}

/// One invariant's line in the report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// `paradigm/V<n>`.
    pub id: String,
    /// The stage the invariant belongs to.
    pub stage: String,
    /// What the report says.
    pub verdict: Verdict,
    /// Why, in one line.
    pub reason: String,
    /// A commit or `file:line`.
    pub evidence: String,
}

/// Every finding for one member, in invariant order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Report {
    /// The findings, as the checks produced them.
    pub findings: Vec<Finding>,
}

impl Report {
    /// How many invariants no runner checks yet (sherdwright V4).
    #[must_use]
    pub fn review(&self) -> usize {
        0
    }

    /// 0 when nothing fails, 1 when anything does (sherdwright §I).
    #[must_use]
    pub fn exit_code(&self) -> u8 {
        0
    }

    /// The report for a terminal.
    #[must_use]
    pub fn human(&self) -> String {
        String::new()
    }

    /// The report as one JSON object and a newline.
    #[must_use]
    pub fn json(&self) -> String {
        String::new()
    }
}
