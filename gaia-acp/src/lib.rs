//! Local-first agent control plane for GAIA 2.0 (#341–#348).
//!
//! Invariant: an agent may propose an action; an independent deterministic
//! enforcement layer decides whether that exact action may execute.
//! Untrusted content cannot grant capability, alter policy, approve an action,
//! expand scope, or bypass a human gate.
//!
//! This crate uses only fake/local MCP adapters. It does not open sockets,
//! call real MCP servers, or touch live credentials.

mod approval;
mod audit;
mod config;
mod gateway;
mod manifest;
mod policy;
mod sandbox;
mod types;

pub use approval::{ApprovalDecision, HumanApprovalReceipt};
pub use audit::{ActionReceipt, AuditChain, PlaneEvent, PlaneState};
pub use config::{lint_mcp_config, McpServerConfig};
pub use gateway::{ControlPlane, InvokeResult};
pub use manifest::{CapabilityManifest, IdentityKind, PrincipalId, RevocationList};
pub use policy::{PolicyDecision, PolicyEngine, POLICY_VERSION};
pub use sandbox::{classify_destination, EgressClass, SandboxProfile};
pub use types::{
    ActionClass, ProposedAction, ReasonCode, RiskTier, SignedIntent, TrustedPolicy,
    UntrustedContent, UntrustedToolOutput,
};

pub const CRATE_SCOPE: &str = "local-fake-mcp-only";
