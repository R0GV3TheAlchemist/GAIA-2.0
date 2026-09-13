//! #97 tools + sandbox. World-model claims are simulated.

use crate::AikdError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolResult {
    pub tool: String,
    pub provenance: String,
    pub signed: bool,
}

impl ToolResult {
    pub fn run(tool: &str, signed: bool) -> Result<Self, AikdError> {
        if !signed {
            return Err(AikdError::NeedVerify);
        }
        Ok(Self {
            tool: tool.into(),
            provenance: "agentic-fixture".into(),
            signed: true,
        })
    }
}

pub fn computer_use(escape_host: bool) -> Result<&'static str, AikdError> {
    if escape_host {
        return Err(AikdError::NeedVerify);
    }
    Ok("sandboxed")
}

pub fn world_model_claim(text: &str) -> String {
    format!("simulated: {text}")
}
