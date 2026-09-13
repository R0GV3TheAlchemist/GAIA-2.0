//! L5 agent ecosystem (#5).
//! #24 Wasmtime guest plus #25 local system/cognitive pack.
//! This is not a marketplace, containerd, or A2A transport.

mod host;
mod pack;
mod runtime;
mod wasm;

pub use host::{AgentHost, HostError, Review};
pub use pack::{catalog, find as find_pack, AgentKind, PackEntry};
pub use runtime::{
    AgentManifest, AgentOutcome, AgentRuntime, Capability, ResourceLimits, RuntimeError,
};
pub use wasm::{WasiGrant, WasmOutcome, WasmRuntime};
