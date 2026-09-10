//! L5 agent ecosystem (#5).
//! #24 policy-runtime harness only: not a WASM/WASI sandbox.

mod runtime;

pub use runtime::{
    AgentManifest, AgentOutcome, AgentRuntime, Capability, ResourceLimits, RuntimeError,
};
