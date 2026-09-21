//! Sandbox subsystem — Wasmtime 46 + WASI 0.3 isolation backend.
//!
//! Module layout (mirrors #740 directory spec):
//!
//! ```text
//! sandbox/
//!   error.rs    — SandboxError + GAIA_CAPABILITY_DENIED constant
//!   limits.rs   — ResourceQuota + GaiaResourceLimiter (Wasmtime ResourceLimiter)
//!   profile.rs  — SandboxProfile (declarative, deny-by-default policy)
//!   manager.rs  — SandboxManager (engine init, WasiCtxBuilder, execute)
//! ```

pub mod error;
pub mod limits;
pub mod manager;
pub mod profile;
