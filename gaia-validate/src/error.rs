//! Error types for `gaia-validate`.

use std::path::PathBuf;
use thiserror::Error;

/// All errors that can be returned by this crate.
#[derive(Debug, Error)]
pub enum ValidateError {
    /// The validator script could not be found at the expected path.
    #[error("agent-validate.sh not found at {path}")]
    ScriptNotFound { path: PathBuf },

    /// The script was found but could not be launched.
    #[error("failed to launch agent-validate.sh: {source}")]
    Launch {
        #[source]
        source: std::io::Error,
    },

    /// The script exited with code 2 (usage error — bad arguments).
    #[error("agent-validate.sh usage error: {detail}")]
    Usage { detail: String },

    /// `agent-validation.json` was not written or could not be read.
    #[error("could not read result file {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// `agent-validation.json` contained invalid JSON.
    #[error("could not parse result file {path}: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    /// The directory passed to `get_attempt_history` could not be read.
    #[error("could not read history directory {path}: {source}")]
    HistoryDir {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
