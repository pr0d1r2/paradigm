//! The report: one verdict per root invariant (sherdwright §I json).

use crate::json::string;
use crate::text::visible;
use std::fmt::Write;

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
        match self {
            Self::Hold => "hold",
            Self::Waived => "waived",
            Self::Fail => "fail",
            Self::Review => "review",
            Self::Newer => "newer",
        }
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
        self.count(Verdict::Review)
    }

    /// 0 when nothing fails, 1 when anything does (sherdwright §I).
    #[must_use]
    pub fn exit_code(&self) -> u8 {
        u8::from(self.count(Verdict::Fail) > 0)
    }

    /// The report for a terminal.
    #[must_use]
    pub fn human(&self) -> String {
        let lines = self.findings.iter().map(Finding::human);
        let mut out: String = lines.map(|l| l + "\n").collect();
        let (n, fail, review) = (
            self.findings.len(),
            self.count(Verdict::Fail),
            self.review(),
        );
        let _ = writeln!(out, "{n} invariants · {fail} fail · {review} review");
        out
    }

    /// The report as one JSON object and a newline.
    #[must_use]
    pub fn json(&self) -> String {
        let findings: Vec<String> = self.findings.iter().map(Finding::json).collect();
        let review = self.review();
        format!(
            "{{\"findings\":[{}],\"review\":{review}}}\n",
            findings.join(",")
        )
    }

    fn count(&self, verdict: Verdict) -> usize {
        self.findings
            .iter()
            .filter(|f| f.verdict == verdict)
            .count()
    }
}

impl Finding {
    /// One line: the verdict padded to one width, then the invariant.
    fn human(&self) -> String {
        let (id, stage) = (visible(&self.id), visible(&self.stage));
        let (reason, evidence) = (visible(&self.reason), visible(&self.evidence));
        let verdict = self.verdict.name();
        format!("{verdict:<7} {id} ({stage}): {reason} [{evidence}]")
    }

    fn json(&self) -> String {
        let fields = [
            ("id", string(&self.id)),
            ("stage", string(&self.stage)),
            ("verdict", string(self.verdict.name())),
            ("reason", string(&self.reason)),
            ("evidence", string(&self.evidence)),
        ];
        let fields: Vec<String> = fields.iter().map(|(k, v)| format!("\"{k}\":{v}")).collect();
        format!("{{{}}}", fields.join(","))
    }
}
