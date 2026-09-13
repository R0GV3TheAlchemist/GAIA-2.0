//! L5 agent ecosystem (#5).
//! #24 policy runtime plus a Wasmtime guest with fuel, memory limits, and WASI import grants.
//! This is not containerd or a production multi-tenant sandbox. There is no WASI host.

mod runtime;
mod wasm;

pub use runtime::{
    AgentManifest, AgentOutcome, AgentRuntime, Capability, ResourceLimits, RuntimeError,
};
pub use wasm::{WasiGrant, WasmOutcome, WasmRuntime};
