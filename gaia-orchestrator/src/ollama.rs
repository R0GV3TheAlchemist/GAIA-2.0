//! Local Ollama only. No cloud. Not llama.cpp.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub fn generate(host: &str, model: &str, prompt: &str) -> Result<String, String> {
    let body = serde_json::json!({
        "model": model,
        "prompt": prompt,
        "stream": false
    })
    .to_string();
    let mut stream = TcpStream::connect(host)
        .map_err(|e| format!("ollama unavailable at {host}: {e}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(120)))
        .ok();
    stream
        .set_write_timeout(Some(Duration::from_secs(15)))
        .ok();
    let req = format!(
        "POST /api/generate HTTP/1.1\r\nHost: {host}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream
        .write_all(req.as_bytes())
        .map_err(|e| format!("ollama write: {e}"))?;
    let mut buf = String::new();
    stream
        .read_to_string(&mut buf)
        .map_err(|e| format!("ollama read: {e}"))?;
    let status = buf.lines().next().unwrap_or("");
    if !status.contains("200") {
        return Err(format!("ollama HTTP error: {status}"));
    }
    let json = buf.split("\r\n\r\n").nth(1).ok_or("ollama malformed HTTP")?;
    let v: serde_json::Value =
        serde_json::from_str(json.trim()).map_err(|e| format!("ollama json: {e}"))?;
    v.get("response")
        .and_then(|r| r.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "ollama missing response".into())
}
