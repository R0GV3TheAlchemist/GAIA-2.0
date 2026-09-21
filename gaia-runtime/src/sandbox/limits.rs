//! Resource quota definition and Wasmtime `ResourceLimiter` implementation.
//!
//! [`GaiaResourceLimiter`] is attached to a Wasmtime `Store` via
//! `store.limiter(|data| &mut data.limiter)` before each component execution.
//! It enforces hard ceilings on linear memory and table growth; the
//! CPU/wall-clock budget is enforced separately via epoch interruption in
//! [`super::manager`].

use serde::{Deserialize, Serialize};
use wasmtime::ResourceLimiter;

/// Hard resource ceilings for a single sandbox execution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResourceQuota {
    /// Maximum linear-memory size in bytes.
    /// Exceeding this causes Wasmtime to raise an OOM trap → clean termination.
    pub max_memory_bytes: usize,

    /// Wall-clock epoch budget.
    ///
    /// Wasmtime's epoch counter is incremented by the host runtime at a
    /// configurable tick rate (typically 1 Hz or 10 Hz).  This value is the
    /// number of epoch ticks before an interrupt is raised, NOT milliseconds.
    /// At the default 1-tick-per-second rate, `max_epochs = 5` ≈ 5 seconds.
    pub max_epochs: u64,

    /// Maximum number of open file descriptors (tracked for audit purposes;
    /// hard enforcement is via OS rlimit in production).
    pub max_fds: u32,

    /// Maximum number of spawnable sub-processes (0 = none allowed).
    pub max_processes: u32,
}

impl Default for ResourceQuota {
    fn default() -> Self {
        Self {
            max_memory_bytes: 64 * 1024 * 1024, // 64 MiB
            max_epochs:       5,                 // ~5 s at 1 tick/s
            max_fds:          32,
            max_processes:    0,
        }
    }
}

/// Wasmtime [`ResourceLimiter`] that enforces a [`ResourceQuota`].
///
/// Store this inside the `Store`'s data type and reference it via:
/// ```ignore
/// store.limiter(|data: &mut StoreData| &mut data.limiter);
/// ```
pub struct GaiaResourceLimiter {
    quota:    ResourceQuota,
    mem_used: usize,
}

impl GaiaResourceLimiter {
    /// Construct a limiter from a quota snapshot.
    pub fn new(quota: ResourceQuota) -> Self {
        Self { quota, mem_used: 0 }
    }

    /// Current linear-memory high-water mark in bytes.
    pub fn mem_used(&self) -> usize {
        self.mem_used
    }
}

impl ResourceLimiter for GaiaResourceLimiter {
    /// Called by Wasmtime whenever the component requests more linear memory.
    ///
    /// Returns `Ok(false)` — causing an OOM trap — when `desired` exceeds
    /// `max_memory_bytes`.
    fn memory_growing(
        &mut self,
        _current:  usize,
        desired:   usize,
        _maximum:  Option<usize>,
    ) -> anyhow::Result<bool> {
        if desired > self.quota.max_memory_bytes {
            Ok(false) // deny → Wasmtime raises OOM trap → clean termination
        } else {
            self.mem_used = desired;
            Ok(true)
        }
    }

    /// Called by Wasmtime whenever the component requests more table entries.
    ///
    /// Capped at 10 000 entries as a sane default.
    fn table_growing(
        &mut self,
        _current: u32,
        desired:  u32,
        _maximum: Option<u32>,
    ) -> anyhow::Result<bool> {
        Ok(desired <= 10_000)
    }
}
