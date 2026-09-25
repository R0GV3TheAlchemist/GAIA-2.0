//! Attempt history loading and sorting.

use std::{
    path::{Path, PathBuf},
    collections::BTreeMap,
};
use crate::{
    result::ValidationResult,
    error::ValidateError,
};

/// A sorted collection of [`ValidationResult`] entries, one per attempt.
#[derive(Debug, Clone)]
pub struct AttemptHistory {
    /// Entries sorted ascending by attempt number.
    pub entries: Vec<ValidationResult>,
}

impl AttemptHistory {
    /// Returns the most recent attempt result, or `None` if empty.
    pub fn latest(&self) -> Option<&ValidationResult> {
        self.entries.last()
    }

    /// Returns `true` if the last two attempts share identical fingerprints,
    /// indicating the loop has made no progress.
    pub fn is_stuck(&self) -> bool {
        if self.entries.len() < 2 {
            return false;
        }
        let last = &self.entries[self.entries.len() - 1];
        let prev = &self.entries[self.entries.len() - 2];
        let last_fps: std::collections::HashSet<&str> = last.fingerprints().into_iter().collect();
        let prev_fps: std::collections::HashSet<&str> = prev.fingerprints().into_iter().collect();
        !last_fps.is_empty() && last_fps == prev_fps
    }

    /// Returns the total number of attempts recorded.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if there are no attempt records.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Scan `dir` for attempt result files and return a sorted [`AttemptHistory`].
///
/// Recognised filenames:
///   - `agent-validation.json`   → attempt 1 (fallback when no numbered files exist)
///   - `agent-validation-N.json` → attempt N (N ≥ 1)
pub fn load(dir: &Path) -> Result<AttemptHistory, ValidateError> {
    let read = std::fs::read_dir(dir)
        .map_err(|e| ValidateError::HistoryDir { path: dir.to_path_buf(), source: e })?;

    let mut by_attempt: BTreeMap<u32, ValidationResult> = BTreeMap::new();
    let mut unnumbered: Option<ValidationResult> = None;

    for entry in read.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        let path: PathBuf = entry.path();

        if name_str == "agent-validation.json" {
            if let Ok(r) = parse_file(&path) {
                unnumbered = Some(r);
            }
        } else if let Some(n) = parse_numbered_name(&name_str) {
            if let Ok(r) = parse_file(&path) {
                by_attempt.insert(n, r);
            }
        }
    }

    // Use unnumbered file as attempt 1 only when no numbered files were found
    if by_attempt.is_empty() {
        if let Some(r) = unnumbered {
            by_attempt.insert(r.attempt.max(1), r);
        }
    }

    Ok(AttemptHistory {
        entries: by_attempt.into_values().collect(),
    })
}

fn parse_file(path: &Path) -> Result<ValidationResult, ()> {
    let raw = std::fs::read_to_string(path).map_err(|_| ())?;
    serde_json::from_str(&raw).map_err(|_| ())
}

/// Parse `"agent-validation-N.json"` → `Some(N)`, else `None`.
fn parse_numbered_name(name: &str) -> Option<u32> {
    let stem = name.strip_prefix("agent-validation-")?.strip_suffix(".json")?;
    stem.parse::<u32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbered_name_happy() {
        assert_eq!(parse_numbered_name("agent-validation-1.json"), Some(1));
        assert_eq!(parse_numbered_name("agent-validation-3.json"), Some(3));
    }

    #[test]
    fn parse_numbered_name_rejects_bad() {
        assert!(parse_numbered_name("agent-validation.json").is_none());
        assert!(parse_numbered_name("other.json").is_none());
        assert!(parse_numbered_name("agent-validation-abc.json").is_none());
    }
}
