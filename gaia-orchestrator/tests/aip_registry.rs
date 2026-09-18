use gaia_orchestrator::{DiscoveryStub, JsonRpcRequest, McpMessage, McpRegistry};

#[test]
fn unsigned_mcp_is_rejected() {
    let mut reg = McpRegistry::local();
    let err = reg.invoke(&McpMessage::unsigned("tools/call", "research.summarize"));
    assert!(err.unwrap_err().contains("GAIA_UNAUTHENTICATED"));
    let rpc = reg.handle(JsonRpcRequest::unsigned(1, "tools/list", ""));
    assert!(rpc.error.unwrap().contains("GAIA_UNAUTHENTICATED"));
}

#[test]
fn local_registry_lists_aip_names_only() {
    let reg = McpRegistry::local();
    let names: Vec<_> = reg.list().iter().map(|a| a.name.as_str()).collect();
    assert!(names.contains(&"gaia-local-researcher"));
    assert!(reg.resources().iter().all(|r| r.endpoint.as_deref() != Some("tcp")));
    assert!(!reg.live_wire());
    assert!(DiscoveryStub.browse_mdns().is_err());
    assert_eq!(DiscoveryStub.browse_local(&reg).len(), reg.list().len());
}
