use gaia_acp::*;

fn now() -> u64 {
    1_700_000_000
}

fn action(agent: &str, tool: &str, target: &str, class: ActionClass) -> ProposedAction {
    ProposedAction {
        agent_id: agent.into(),
        tool: tool.into(),
        method: "call".into(),
        target: target.into(),
        action_class: class,
        payload: "ok".into(),
        nonce: format!("nonce-{agent}"),
        gateway_id: "gateway-local".into(),
        server_id: "server-local".into(),
        resource_id: "repo-local".into(),
        wants_delegation: false,
    }
}

#[test]
fn invoke_emits_allow_deny_replay_and_kill() {
    let mut plane = ControlPlane::start(now(), "agent-a").unwrap();
    let mut man = CapabilityManifest::local_reader("agent-a", now());
    let ok = action(
        "agent-a",
        "local_parse",
        "scratch/x",
        ActionClass::LocalParse,
    );
    assert!(plane.invoke(&mut man, &ok, None, None).executed);

    let bad = action(
        "agent-a",
        "shell",
        "https://evil.test",
        ActionClass::NetworkEgress,
    );
    let d = plane.invoke(&mut man, &bad, None, None);
    assert!(!d.executed);

    plane.kill("agent-a");
    assert!(plane.traces.kinds().contains(&TraceKind::Allow));
    assert!(plane.traces.kinds().contains(&TraceKind::Deny));
    assert!(plane.traces.kinds().contains(&TraceKind::Kill));
    assert!(refuse_live_supabase().is_err());
}
