//! The committed list of public repositories (`dev/public-repos.toml`).
//!
//! A strict subset of TOML: comments, one top-level `self = "owner/name"`,
//! and `[[repo]]` tables holding `name` and a `public` date. taplo checks
//! the syntax; this module checks the meaning.

/// Repositories committed text may name.
#[derive(Debug, Default)]
pub struct Allowlist {
    own: Option<String>,
    repos: Vec<String>,
}

enum Line {
    Blank,
    Table,
    Pair(String, String),
}

#[derive(Default)]
struct Row {
    name: Option<String>,
    public: Option<String>,
}

impl Allowlist {
    /// Parses the file.
    ///
    /// # Errors
    /// Names the first line, or the row, that breaks the format.
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut list = Self::default();
        let mut row: Option<Row> = None;
        for (index, raw) in text.lines().enumerate() {
            let at = |e: String| format!("line {}: {e}", index + 1);
            match classify(raw).map_err(at)? {
                Line::Blank => {}
                Line::Table => list.close(row.replace(Row::default())).map_err(at)?,
                Line::Pair(key, value) => list.pair(row.as_mut(), &key, value).map_err(at)?,
            }
        }
        list.close(row)?;
        Ok(list)
    }

    /// Whether `repo` (`owner/name`) may be named, ignoring case.
    #[must_use]
    pub fn allows(&self, repo: &str) -> bool {
        let repo = repo.to_lowercase();
        self.own.as_deref() == Some(repo.as_str()) || self.repos.contains(&repo)
    }

    fn pair(&mut self, row: Option<&mut Row>, key: &str, value: String) -> Result<(), String> {
        match (row, key) {
            (None, "self") => self.own = Some(value.to_lowercase()),
            (Some(row), "name") => row.name = Some(value),
            (Some(row), "public") => row.public = Some(value),
            _ => return Err(format!("unknown key `{key}`")),
        }
        Ok(())
    }

    fn close(&mut self, row: Option<Row>) -> Result<(), String> {
        let Some(row) = row else { return Ok(()) };
        let name = row.name.ok_or("a [[repo]] row has no `name`")?;
        let public = row
            .public
            .ok_or_else(|| format!("`{name}` has no `public` date"))?;
        if !is_date(&public) {
            return Err(format!(
                "`{name}`: `public` must be YYYY-MM-DD, not `{public}`"
            ));
        }
        self.repos.push(name.to_lowercase());
        Ok(())
    }
}

fn classify(raw: &str) -> Result<Line, String> {
    let line = raw.trim();
    if line.is_empty() || line.starts_with('#') {
        return Ok(Line::Blank);
    }
    if line == "[[repo]]" {
        return Ok(Line::Table);
    }
    let (key, value) = line.split_once('=').ok_or("expected `key = \"value\"`")?;
    let quoted = value
        .trim()
        .strip_prefix('"')
        .and_then(|v| v.strip_suffix('"'));
    let value = quoted.ok_or("the value must be a quoted string")?;
    Ok(Line::Pair(key.trim().to_string(), value.to_string()))
}

fn is_date(text: &str) -> bool {
    let shape = |(index, c): (usize, char)| match index {
        4 | 7 => c == '-',
        _ => c.is_ascii_digit(),
    };
    text.len() == 10 && text.char_indices().all(shape)
}
