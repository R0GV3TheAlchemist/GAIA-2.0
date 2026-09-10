use crate::intent::IntentGraph;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct McpTool {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AipManifest {
    pub name: String,
    pub version: String,
    pub tools: Vec<McpTool>,
}

/// JSON-RPC 2.0 envelope. Cryptographic verify is #19; presence is the v0 gate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct McpMessage {
    pub jsonrpc: String,
    pub method: String,
    pub params: String,
    pub signature: Option<String>,
}

impl McpMessage {
    pub fn unsigned(method: &str, params: &str) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            method: method.into(),
            params: params.into(),
            signature: None,
        }
    }

    pub fn marked_signed(method: &str, params: &str, token: &str) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            method: method.into(),
            params: params.into(),
            signature: Some(token.into()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct McpRegistry {
    pub agents: Vec<AipManifest>,
    /// Default policy: unsigned traffic rejected.
    pub reject_unsigned: bool,
}

impl McpRegistry {
    pub fn local() -> Self {
        Self {
            reject_unsigned: true,
            agents: vec![AipManifest {
                name: "gaia-local-researcher".into(),
                version: "0.1.0".into(),
                tools: vec![McpTool {
                    name: "research.summarize".into(),
                    description: "Local research+summarize stub".into(),
                }],
            }],
        }
    }

    pub fn list(&self) -> &[AipManifest] {
        &self.agents
    }

    pub fn admit(&self, msg: &McpMessage) -> Result<(), String> {
        if self.reject_unsigned && msg.signature.is_none() {
            return Err("unsigned MCP traffic rejected".into());
        }
        if msg.jsonrpc != "2.0" {
            return Err("jsonrpc 2.0 required".into());
        }
        Ok(())
    }

    pub fn invoke(&self, msg: &McpMessage) -> Result<String, String> {
        self.admit(msg)?;
        let found = self.agents.iter().any(|a| {
            a.tools.iter().any(|t| t.name == msg.method)
        });
        if !found {
            return Err("tool not in registry".into());
        }
        Ok(format!("mcp-ok:{}", msg.method))
    }

    /// Map an intent onto a registered MCP tool and invoke it.
    pub fn invoke_from_intent(&self, graph: &IntentGraph, signed_token: &str) -> Result<String, String> {
        let method = if graph.goal.to_ascii_lowercase().contains("research") {
            "research.summarize"
        } else {
            return Err("no MCP tool mapped for intent".into());
        };
        let msg = McpMessage::marked_signed(method, &graph.goal, signed_token);
        self.invoke(&msg)
    }
}

/// mDNS/DHT discovery is not on the wire in v0.
#[derive(Debug, Clone, Default)]
pub struct DiscoveryStub;

impl DiscoveryStub {
    pub fn advertise(&self, _manifest: &AipManifest) -> Result<(), String> {
        Ok(())
    }

    pub fn browse_local<'a>(&self, registry: &'a McpRegistry) -> &'a [AipManifest] {
        registry.list()
    }
}
