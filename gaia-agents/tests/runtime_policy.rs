//! #24 policy harness proof. This is deliberately not a WASM/WASI sandbox test.

use gaia_agents::{
    AgentManifest, AgentOutcome, AgentRuntime, Capability, ResourceLimits, RuntimeError,
};

fn hello_manifest() -> AgentManifest {
    AgentManifest {
        name: "hello-agent".into(),
        version: "0.1.0".into(),
        declared_capabilities: vec![Capability::MemoryRead],
        limits: ResourceLimits::default(),
    }
}

#[test]
fn hello_fixture_runs_with_declared_capability() {
    let runtime = AgentRuntime::default();
    let outcome = runtime
        .invoke(&hello_manifest(), "hello", Some(Capability::MemoryRead))
        .unwrap();
    assert_eq!(outcome, AgentOutcome::Output("hello from hello-agent".into()));
}

#[test]
fn undeclared_capability_is_denied_before_fixture_behavior() {
    let runtime = AgentRuntime::default();
    let err = runtime
        .invoke(&hello_manifest(), "hello", Some(Capability::Network))
        .unwrap_err();
    assert_eq!(
        err,
        RuntimeError::UndeclaredCapability {
            agent: "hello-agent".into(),
            capability: Capability::Network,
        }
    );
}

#[test]
fn excessive_resource_request_is_rejected_at_admission() {
    let runtime = AgentRuntime::default();
    let mut manifest = hello_manifest();
    manifest.limits.memory_mib = 513;
    assert!(matches!(runtime.admit(&manifest), Err(RuntimeError::LimitRejected { .. })));
}

#[test]
fn fixture_crash_is_an_isolated_agent_outcome() {
    let runtime = AgentRuntime::default();
    let crash = runtime.invoke(&hello_manifest(), "crash", None).unwrap();
    assert!(matches!(crash, AgentOutcome::Crashed { agent, .. } if agent == "hello-agent"));
    let next = runtime.invoke(&hello_manifest(), "hello", None).unwrap();
    assert!(matches!(next, AgentOutcome::Output(_)));
}
