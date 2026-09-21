//! [`ExecutionError`] — typed error codes for all 12 pipeline stages.
//!
//! Every variant maps to one of the eight GAIA error-code constants defined
//! in the issue specification.  The constants are `pub` so callers can match
//! on the string codes in API responses without depending on the enum.

use thiserror::Error;

// ── GAIA error-code constants ────────────────────────────────────────────────

pub const GAIA_INTENT_SIGNATURE_REQUIRED: &str = "GAIA_INTENT_SIGNATURE_REQUIRED";
pub const GAIA_INTENT_SCHEMA_INVALID:     &str = "GAIA_INTENT_SCHEMA_INVALID";
pub const GAIA_NO_CAPABLE_AGENT:          &str = "GAIA_NO_CAPABLE_AGENT";
pub const GAIA_CAPABILITY_DENIED:         &str = "GAIA_CAPABILITY_DENIED";
pub const GAIA_RESOURCE_UNAVAILABLE:      &str = "GAIA_RESOURCE_UNAVAILABLE";
pub const GAIA_EXECUTION_TIMEOUT:         &str = "GAIA_EXECUTION_TIMEOUT";
pub const GAIA_REPLAN_EXHAUSTED:          &str = "GAIA_REPLAN_EXHAUSTED";
pub const GAIA_AUDIT_FAILURE:             &str = "GAIA_AUDIT_FAILURE";

// ── ExecutionError ───────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum ExecutionError {
    /// Stage 1 — intent is missing a valid Ed25519 signature.
    #[error("{GAIA_INTENT_SIGNATURE_REQUIRED}: {0}")]
    SignatureRequired(String),

    /// Stage 1 — intent JSON does not match the declared schema.
    #[error("{GAIA_INTENT_SCHEMA_INVALID}: {0}")]
    SchemaInvalid(String),

    /// Stage 4 — no registered agent satisfies the capability requirement.
    #[error("{GAIA_NO_CAPABLE_AGENT}: {0}")]
    NoCapableAgent(String),

    /// Stage 6 — ACP policy rejected the agent's capability request.
    #[error("{GAIA_CAPABILITY_DENIED}: {0}")]
    CapabilityDenied(String),

    /// Stage 7 — insufficient CPU / memory quota.
    #[error("{GAIA_RESOURCE_UNAVAILABLE}: {0}")]
    ResourceUnavailable(String),

    /// Stage 8 — sandbox wall-clock timeout exceeded.
    #[error("{GAIA_EXECUTION_TIMEOUT}: {0}")]
    Timeout(String),

    /// Stage 10 — every fallback path failed.
    #[error("{GAIA_REPLAN_EXHAUSTED}: {0}")]
    ReplanExhausted(String),

    /// Stage 11 — audit log write failed; execution must halt.
    #[error("{GAIA_AUDIT_FAILURE}: {0}")]
    AuditFailure(String),
}
