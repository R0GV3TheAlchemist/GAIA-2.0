//! #23 MCP stub: invoke from intent, AIP registry, reject unsigned.

use gaia_memos::MemOs;
use gaia_orchestrator::{DiscoveryStub, IntentEngine, McpMessage, McpRegistry};

#[test]
fn registry_lists_local_aip_manifests() {
    let reg = McpRegistry::local();
    assert!(reg.list().iter().any(|a| a.name == "gaia-local-researcher"));
    assert!(reg.list()[0].tools.iter().any(|t| t.name == "research.summarize"));
}

#[test]
fn unsigned_mcp_is_rejected() {
    let reg = McpRegistry::local();
    let msg = McpMessage::unsigned("research.summarize", "{}");
    assert!(reg.invoke(&msg).unwrap_err().contains("unsigned"));
}

#[test]
fn signed_tool_invokes_from_intent() {
    let mem = MemOs::new();
    let g = IntentEngine::local_stub()
        .parse("research and summarize CARE", &mem)
        .unwrap();
    let reg = McpRegistry::local();
    let out = reg.invoke_from_intent(&g, "v0-token").unwrap();
    assert_eq!(out, "mcp-ok:research.summarize");
}

#[test]
fn discovery_stub_lists_without_network() {
    let reg = McpRegistry::local();
    let disc = DiscoveryStub;
    disc.advertise(&reg.list()[0]).unwrap();
    assert!(!disc.browse_local(&reg).is_empty());
}
