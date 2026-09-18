//! GAIA local security primitives. Deny by default.
//! These components are in-process seams, not claims of a distributed security service.

pub mod capability;
pub mod firewall;
pub mod memory_guard;
pub mod nonce;
pub mod trust_boundary;

pub use capability::{
    AuthzAuditEvent, AuthzDecision, CapabilityEnforcer, CapabilityToken, DenyCode,
    RevocationRecord,
};
pub use firewall::{FirewallConfig, FirewallDecision, FirewallReason, PromptFirewall};
pub use memory_guard::{MemoryGuard, MemoryGuardError, ProvenanceTag, SignedMemoryRecord};
pub use nonce::{NonceStore, ReplayResult};
pub use trust_boundary::{
    may_supply_authority, AuthorityUse, TrustBoundaryDecision, TrustBoundaryDenyCode, TrustClass,
    Untrusted,
};
