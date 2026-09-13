//! #101 retrieval stub. Not Ollama, vLLM, or Qdrant.

use crate::{AikdError, Layer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub layer: Layer,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryHit {
    pub parametric: Span,
    pub retrieved: Span,
    pub used_network: bool,
}

impl QueryHit {
    pub fn offline(question: &str) -> Result<Self, AikdError> {
        if question.is_empty() {
            return Err(AikdError::CannotKnow);
        }
        Ok(Self {
            parametric: Span {
                layer: Layer::Weights,
                text: "parametric-fixture".into(),
            },
            retrieved: Span {
                layer: Layer::Retrieved,
                text: format!("bundled-corpus:{question}"),
            },
            used_network: false,
        })
    }
}
