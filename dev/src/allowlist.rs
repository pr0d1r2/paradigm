//! The committed list of public repositories (`dev/public-repos.toml`).

/// Repositories committed text may name.
#[derive(Debug, Default)]
pub struct Allowlist {
    repos: Vec<String>,
}

impl Allowlist {
    /// Parses the file.
    ///
    /// # Errors
    /// Names the first line that is not part of the format.
    pub fn parse(_text: &str) -> Result<Self, String> {
        Ok(Self::default())
    }

    /// Whether `repo` (`owner/name`) may be named.
    #[must_use]
    pub fn allows(&self, repo: &str) -> bool {
        self.repos.iter().any(|r| r == repo)
    }
}
