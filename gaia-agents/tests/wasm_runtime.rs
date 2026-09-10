//! #24 minimal Wasmtime proof: fixture runs, policy denies, trap is isolated.
//! No assertion here implies WASI/containerd/production sandbox equivalence.

use gaia_agents::{
    AgentManifest, Capability, ResourceLimits, RuntimeError, WasmOutcome, WasmRuntime,
};

fn manifest() -> AgentManifest {
    AgentManifest {
        name: "wasm-hello".into(),
        version: "0.1.0".into(),
        declared_capabilities: vec![Capability::MemoryRead],
        limits: ResourceLimits::default(),
    }
}

#[test]
fn embedded_hello_guest_runs_without_wasi_resources() {
    let output = WasmRuntime::new()
        .invoke_fixture(&manifest(), "hello", Some(Capability::MemoryRead))
        .unwrap();
    assert_eq!(output, WasmOutcome::Output("hello from wasm".into()));
}

#[test]
fn ungranted_capability_never_reaches_guest() {
    let err = WasmRuntime::new()
        .invoke_fixture(&manifest(), "hello", Some(Capability::Network))
        .unwrap_err();
    assert_eq!(
        err,
        RuntimeError::UndeclaredCapability {
            agent: "wasm-hello".into(),
            capability: Capability::Network,
        }
    );
}

#[test]
fn guest_trap_is_isolated_and_next_guest_runs() {
    let runtime = WasmRuntime::new();
    let trapped = runtime.invoke_fixture(&manifest(), "trap", None).unwrap();
    assert!(matches!(trapped, WasmOutcome::Trapped { agent, .. } if agent == "wasm-hello"));
    let next = runtime.invoke_fixture(&manifest(), "hello", None).unwrap();
    assert_eq!(next, WasmOutcome::Output("hello from wasm".into()));
}
