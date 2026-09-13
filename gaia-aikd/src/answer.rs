//! #95 answer envelope. Not Llama runtime, Qdrant, or conformal math.

use crate::{AikdError, Tier};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Weights,
    Retrieved,
    Tool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Answer {
    pub layer: Layer,
    pub tier: Tier,
    pub citations: Vec<String>,
    pub uncertainty: f64,
}

impl Answer {
    pub fn emit(layer: Layer, tier: Tier, citations: &[&str]) -> Result<Self, AikdError> {
        match tier {
            Tier::T5 => Err(AikdError::NeedVerify),
            Tier::T1 | Tier::T2 if citations.is_empty() => Err(AikdError::MissingCitation),
            _ => Ok(Self {
                layer,
                tier,
                citations: citations.iter().map(|s| (*s).to_string()).collect(),
                uncertainty: 0.5,
            }),
        }
    }
}
