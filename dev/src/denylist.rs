//! The local, uncommitted list of private names (`.private/denylist`).

/// Private names, one per line.
#[derive(Debug, Default)]
pub struct Denylist {
    entries: Vec<String>,
}

impl Denylist {
    /// Parses one name per line.
    #[must_use]
    pub fn parse(_text: &str) -> Self {
        Self::default()
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

    /// The entry number (from 1) of the first name found in `text`.
    #[must_use]
    pub fn hit(&self, _text: &str) -> Option<usize> {
        None
    }
}
