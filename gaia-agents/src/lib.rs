//! L5 agent ecosystem (#5).
//! #24 Wasmtime guest, #25 pack, #26 A2A handoff and signed install.
//! #722 Full DISCOVER→ARCHIVE lifecycle state machine.
//! This is not a marketplace transport or containerd sandbox.

mod a2a;
mod host;
mod lifecycle;
mod pack;
mod runtime;
pub mod tool_registry;
mod wasm;

pub use a2a::{
    registry_layout, ContextBundle, Handoff, MarketError, Package, PackageMarket, PrivacyMode,
};
pub use host::{AgentHost, HostError, Review};
pub use lifecycle::{
    AgentCheckpoint, AgentDid, CapabilityToken, LifecycleError, LifecycleEvent, LifecycleManager,
    LifecycleStage, ResourceSnapshot,
};
pub use pack::{catalog, find as find_pack, AgentKind, PackEntry};
pub use runtime::{
    AgentManifest, AgentOutcome, AgentRuntime, Capability, ResourceLimits, RuntimeError,
};
pub use tool_registry::{
    AgentId as ToolAgentId, ToolId, ToolPermissionTier, ToolRegistration, ToolRegistry,
};
pub use wasm::{WasiGrant, WasmOutcome, WasmRuntime};
