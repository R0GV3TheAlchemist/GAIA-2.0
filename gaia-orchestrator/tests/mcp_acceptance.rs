//! #23 AT-01..AT-06. Local/in-process only. AT-07..AT-10 remain unclaimed.

use gaia_memos::MemOs;
use gaia_orchestrator::{IntentEngine, IntentSigner, JsonRpcRequest, McpMessage, McpRegistry};

#[test]
at01_signed_intent_invokes_once_with_provenance() {
    let mut mem = MemOs::new();
    let graph = IntentEngine::local_stub()
        .parse("research and summarize CARE", &mut mem)
        .unwrap();
    let signer = IntentSigner::generate();
    let mut reg = McpRegistry::local().trust(&signer);
    let out = reg.invoke_from_intent(&graph, &signer).unwrap();
    assert_eq!(reg.handler_invocations, 1);
    assert!(out.contains(&graph.id.to_string()));
    assert!(out.contains("gaia://agent/agent.researcher"));
    assert!(out.contains("provenance=local-stub"));
    assert!(out.contains("quality=fixture"));
    assert!(out.contains("freshness=static"));
    assert!(out.contains("audit="));
    assert!(reg.audit.iter().any(|e| e.starts_with("ADMIT")));
}

#[test]
at02_resources_list_exposes_aip_without_secrets() {
    let mut reg = McpRegistry::local();
    let signer = IntentSigner::generate();
    let listed = reg.handle(JsonRpcRequest::signed(&signer, 2, "resources/list", "{}"));
    let body = listed.result.unwrap();
    assert!(body.contains("agent.researcher"));
    assert!(body.contains("cognitive-agent.json"));
    assert!(body.contains("research.summarize"));
    assert!(body.contains("in-process"));
    assert!(!body.to_lowercase().contains("private"));
    assert!(!body.contains("ed25519:"));
}

#[test]
at03_unsigned_is_unauthenticated_without_handler() {
    let mut reg = McpRegistry::local();
    let err = reg
        .invoke(&McpMessage::unsigned("research.summarize", "{}"))
        .unwrap_err();
    assert!(err.contains("GAIA_UNAUTHENTICATED"));
    assert_eq!(reg.handler_invocations, 0);
    assert!(reg.audit.iter().any(|e| e.contains("GAIA_UNAUTHENTICATED")));
}

#[test]
at04_forged_signature_is_invalid_without_handler() {
    let mut reg = McpRegistry::local();
    let err = reg
        .invoke(&McpMessage::marked_signed(
            "research.summarize",
            "{}",
            "ed25519:deadbeef:00",
        ))
        .unwrap_err();
    assert!(err.contains("GAIA_SIGNATURE_INVALID"));
    assert_eq!(reg.handler_invocations, 0);
}

#[test]
at05_unknown_and_revoked_keys_are_untrusted() {
    let trusted = IntentSigner::generate();
    let stranger = IntentSigner::generate();
    let mut unknown = McpRegistry::local().trust(&trusted);
    let err = unknown
        .invoke(&McpMessage::signed_by(
            &stranger,
            "research.summarize",
            "research.summarize",
        ))
        .unwrap_err();
    assert!(err.contains("GAIA_IDENTITY_UNTRUSTED"));
    assert_eq!(unknown.handler_invocations, 0);

    let revoked = IntentSigner::generate();
    let mut closed = McpRegistry::local().revoke(&revoked);
    let err = closed
        .invoke(&McpMessage::signed_by(
            &revoked,
            "research.summarize",
            "research.summarize",
        ))
        .unwrap_err();
    assert!(err.contains("GAIA_IDENTITY_UNTRUSTED"));
    assert_eq!(closed.handler_invocations, 0);
}

#[test]
at06_policy_denies_remote_network_without_handler() {
    let signer = IntentSigner::generate();
    let mut reg = McpRegistry::local().trust(&signer);
    let denied = reg.handle(JsonRpcRequest::signed(
        &signer,
        9,
        "tools/call",
        r#"{"tool":"research.summarize","capabilities":["network"],"jurisdiction":"remote"}"#,
    ));
    assert!(denied.error.unwrap().contains("GAIA_POLICY_DENIED"));
    assert_eq!(reg.handler_invocations, 0);
    assert!(reg.audit.iter().any(|e| e.contains("GAIA_POLICY_DENIED")));
}
