//! Local-first agent control plane for GAIA 2.0 (#341-#348, #352, #335, #220, #187).
//! Fake adapters only. No sockets, live MCP, or credentials.

mod adapter;
mod approval;
mod audit;
mod autonomy;
mod claim;
mod config;
mod gateway;
mod live_trace;
mod manifest;
mod policy;
mod quota;
mod registry;
mod sandbox;
pub mod tool_audit;
pub mod tool_auth;
mod trace;
mod types;

pub use adapter::{FakeAdapter, RecordingAdapter};
pub use approval::{ApprovalDecision, HumanApprovalReceipt};
pub use audit::{ActionReceipt, AuditChain, AuditPushInput, PlaneEvent, PlaneState};
pub use autonomy::{gate as autonomy_gate, AutonomyLevel, ConfirmDomain, PeerEnvelope};
pub use claim::{claim_gate, Claim};
pub use config::{description_is_untrusted, digest_pinned, lint_mcp_config, McpServerConfig};
pub use gateway::{ControlPlane, InvokeResult};
pub use live_trace::{
    credential_is_absent, map_row, try_forward, LiveSendError, LiveTraceConfig, LiveTraceMode,
    LiveTraceRole, LiveTraceRow, LiveTraceTransport, RecordingLiveTransport,
};
pub use manifest::{CapabilityManifest, IdentityKind, PrincipalId, RevocationList};
pub use policy::{PolicyDecision, PolicyEngine, PolicyEvaluationContext, POLICY_VERSION};
pub use quota::{resource_quota_gate, ResourceQuota, ResourceUsage};
pub use registry::{LocalAip, LocalRegistry};
pub use sandbox::{
    classify_destination, classify_rebinding_host, classify_redirect, EgressClass, SandboxProfile,
};
pub use tool_audit::{ToolAuditEntry, ToolAuditLog, ToolOutcome};
pub use tool_auth::{
    authorize as authorize_tool, AgentId as ToolAgentId, ApprovalToken, AuthError, ToolCall,
    ToolId as AuthToolId, ToolPermissionTier as AuthToolPermissionTier,
};
pub use trace::{
    execution_allowed, from_invoke, refuse_live_supabase, ClaimClass, GapLock, GateMode,
    InvokeTraceInput, MemoryTraceSink, TraceEvent, TraceKind, TraceSink,
};
pub use types::{
    ActionClass, ProposedAction, ReasonCode, RiskTier, SignedIntent, TrustedPolicy,
    UntrustedContent, UntrustedToolOutput,
};

pub const CRATE_SCOPE: &str = "local-fake-mcp-only";
