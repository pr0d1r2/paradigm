//! One gate finding: where, and what is wrong there.

use std::fmt;

/// A problem at one line of one file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    path: String,
    line: usize,
    what: String,
}

impl Finding {
    /// A finding at `line` (from 1) of `path`.
    #[must_use]
    pub fn new(path: &str, line: usize, what: &str) -> Self {
        Self {
            path: path.to_string(),
            line,
            what: what.to_string(),
        }
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.path, self.line, self.what)
    }
}
