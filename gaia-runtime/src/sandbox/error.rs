//! Structured error type for the gaia-runtime sandbox subsystem.
//!
//! [`GAIA_CAPABILITY_DENIED`] is the canonical audit-receipt string for any
//! capability violation detected at the sandbox boundary.  It must never be
//! changed without a matching protocol version bump.

use thiserror::Error;

/// Canonical error code embedded in every capability-violation audit receipt.
/// Referenced by the ACP audit pipeline (see #726).
pub const GAIA_CAPABILITY_DENIED: &str = "GAIA_CAPABILITY_DENIED";

/// Errors that can arise from sandbox construction or component execution.
#[derive(Debug, Error)]
pub enum SandboxError {
    /// Wasmtime engine or linker initialisation failed.
    #[error("engine initialisation failed: {0}")]
    EngineInit(#[from] anyhow::Error),

    /// A component attempted to use a capability it was not granted.
    /// The inner string is always [`GAIA_CAPABILITY_DENIED`].
    #[error("capability violation: {0}")]
    CapabilityDenied(String),

    /// The component exceeded its memory quota; Wasmtime raised an OOM trap.
    #[error("component exceeded memory quota (OOM termination)")]
    OomTermination,

    /// The component exceeded its wall-clock CPU budget (epoch interrupt).
    #[error("component exceeded CPU/wall-clock budget (timeout)")]
    Timeout,

    /// The component bytes failed to compile as a WASM Component Model binary.
    #[error("component failed to compile: {0}")]
    CompileError(String),

    /// An unexpected Wasmtime trap occurred that does not map to a known category.
    #[error("trap during execution: {0}")]
    Trap(String),
}
