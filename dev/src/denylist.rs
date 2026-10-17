//! The local, uncommitted list of private names (`.private/denylist`).

/// Private names, one per line, compared case-insensitively.
#[derive(Debug, Default)]
pub struct Denylist {
    entries: Vec<String>,
}

impl Denylist {
    /// Parses one name per line; blank lines and `#` comments are skipped.
    #[must_use]
    pub fn parse(text: &str) -> Self {
        let entries = text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(str::to_lowercase)
            .collect();
        Self { entries }
    }

    /// How many names the list holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the list holds no names.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The entry number (from 1) of the first name found in `text`. A
    /// number, never the name, so output cannot leak it (dev V2).
    #[must_use]
    pub fn hit(&self, text: &str) -> Option<usize> {
        let lower = text.to_lowercase();
        let found = self.entries.iter().position(|e| lower.contains(e.as_str()));
        found.map(|index| index + 1)
    }
}
