//! L5 agent ecosystem (#5).
//! #24 policy runtime plus a Wasmtime guest with fuel and memory limits.
//! This is not containerd or a production multi-tenant sandbox.

mod runtime;
mod wasm;

pub use runtime::{
    AgentManifest, AgentOutcome, AgentRuntime, Capability, ResourceLimits, RuntimeError,
};
pub use wasm::{WasmOutcome, WasmRuntime};
