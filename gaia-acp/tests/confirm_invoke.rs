//! Invoke-path proof for #374. Local only.

use gaia_acp::*;

fn now() -> u64 {
    1_700_000_000
}

fn action(tool: &str, target: &str, class: ActionClass) -> ProposedAction {
    ProposedAction {
        agent_id: "agent-a".into(),
        tool: tool.into(),
        method: "call".into(),
        target: target.into(),
        action_class: class,
        payload: "ok".into(),
        nonce: "nonce-agent-a".into(),
        gateway_id: "gateway-local".into(),
        server_id: "server-local".into(),
        resource_id: "repo-local".into(),
        wants_delegation: false,
    }
}

fn plane() -> (ControlPlane, CapabilityManifest) {
    let p = ControlPlane::start(now(), "agent-a").unwrap();
    let mut m = CapabilityManifest::local_reader("agent-a", now());
    m.max_risk = RiskTier::T5;
    m.allowed_tools.extend([
        "ledger".into(),
        "contract".into(),
        "note".into(),
        "ship".into(),
        "public_comment".into(),
        "breaker".into(),
        "vault".into(),
    ]);
    (p, m)
}

#[test]
fn money_legal_medical_irreversible_outbound_invoke_need_receipt() {
    let cases = [
        (ActionClass::ExternalWrite, "ledger", "payment/wire"),
        (ActionClass::ExternalWrite, "contract", "legal/file"),
        (ActionClass::ExternalWrite, "note", "/phi/record"),
        (ActionClass::MergeDeployPublish, "ship", "main"),
        (ActionClass::ExternalWrite, "public_comment", "https://example.invalid/1"),
    ];
    for (class, tool, target) in cases {
        let (mut p, mut m) = plane();
        let r = p.invoke(&mut m, &action(tool, target, class), None, None);
        assert!(!r.executed);
        assert_eq!(r.reason, ReasonCode::ConfirmRequired, "{tool} {target}");
    }
}

#[test]
fn life_safety_invoke_denied_without_receipt() {
    let (mut p, mut m) = plane();
    let r = p.invoke(
        &mut m,
        &action("breaker", "plant/life-safety", ActionClass::Destructive),
        None,
        None,
    );
    assert!(!r.executed);
    assert_eq!(r.reason, ReasonCode::LifeSafetyDenied);
}

#[test]
fn vault_invoke_and_peer_envelope_denied() {
    let (mut p, mut m) = plane();
    let r = p.invoke(
        &mut m,
        &action("vault", "/vault/dump", ActionClass::SecretAccess),
        None,
        None,
    );
    assert!(!r.executed);
    assert_eq!(r.reason, ReasonCode::VaultDumpDenied);

    let env = PeerEnvelope {
        did: "did:gaia:peer-b".into(),
        intent_id: "intent-x".into(),
        purpose: "dump-vault".into(),
        raw_memory: false,
    };
    assert_eq!(env.validate(), Err(ReasonCode::VaultDumpDenied));
}
