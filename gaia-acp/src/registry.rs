//! Local AIP name list. No mDNS, DHT, or live MCP servers (#387).

use crate::types::ReasonCode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalAip {
    pub name: String,
    pub manifest_path: String,
}

#[derive(Debug, Default, Clone)]
pub struct LocalRegistry {
    entries: Vec<LocalAip>,
}

fn path_is_local(path: &str) -> bool {
    let p = path.to_ascii_lowercase();
    if p.starts_with("http://")
        || p.starts_with("https://")
        || p.starts_with("ws://")
        || p.starts_with("wss://")
        || p.contains("mdns")
        || p.contains("/dht/")
        || p.contains("dns-sd")
    {
        return false;
    }
    path.starts_with("./") || path.starts_with("gaia-") || path.starts_with("/")
}

impl LocalRegistry {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn register_local(&mut self, name: &str, manifest_path: &str) -> Result<(), ReasonCode> {
        if name.is_empty() || !path_is_local(manifest_path) {
            return Err(ReasonCode::ConfigRejected);
        }
        if self.entries.iter().any(|e| e.name == name) {
            return Ok(());
        }
        self.entries.push(LocalAip {
            name: name.into(),
            manifest_path: manifest_path.into(),
        });
        Ok(())
    }

    pub fn list_names(&self) -> Vec<String> {
        self.entries.iter().map(|e| e.name.clone()).collect()
    }
}
