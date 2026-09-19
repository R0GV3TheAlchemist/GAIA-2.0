use gaia_acp::*;

fn now() -> u64 {
    1_700_000_000
}

fn action() -> ProposedAction {
    ProposedAction {
        agent_id: "agent-a".into(),
        tool: "local_read".into(),
        method: "call".into(),
        target: "docs/a.md".into(),
        action_class: ActionClass::LocalRead,
        payload: "ok".into(),
        nonce: "nonce-agent-a".into(),
        gateway_id: "gateway-local".into(),
        server_id: "server-local".into(),
        resource_id: "repo-local".into(),
        wants_delegation: false,
    }
}

#[test]
fn pause_denies_next_invoke_resume_allows() {
    let mut p = ControlPlane::start(now(), "agent-a").unwrap();
    let mut m = CapabilityManifest::local_reader("agent-a", now());
    let a = action();
    assert!(p.invoke(&mut m, &a, None, None).executed);
    assert!(p.pause("agent-a").is_ok());
    let paused = p.invoke(&mut m, &a, None, None);
    assert!(!paused.executed);
    assert_eq!(paused.reason, ReasonCode::StateInvalid);
    assert!(p.resume("agent-a").is_ok());
    assert!(p.invoke(&mut m, &a, None, None).executed);
    assert!(p.audit.chain_ok());
}

#[test]
fn kill_cannot_resume() {
    let mut p = ControlPlane::start(now(), "agent-a").unwrap();
    let mut m = CapabilityManifest::local_reader("agent-a", now());
    p.kill("agent-a");
    assert_eq!(p.resume("agent-a"), Err(ReasonCode::EmergencyStop));
    let a = action();
    assert!(!p.invoke(&mut m, &a, None, None).executed);
}
