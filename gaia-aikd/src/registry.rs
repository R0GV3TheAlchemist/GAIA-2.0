//! #100 open registry. Published closed scores are reference only.

use crate::AikdError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchRow {
    pub model: String,
    pub published: bool,
    pub gaia_measured: bool,
}

impl BenchRow {
    pub fn local_open() -> Self {
        Self {
            model: "local-llama-fixture".into(),
            published: false,
            gaia_measured: false,
        }
    }

    pub fn closed_reference(name: &str) -> Result<Self, AikdError> {
        if name.to_ascii_lowercase().contains("gpt") || name.to_ascii_lowercase().contains("claude")
        {
            return Ok(Self {
                model: name.into(),
                published: true,
                gaia_measured: false,
            });
        }
        Ok(Self {
            model: name.into(),
            published: true,
            gaia_measured: false,
        })
    }

    pub fn claim_closed_as_measured(name: &str) -> Result<Self, AikdError> {
        if name.to_ascii_lowercase().contains("gpt") || name.to_ascii_lowercase().contains("claude")
        {
            return Err(AikdError::ClosedScoreClaim);
        }
        Err(AikdError::ClosedScoreClaim)
    }
}

pub fn system_tags() -> [&'static str; 19] {
    [
        "agentic",
        "xai",
        "embodied",
        "multimodal",
        "rag",
        "tool-use",
        "code",
        "math",
        "science",
        "medical-ref",
        "legal-ref",
        "speech",
        "vision",
        "planning",
        "memory",
        "eval",
        "open-weight",
        "on-device",
        "sandbox",
    ]
}
