use serde::{Deserialize, Serialize};

use crate::types::ReasonCode;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct McpServerConfig {
    pub server_id: String,
    pub publisher: String,
    pub command: String,
    pub image: String,
    pub version: String,
    pub reviewed: bool,
    pub env: Vec<String>,
    pub shell_wrapper: bool,
}

impl McpServerConfig {
    pub fn example_safe() -> Self {
        Self {
            server_id: "local.fake.parse".into(),
            publisher: "gaia-reviewed".into(),
            command: "/opt/gaia/fake-mcp".into(),
            image: "ghcr.io/gaia/fake-mcp".into(),
            version: "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
            reviewed: true,
            env: vec![],
            shell_wrapper: false,
        }
    }
}

pub fn lint_mcp_config(cfg: &McpServerConfig) -> Result<(), ReasonCode> {
    if !cfg.reviewed {
        return Err(ReasonCode::ConfigRejected);
    }
    if cfg.shell_wrapper
        || cfg.command.contains("sh -c")
        || cfg.command.contains("/bin/sh")
        || cfg.command.contains("bash -c")
    {
        return Err(ReasonCode::ConfigRejected);
    }
    if cfg.version == "latest" || cfg.image.ends_with(":latest") {
        return Err(ReasonCode::ConfigRejected);
    }
    if !cfg.version.starts_with("sha256:") {
        return Err(ReasonCode::ConfigRejected);
    }
    let secretish = ["TOKEN", "SECRET", "PASSWORD", "API_KEY"];
    if cfg
        .env
        .iter()
        .any(|e| secretish.iter().any(|s| e.to_ascii_uppercase().contains(s)))
    {
        return Err(ReasonCode::ConfigRejected);
    }
    Ok(())
}
