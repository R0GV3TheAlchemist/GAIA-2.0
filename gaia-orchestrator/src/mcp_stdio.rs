//! MCP stdio framing for local JSON-RPC transport.
//! No TCP, network discovery, or unsigned dispatch.

use crate::mcp::{JsonRpcRequest, McpRegistry};
use std::io::{BufRead, Write};
use std::path::Path;

pub fn encode_line(json: &str) -> Result<String, String> {
    if json.contains('\n') {
        return Err("stdio frame must not contain embedded newlines".into());
    }
    Ok(format!("{json}\n"))
}

pub fn decode_line(line: &str) -> Result<String, String> {
    let trimmed = line.trim_end_matches(['\n', '\r']);
    if trimmed.is_empty() {
        return Err("empty stdio frame".into());
    }
    let _: serde_json::Value =
        serde_json::from_str(trimmed).map_err(|e| format!("stdio json: {e}"))?;
    Ok(trimmed.to_string())
}

pub fn serve_once<R: BufRead, W: Write>(
    registry: &mut McpRegistry,
    reader: &mut R,
    writer: &mut W,
) -> Result<(), String> {
    let mut line = String::new();
    let bytes = reader
        .read_line(&mut line)
        .map_err(|e| format!("stdio read: {e}"))?;
    if bytes == 0 {
        return Err("stdio eof".into());
    }

    let request_json = decode_line(&line)?;
    let request: JsonRpcRequest =
        serde_json::from_str(&request_json).map_err(|e| format!("stdio request: {e}"))?;
    let response_json = serde_json::to_string(&registry.handle(request))
        .map_err(|e| format!("stdio response: {e}"))?;
    let response_line = encode_line(&response_json)?;

    writer
        .write_all(response_line.as_bytes())
        .map_err(|e| format!("stdio write: {e}"))?;
    writer.flush().map_err(|e| format!("stdio flush: {e}"))?;
    Ok(())
}

pub fn persist_session(path: &Path, request_json: &str, response_json: &str) -> Result<(), String> {
    let mut f = std::fs::File::create(path).map_err(|e| format!("mcp session create: {e}"))?;
    write!(
        f,
        "{}{}",
        encode_line(request_json)?,
        encode_line(response_json)?,
    )
    .map_err(|e| format!("mcp session write: {e}"))?;
    Ok(())
}
