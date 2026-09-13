//! MCP stdio framing (spec 2026-07-28). Local files only. Not TCP. Not mDNS.

use std::io::Write;
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

pub fn persist_session(path: &Path, request_json: &str, response_json: &str) -> Result<(), String> {
    let mut f = std::fs::File::create(path).map_err(|e| format!("mcp session create: {e}"))?;
    write!(
        f,
        "{}{}",
        encode_line(request_json)?,
        encode_line(response_json)?
    )
    .map_err(|e| format!("mcp session write: {e}"))?;
    Ok(())
}
