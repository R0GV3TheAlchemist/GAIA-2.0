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
