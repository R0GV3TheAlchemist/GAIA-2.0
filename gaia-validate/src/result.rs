//! Typed representation of `agent-validation.json`.

use serde::{Deserialize, Serialize};

/// The structured result written by `scripts/agent-validate.sh`.
///
/// Mirrors the JSON schema defined in `agent-validation.json`.
/// Unknown fields are ignored so that future schema extensions remain
/// backward-compatible.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidationResult {
    /// Schema version (`"1.0"` or `"1.1"`).
    pub schema_version: String,

    /// Git commit SHA this result was produced against.
    pub head_sha: String,

    /// ISO-8601 timestamp of when the script ran.
    pub timestamp: String,

    /// Validation mode: `full`, `changed`, or `targeted`.
    pub mode: String,

    /// Correction loop attempt number (1-indexed).
    #[serde(default = "default_attempt")]
    pub attempt: u32,

    /// Overall status: `passed`, `failed`, or `no_progress`.
    pub status: ValidationStatus,

    /// Name of the first stage that failed, if any.
    #[serde(default)]
    pub failed_stage: Option<String>,

    /// `true` when the failure set is identical to the previous attempt.
    #[serde(default)]
    pub no_progress: bool,

    /// Per-stage results.
    #[serde(default)]
    pub stages: Vec<StageResult>,

    /// Compiler diagnostics extracted from `cargo check --message-format=json`.
    #[serde(default)]
    pub diagnostics: Vec<Diagnostic>,
}

fn default_attempt() -> u32 { 1 }

impl ValidationResult {
    /// Returns `true` if the validation passed cleanly.
    pub fn is_passing(&self) -> bool {
        self.status == ValidationStatus::Passed
    }

    /// Returns `true` if the result is `no_progress`.
    ///
    /// When this is `true` the correction loop **must** escalate to a
    /// human rather than proposing another repair.
    pub fn is_no_progress(&self) -> bool {
        self.status == ValidationStatus::NoProgress || self.no_progress
    }

    /// Returns the set of unique failure fingerprints from this result.
    ///
    /// Useful when constructing the `--fingerprint` argument for the next
    /// attempt: serialise this result to disk and pass the path to
    /// [`crate::validate`].
    pub fn fingerprints(&self) -> Vec<&str> {
        self.diagnostics
            .iter()
            .filter_map(|d| d.fingerprint.as_deref())
            .collect()
    }
}

/// Overall validation status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Passed,
    Failed,
    NoProgress,
    /// Catch-all for forward-compatible unknown values.
    #[serde(other)]
    Unknown,
}

/// Result of a single validation stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StageResult {
    pub name: String,
    pub status: String,
    pub exit_code: i32,
    pub duration_s: u64,
}

/// A single compiler or tool diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Diagnostic {
    pub level: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub path: String,
    pub message: String,
    #[serde(default)]
    pub fingerprint: Option<String>,
}
