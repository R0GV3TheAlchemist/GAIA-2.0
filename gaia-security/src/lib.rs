//! GAIA 2.0 Security Layer — gaia-security crate
//!
//! Implements the AI Hacking Defence Stack:
//!   1. PromptFirewall   — intent-level prompt-injection detection
//!   2. NonceStore       — replay-attack prevention (sliding-window)
//!   3. MemoryGuard      — keyed digest provenance for MemCube writes
//!   4. CapabilityEnforcer — strongly-consistent revocation enforcement
//!
//! All modules are zero-trust by default: deny unless every check passes.
//! No module claims production-grade cryptographic audit; see SECURITY.md.
//! Author: Kyle Steen (R0GV3TheAlchemist)

pub mod capability;
pub mod firewall;
pub mod memory_guard;
pub mod nonce;

pub use capability::{AuthzDecision, CapabilityEnforcer, CapabilityToken, RevocationRecord};
pub use firewall::{DenyReason, FirewallConfig, FirewallDecision, PromptFirewall};
pub use memory_guard::{MemoryGuard, MemoryGuardError, ProvenanceTag};
pub use nonce::{NonceStore, ReplayResult};
