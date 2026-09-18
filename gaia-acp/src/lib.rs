//! Local-first agent control plane for GAIA 2.0 (#341-#348, #352, #335, #220, #187).
//! Fake adapters only. No sockets, live MCP, or credentials.

mod adapter;
mod approval;
mod audit;
mod autonomy;
mod config;
mod gateway;
mod manifest;
mod policy;
mod sandbox;
mod trace;
mod types;

pub use adapter::{FakeAdapter, RecordingAdapter};
pub use approval::{ApprovalDecision, HumanApprovalReceipt};
pub use audit::{ActionReceipt, AuditChain, PlaneEvent, PlaneState};
pub use autonomy::{gate as autonomy_gate, AutonomyLevel, ConfirmDomain, PeerEnvelope};
pub use config::{description_is_untrusted, digest_pinned, lint_mcp_config, McpServerConfig};
pub use gateway::{ControlPlane, InvokeResult};
pub use manifest::{CapabilityManifest, IdentityKind, PrincipalId, RevocationList};
pub use policy::{PolicyDecision, PolicyEngine, POLICY_VERSION};
pub use sandbox::{classify_destination, classify_rebinding_host, classify_redirect, EgressClass, SandboxProfile};
pub use trace::{execution_allowed, from_invoke, refuse_live_supabase, ClaimClass, GapLock, GateMode, MemoryTraceSink, TraceEvent, TraceKind, TraceSink};
pub use types::{
    ActionClass, ProposedAction, ReasonCode, RiskTier, SignedIntent, TrustedPolicy,
    UntrustedContent, UntrustedToolOutput,
};

pub const CRATE_SCOPE: &str = "local-fake-mcp-only";
