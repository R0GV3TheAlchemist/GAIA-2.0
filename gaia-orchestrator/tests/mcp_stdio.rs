use gaia_orchestrator::{decode_line, encode_line, persist_session, DiscoveryStub, IntentSigner, JsonRpcRequest, McpRegistry};

#[test]
fn stdio_frame_is_single_newline() {
    let line = encode_line(r#"{"jsonrpc":"2.0"}"#).unwrap();
    assert!(line.ends_with('\n'));
    assert_eq!(line.matches('\n').count(), 1);
    assert!(encode_line("{\n}").is_err());
}

#[test]
fn tcp_listen_stays_refused() {
    let reg = McpRegistry::local();
    assert!(!reg.live_wire());
    assert!(reg.listen_tcp("127.0.0.1:0").is_err());
    assert!(DiscoveryStub.browse_mdns().is_err());
}

#[test]
fn durable_session_file_roundtrip_keeps_signature_gate() {
    let reg = McpRegistry::local();
    let unsigned = reg.handle(JsonRpcRequest::unsigned(3, "tools/call", "research.summarize"));
    assert!(unsigned.error.as_ref().unwrap().contains("unsigned"));

    let signer = IntentSigner::generate();
    let req = JsonRpcRequest::signed(&signer, 4, "tools/call", "research.summarize");
    let resp = reg.handle(req.clone());
    assert!(resp.error.is_none());
    assert!(resp.result.as_ref().unwrap().contains("research.summarize") || resp.result.as_ref().unwrap().contains("mcp-ok"));

    let req_json = serde_json::to_string(&req).unwrap();
    let resp_json = serde_json::to_string(&resp).unwrap();
    let path = std::env::temp_dir().join("gaia-mcp-session.ndjson");
    persist_session(&path, &req_json, &resp_json).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let mut lines = text.lines();
    let first = decode_line(lines.next().unwrap()).unwrap();
    let second = decode_line(lines.next().unwrap()).unwrap();
    assert!(first.contains("tools/call"));
    assert!(second.contains("jsonrpc"));
    let _ = std::fs::remove_file(&path);
}
