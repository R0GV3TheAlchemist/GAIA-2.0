//! `gaia-runtime` — WASM/WASI 0.3 sandbox execution engine for GAIA agents.
//!
//! # Overview
//!
//! This crate wraps Wasmtime 46 (WASI 0.3, Component Model) behind a small,
//! GAIA-idiomatic API surface:
//!
//! - [`SandboxProfile`]  — declarative, deny-by-default capability policy
//! - [`SandboxManager`]  — compiles + executes WASM components under the profile
//! - [`GaiaResourceLimiter`] — Wasmtime `ResourceLimiter` enforcing memory/table quotas
//! - [`SandboxError`]    — structured error type with `GAIA_CAPABILITY_DENIED` constant
//!
//! See issue #740 for the full specification.

pub mod sandbox;

pub use sandbox::{
    error::{SandboxError, GAIA_CAPABILITY_DENIED},
    limits::{GaiaResourceLimiter, ResourceQuota},
    manager::SandboxManager,
    profile::SandboxProfile,
};

#[cfg(test)]
mod tests {
    use super::*;
    use wasmtime::ResourceLimiter;

    // -------------------------------------------------------------------------
    // SandboxProfile — deny-by-default invariants
    // -------------------------------------------------------------------------

    /// The default profile must deny every capability.
    /// This is the primary security invariant of gaia-runtime:
    /// a component gets nothing unless explicitly granted.
    #[test]
    fn profile_default_denies_all_capabilities() {
        let p = SandboxProfile::default();
        assert!(!p.scratch_only_writes, "default must not grant scratch writes");
        assert!(!p.network,             "default must not grant network access");
        assert!(!p.inherited_env,       "default must not forward env vars");
    }

    /// Explicitly enabling scratch writes flips only that field.
    #[test]
    fn profile_scratch_only_writes_field() {
        let p = SandboxProfile {
            scratch_only_writes: true,
            ..SandboxProfile::default()
        };
        assert!(p.scratch_only_writes);
        assert!(!p.network);
        assert!(!p.inherited_env);
    }

    /// Explicitly enabling network flips only that field.
    #[test]
    fn profile_network_field() {
        let p = SandboxProfile {
            network: true,
            ..SandboxProfile::default()
        };
        assert!(p.network);
        assert!(!p.scratch_only_writes);
        assert!(!p.inherited_env);
    }

    // -------------------------------------------------------------------------
    // ResourceQuota — default values match specification
    // -------------------------------------------------------------------------

    /// Default quota values must match the constants documented in limits.rs.
    /// If these change, it is a breaking governance decision and must be logged.
    #[test]
    fn quota_default_values() {
        let q = ResourceQuota::default();
        assert_eq!(q.max_memory_bytes, 64 * 1024 * 1024, "default memory quota must be 64 MiB");
        assert_eq!(q.max_epochs,       5,                 "default epoch budget must be 5");
        assert_eq!(q.max_fds,          32,                "default fd limit must be 32");
        assert_eq!(q.max_processes,    0,                 "default must allow zero sub-processes");
    }

    // -------------------------------------------------------------------------
    // GaiaResourceLimiter — memory enforcement
    // -------------------------------------------------------------------------

    /// A memory request within quota must be approved.
    #[test]
    fn resource_limiter_allows_within_quota() {
        let quota = ResourceQuota {
            max_memory_bytes: 64 * 1024 * 1024,
            ..ResourceQuota::default()
        };
        let mut limiter = GaiaResourceLimiter::new(quota);
        let result = limiter.memory_growing(0, 32 * 1024 * 1024, None);
        assert!(result.unwrap(), "request within quota must be approved");
        assert_eq!(limiter.mem_used(), 32 * 1024 * 1024);
    }

    /// A memory request that exceeds the quota must be denied (returns Ok(false)).
    /// Wasmtime converts Ok(false) into a clean OOM trap — never a panic.
    #[test]
    fn resource_limiter_denies_over_quota() {
        let quota = ResourceQuota {
            max_memory_bytes: 64 * 1024 * 1024,
            ..ResourceQuota::default()
        };
        let mut limiter = GaiaResourceLimiter::new(quota);
        let result = limiter.memory_growing(0, 128 * 1024 * 1024, None);
        assert!(!result.unwrap(), "request over quota must be denied via Ok(false)");
    }
}
