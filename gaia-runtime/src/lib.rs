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
//!
//! Runtime-integrity scaffold (#932 / #934): grounding, faithfulness,
//! circuit_breaker, quotas, dormancy. Typed stubs with unit tests. No LLM
//! judge, no RSS measurement, no SOS wiring.

pub mod circuit_breaker;
pub mod dormancy;
pub mod faithfulness;
pub mod grounding;
pub mod quotas;
pub mod sandbox;

pub use circuit_breaker::{CircuitBreaker, CircuitOpen, CircuitState};
pub use dormancy::{DormancyGuard, DormancyMode, DormantSystem};
pub use faithfulness::{score_faithfulness, FaithfulnessScore};
pub use grounding::{
    enforce_grounding, ChunkId, GenerationMode, GroundedResponse, GroundingError,
};
pub use quotas::{AgentQuota, RateLimitExceeded, RuntimeAnomaly};
pub use sandbox::{
    error::{SandboxError, GAIA_CAPABILITY_DENIED},
    limits::{GaiaResourceLimiter, ResourceQuota},
    manager::SandboxManager,
    profile::SandboxProfile,
};
