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
fn invoke_without_claim_still_executes() {
    let mut p = ControlPlane::start(now(), "agent-a").unwrap();
    let mut m = CapabilityManifest::local_reader("agent-a", now());
    assert!(p.invoke(&mut m, &action(), None, None).executed);
}

#[test]
fn grounded_invoke_with_sources_executes() {
    let mut p = ControlPlane::start(now(), "agent-a").unwrap();
    let mut m = CapabilityManifest::local_reader("agent-a", now());
    let r = p.invoke_grounded(
        &mut m,
        &action(),
        None,
        None,
        GroundingClaim::required(vec!["chunk-1".into()]),
    );
    assert!(r.executed);
    assert_eq!(r.reason, ReasonCode::Allow);
}

#[test]
fn grounded_invoke_without_sources_denied() {
    let mut p = ControlPlane::start(now(), "agent-a").unwrap();
    let mut m = CapabilityManifest::local_reader("agent-a", now());
    let r = p.invoke_grounded(
        &mut m,
        &action(),
        None,
        None,
        GroundingClaim::required(vec![]),
    );
    assert!(!r.executed);
    assert_eq!(r.reason, ReasonCode::GroundingViolation);
}

#[test]
fn silent_ungrounded_denied() {
    let mut p = ControlPlane::start(now(), "agent-a").unwrap();
    let mut m = CapabilityManifest::local_reader("agent-a", now());
    let r = p.invoke_grounded(
        &mut m,
        &action(),
        None,
        None,
        GroundingClaim::ungrounded(false),
    );
    assert!(!r.executed);
    assert_eq!(r.reason, ReasonCode::UngroundedOptInRequired);
}
