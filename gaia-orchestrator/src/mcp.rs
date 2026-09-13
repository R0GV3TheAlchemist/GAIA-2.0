use crate::intent::IntentGraph;
use crate::trust::{verify_tagged_signature, IntentSigner};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct McpTool {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct McpResource {
    pub uri: String,
    pub name: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AipManifest {
    pub name: String,
    pub version: String,
    pub tools: Vec<McpTool>,
    pub resources: Vec<McpResource>,
}

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

    pub fn signed_by(signer: &IntentSigner, method: &str, params: &str) -> Self {
        let payload = format!("{method}\n{params}");
        Self {
            jsonrpc: "2.0".into(),
            method: method.into(),
            params: params.into(),
            signature: Some(signer.sign_bytes(payload.as_bytes())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<String>,
    pub error: Option<String>,
}

impl JsonRpcRequest {
    pub fn signed(signer: &IntentSigner, id: u64, method: &str, params: &str) -> Self {
        let payload = format!("{method}\n{params}");
        Self {
            jsonrpc: "2.0".into(),
            id,
            method: method.into(),
            params: params.into(),
            signature: Some(signer.sign_bytes(payload.as_bytes())),
        }
    }

    pub fn unsigned(id: u64, method: &str, params: &str) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            method: method.into(),
            params: params.into(),
            signature: None,
        }
    }

    fn as_message(&self) -> McpMessage {
        McpMessage {
            jsonrpc: self.jsonrpc.clone(),
            method: self.method.clone(),
            params: self.params.clone(),
            signature: self.signature.clone(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct McpRegistry {
    pub agents: Vec<AipManifest>,
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
                resources: vec![McpResource {
                    uri: "gaia://agent/gaia-local-researcher".into(),
                    name: "gaia-local-researcher".into(),
                    kind: "agent".into(),
                }],
            }],
        }
    }

    pub fn live_wire(&self) -> bool {
        false
    }

    pub fn listen_tcp(&self, _bind: &str) -> Result<(), String> {
        Err("MCP is in-process only; no TCP listen".into())
    }

    pub fn list(&self) -> &[AipManifest] {
        &self.agents
    }

    pub fn tools(&self) -> Vec<&McpTool> {
        self.agents.iter().flat_map(|a| a.tools.iter()).collect()
    }

    pub fn resources(&self) -> Vec<&McpResource> {
        self.agents.iter().flat_map(|a| a.resources.iter()).collect()
    }

    pub fn admit(&self, msg: &McpMessage) -> Result<(), String> {
        if self.reject_unsigned && msg.signature.is_none() {
            return Err("unsigned MCP traffic rejected".into());
        }
        if msg.jsonrpc != "2.0" {
            return Err("jsonrpc 2.0 required".into());
        }
        if let Some(signature) = &msg.signature {
            let payload = format!("{}\n{}", msg.method, msg.params);
            verify_tagged_signature(payload.as_bytes(), signature)?;
        }
        Ok(())
    }

    fn dispatch_tool(&self, name: &str) -> Result<String, String> {
        if !self.tools().iter().any(|t| t.name == name) {
            return Err("tool not in registry".into());
        }
        Ok(format!("mcp-ok:{name}"))
    }

    pub fn invoke(&self, msg: &McpMessage) -> Result<String, String> {
        self.admit(msg)?;
        self.dispatch_tool(&msg.method)
    }

    pub fn invoke_from_intent(&self, graph: &IntentGraph, signer: &IntentSigner) -> Result<String, String> {
        let method = if graph.goal.to_ascii_lowercase().contains("research") {
            "research.summarize"
        } else {
            return Err("no MCP tool mapped for intent".into());
        };
        let msg = McpMessage::signed_by(signer, method, &graph.goal);
        self.invoke(&msg)
    }

    pub fn handle(&self, req: JsonRpcRequest) -> JsonRpcResponse {
        let fail = |error: String| JsonRpcResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: None,
            error: Some(error),
        };
        if let Err(error) = self.admit(&req.as_message()) {
            return fail(error);
        }
        let result = match req.method.as_str() {
            "initialize" => Ok("protocol=2026-07-28 transport=in-process".into()),
            "tools/list" => serde_json::to_string(&self.tools()).map_err(|e| e.to_string()),
            "resources/list" => serde_json::to_string(&self.resources()).map_err(|e| e.to_string()),
            "tools/call" => self.dispatch_tool(&req.params),
            _ => Err(format!("unknown method {}", req.method)),
        };
        match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: "2.0".into(),
                id: req.id,
                result: Some(value),
                error: None,
            },
            Err(error) => fail(error),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DiscoveryStub;

impl DiscoveryStub {
    pub fn advertise(&self, _manifest: &AipManifest) -> Result<(), String> {
        Ok(())
    }

    pub fn browse_local<'a>(&self, registry: &'a McpRegistry) -> &'a [AipManifest] {
        registry.list()
    }

    pub fn browse_mdns(&self) -> Result<(), String> {
        Err("mDNS/DHT not on the wire in v0".into())
    }
}
