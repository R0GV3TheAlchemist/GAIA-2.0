//! #24 Wasmtime proof: fixture runs, policy denies, trap isolated, limits bind.
//! No assertion here implies WASI/containerd/production sandbox equivalence.

use gaia_agents::{AgentManifest, Capability, RuntimeError, WasmOutcome, WasmRuntime};

fn manifest() -> AgentManifest {
    AgentManifest {
        name: "wasm-hello".into(),
        version: "0.1.0".into(),
        declared_capabilities: vec![Capability::MemoryRead],
        limits: gaia_agents::ResourceLimits::default(),
    }
}

#[test]
fn embedded_hello_guest_runs_without_wasi_resources() {
    let output = WasmRuntime::new()
        .invoke_fixture(&manifest(), "hello", Some(Capability::MemoryRead))
        .unwrap();
    match output {
        WasmOutcome::Output { text, chunks } => {
            assert_eq!(text, "hello from wasmstreamed");
            assert_eq!(chunks, vec!["hello from wasm".to_string(), "streamed".to_string()]);
        }
        other => panic!("unexpected {other:?}"),
    }
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
    assert!(matches!(next, WasmOutcome::Output { ref text, .. } if text.starts_with("hello from wasm")));
}

#[test]
fn admitted_memory_limit_stops_guest_growth() {
    let mut limited = manifest();
    limited.limits.memory_mib = 1;
    let outcome = WasmRuntime::new()
        .invoke_fixture(&limited, "grow", None)
        .unwrap();
    assert!(
        matches!(outcome, WasmOutcome::Trapped { .. }),
        "expected trap under 1 MiB limit, got {outcome:?}"
    );
    let next = WasmRuntime::new()
        .invoke_fixture(&manifest(), "hello", None)
        .unwrap();
    assert!(matches!(next, WasmOutcome::Output { .. }));
}

#[test]
fn zero_fuel_traps_before_useful_work() {
    let mut starved = manifest();
    starved.limits.cpu_millis = 0;
    let outcome = WasmRuntime::new()
        .invoke_fixture(&starved, "hello", None)
        .unwrap();
    assert!(
        matches!(outcome, WasmOutcome::Trapped { .. }),
        "expected out-of-fuel trap, got {outcome:?}"
    );
}

#[test]
fn excessive_manifest_limit_is_rejected_before_guest() {
    let mut fat = manifest();
    fat.limits.memory_mib = 513;
    let err = WasmRuntime::new()
        .invoke_fixture(&fat, "hello", None)
        .unwrap_err();
    assert!(matches!(err, RuntimeError::LimitRejected { .. }));
}
