use gaia_orchestrator::{
    decode_line, encode_line, persist_session, DiscoveryStub, IntentSigner, JsonRpcRequest,
    JsonRpcResponse, McpRegistry,
};
use std::io::Cursor;

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
fn stdio_signed_tool_call_dispatches_and_returns_one_response_line() {
    let mut registry = McpRegistry::local();
    let signer = IntentSigner::generate();
    let request = JsonRpcRequest::signed(&signer, 4, "tools/call", "research.summarize");
    let request_json = serde_json::to_string(&request).unwrap();

    let mut reader = Cursor::new(encode_line(&request_json).unwrap());
    let mut writer = Vec::new();
    registry.serve_once(&mut reader, &mut writer).unwrap();

    let response_line = String::from_utf8(writer).unwrap();
    assert_eq!(response_line.matches('\n').count(), 1);
    let response: JsonRpcResponse =
        serde_json::from_str(&decode_line(&response_line).unwrap()).unwrap();
    assert_eq!(response.id, 4);
    assert!(response.error.is_none());
    assert!(response.result.unwrap().contains("research.summarize"));
}

#[test]
fn stdio_unsigned_tool_call_returns_error_without_dispatch() {
    let mut registry = McpRegistry::local();
    let request = JsonRpcRequest::unsigned(3, "tools/call", "research.summarize");
    let request_json = serde_json::to_string(&request).unwrap();

    let mut reader = Cursor::new(encode_line(&request_json).unwrap());
    let mut writer = Vec::new();
    registry.serve_once(&mut reader, &mut writer).unwrap();

    let response_line = String::from_utf8(writer).unwrap();
    let response: JsonRpcResponse =
        serde_json::from_str(&decode_line(&response_line).unwrap()).unwrap();
    assert_eq!(response.id, 3);
    assert!(response.result.is_none());
    assert!(response.error.unwrap().contains("unsigned"));
    assert_eq!(registry.handler_invocations, 0);
}

#[test]
fn stdio_rejects_malformed_json_before_dispatch() {
    let mut registry = McpRegistry::local();
    let mut reader = Cursor::new("{not-json}\n");
    let mut writer = Vec::new();

    let error = registry.serve_once(&mut reader, &mut writer).unwrap_err();

    assert!(error.contains("stdio json"));
    assert!(writer.is_empty());
}

#[test]
fn durable_session_file_roundtrip_keeps_signature_gate() {
    let mut reg = McpRegistry::local();
    let unsigned = reg.handle(JsonRpcRequest::unsigned(
        3,
        "tools/call",
        "research.summarize",
    ));
    assert!(unsigned.error.as_ref().unwrap().contains("unsigned"));

    let signer = IntentSigner::generate();
    let req = JsonRpcRequest::signed(&signer, 4, "tools/call", "research.summarize");
    let resp = reg.handle(req.clone());
    assert!(resp.error.is_none());

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
