//! L5 agent ecosystem (#5).
//! #24 Wasmtime guest, #25 pack, #26 A2A handoff and signed install.
//! This is not a marketplace transport or containerd sandbox.

mod a2a;
mod host;
mod pack;
mod runtime;
mod wasm;

pub use a2a::{
    registry_layout, ContextBundle, Handoff, MarketError, Package, PackageMarket, PrivacyMode,
};
pub use host::{AgentHost, HostError, Review};
pub use pack::{catalog, find as find_pack, AgentKind, PackEntry};
pub use runtime::{
    AgentManifest, AgentOutcome, AgentRuntime, Capability, ResourceLimits, RuntimeError,
};
pub use wasm::{WasiGrant, WasmOutcome, WasmRuntime};
