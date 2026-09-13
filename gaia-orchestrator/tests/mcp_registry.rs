//! #23 MCP stub: JSON-RPC session, invoke from intent, AIP registry, reject unsigned.

use gaia_memos::MemOs;
use gaia_orchestrator::{DiscoveryStub, IntentEngine, IntentSigner, JsonRpcRequest, McpMessage, McpRegistry};

#[test]
fn registry_lists_local_aip_manifests() {
    let reg = McpRegistry::local();
    assert!(reg.list().iter().any(|a| a.name == "gaia-local-researcher"));
    assert!(reg.list()[0].tools.iter().any(|t| t.name == "research.summarize"));
    assert!(reg
        .resources()
        .iter()
        .any(|r| r.uri == "gaia://agent/gaia-local-researcher"));
}

#[test]
fn unsigned_mcp_is_rejected() {
    let reg = McpRegistry::local();
    let msg = McpMessage::unsigned("research.summarize", "{}");
    assert!(reg.invoke(&msg).unwrap_err().contains("unsigned"));
}

#[test]
fn signed_tool_invokes_from_intent() {
    let mut mem = MemOs::new();
    let g = IntentEngine::local_stub()
        .parse("research and summarize CARE", &mut mem)
        .unwrap();
    let reg = McpRegistry::local();
    let signer = IntentSigner::generate();
    let out = reg.invoke_from_intent(&g, &signer).unwrap();
    assert_eq!(out, "mcp-ok:research.summarize");
}

#[test]
fn discovery_stub_lists_without_network() {
    let reg = McpRegistry::local();
    let disc = DiscoveryStub;
    disc.advertise(&reg.list()[0]).unwrap();
    assert!(!disc.browse_local(&reg).is_empty());
}

#[test]
fn forged_mcp_token_is_rejected() {
    let reg = McpRegistry::local();
    let msg = McpMessage::marked_signed("research.summarize", "{}", "v0-token");
    assert!(reg.invoke(&msg).is_err());
}

#[test]
fn jsonrpc_session_lists_tools_and_resources() {
    let reg = McpRegistry::local();
    let signer = IntentSigner::generate();
    let tools = reg.handle(JsonRpcRequest::signed(&signer, 1, "tools/list", "{}"));
    assert!(tools.error.is_none());
    assert!(tools.result.unwrap().contains("research.summarize"));
    let resources = reg.handle(JsonRpcRequest::signed(&signer, 2, "resources/list", "{}"));
    assert!(resources.result.unwrap().contains("gaia://agent/gaia-local-researcher"));
}

#[test]
fn jsonrpc_tools_call_requires_signature() {
    let reg = McpRegistry::local();
    let unsigned = reg.handle(JsonRpcRequest::unsigned(3, "tools/call", "research.summarize"));
    assert!(unsigned.error.unwrap().contains("unsigned"));
    let signer = IntentSigner::generate();
    let ok = reg.handle(JsonRpcRequest::signed(
        &signer,
        4,
        "tools/call",
        "research.summarize",
    ));
    assert_eq!(ok.result.as_deref(), Some("mcp-ok:research.summarize"));
}
