use gaia_acp::*;

fn act(class: ActionClass, tool: &str, target: &str) -> ProposedAction {
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

#[test]
fn default_level_is_suggest() {
    assert_eq!(AutonomyLevel::default_level(), AutonomyLevel::Suggest);
}

#[test]
fn default_level_cannot_write_or_egress() {
    let w = act(ActionClass::RepoWrite, "edit", "docs/a.md");
    assert_eq!(
        autonomy_gate(AutonomyLevel::default_level(), &w, None),
        Err(ReasonCode::AutonomyCap)
    );
}

#[test]
fn life_safety_without_ack_is_denied() {
    let a = act(ActionClass::Destructive, "breaker", "plant/life-safety");
    assert_eq!(
        autonomy_gate(AutonomyLevel::BoundedRemediate, &a, None),
        Err(ReasonCode::LifeSafetyDenied)
    );
}

#[test]
fn peer_cannot_dump_vault() {
    let env = PeerEnvelope {
        did: "did:gaia:peer-b".into(),
        intent_id: "intent-x".into(),
        purpose: "dump-vault".into(),
        raw_memory: false,
    };
    assert_eq!(env.validate(), Err(ReasonCode::VaultDumpDenied));
    let raw = PeerEnvelope {
        did: "did:gaia:peer-b".into(),
        intent_id: "intent-x".into(),
        purpose: "summarize".into(),
        raw_memory: true,
    };
    assert_eq!(raw.validate(), Err(ReasonCode::VaultDumpDenied));
}

#[test]
fn money_legal_medical_irreversible_outbound_need_receipt() {
    for (class, tool, target) in [
        (ActionClass::ExternalWrite, "ledger", "payment/wire"),
        (ActionClass::ExternalWrite, "contract", "legal/file"),
        (ActionClass::ExternalWrite, "note", "/phi/record"),
        (ActionClass::MergeDeployPublish, "ship", "main"),
        (ActionClass::ExternalWrite, "public_comment", "https://example.invalid/1"),
    ] {
        let a = act(class, tool, target);
        assert_eq!(
            autonomy_gate(AutonomyLevel::BoundedRemediate, &a, None),
            Err(ReasonCode::ConfirmRequired),
            "{tool} {target}"
        );
    }
}

#[test]
fn plant_act_level_refused_in_software() {
    let a = act(ActionClass::LocalRead, "local_read", "docs/a.md");
    assert_eq!(
        autonomy_gate(AutonomyLevel::PlantAct, &a, None),
        Err(ReasonCode::AutonomyCap)
    );
}
