//! Shell runner — invokes `scripts/agent-validate.sh` and reads the result.

use std::{
    path::{Path, PathBuf},
    process::Command,
};
use crate::{
    error::ValidateError,
    result::ValidationResult,
};

/// Locate the workspace root by walking up from the current directory
/// until a `Cargo.toml` containing `[workspace]` is found.
fn find_workspace_root() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        let candidate = dir.join("Cargo.toml");
        if candidate.exists() {
            if let Ok(contents) = std::fs::read_to_string(&candidate) {
                if contents.contains("[workspace]") {
                    return Some(dir);
                }
            }
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// Run `scripts/agent-validate.sh` and return a typed [`ValidationResult`].
pub(crate) fn run(
    mode: &str,
    attempt: u32,
    prior_result_path: Option<&Path>,
) -> Result<ValidationResult, ValidateError> {
    let root = find_workspace_root()
        .unwrap_or_else(|| PathBuf::from("."));

    let script = root.join("scripts").join("agent-validate.sh");
    if !script.exists() {
        return Err(ValidateError::ScriptNotFound { path: script });
    }

    let result_path = root.join("agent-validation.json");

    let mut cmd = Command::new("bash");
    cmd.arg(&script)
       .arg(mode)
       .arg("--attempt")
       .arg(attempt.to_string())
       .current_dir(&root);

    if let Some(prior) = prior_result_path {
        cmd.arg("--fingerprint").arg(prior);
    }

    let output = cmd.output().map_err(|e| ValidateError::Launch { source: e })?;

    // Exit code 2 = usage error (bad arguments — should not happen in production)
    if output.status.code() == Some(2) {
        let detail = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(ValidateError::Usage { detail });
    }

    // Exit codes 0 (passed), 1 (failed), 3 (no_progress) are all valid —
    // the structured result in agent-validation.json carries the details.
    crate::get_validation_result(&result_path)
}
