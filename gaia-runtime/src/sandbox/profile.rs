//! Declarative, deny-by-default sandbox policy.
//!
//! [`SandboxProfile`] is the single source of truth for what a WASM component
//! is allowed to do.  Every field drives real Wasmtime / WASI 0.3 configuration
//! in [`super::manager::SandboxManager::build_wasi_ctx`] — there are no
//! metadata-only or documentation-only fields.

use serde::{Deserialize, Serialize};

use crate::sandbox::limits::ResourceQuota;

/// Capability and resource policy for a single sandbox execution.
///
/// # Deny-by-default
///
/// `SandboxProfile::default()` grants **nothing** — no filesystem, no network,
/// no environment variables, minimal quotas.  Callers must opt into each
/// capability explicitly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxProfile {
    /// Mount a temporary `/scratch` directory with full read+write access.
    /// When `false` (default) the component has no filesystem preopens at all.
    pub scratch_only_writes: bool,

    /// Grant the `wasi:sockets` world import so the component can open sockets.
    /// When `false` (default) the component cannot perform any network I/O.
    pub network: bool,

    /// Forward all host environment variables into the WASI environment.
    /// When `false` (default) the component sees an empty environment.
    pub inherited_env: bool,

    /// Hard resource ceilings enforced by [`super::limits::GaiaResourceLimiter`]
    /// and Wasmtime epoch interruption.
    pub quota: ResourceQuota,
}

impl Default for SandboxProfile {
    /// Deny-by-default: no filesystem, no network, no env, conservative quotas.
    fn default() -> Self {
        Self {
            scratch_only_writes: false,
            network: false,
            inherited_env: false,
            quota: ResourceQuota::default(),
        }
    }
}
