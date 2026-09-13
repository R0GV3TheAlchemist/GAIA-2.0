//! #102 hallucination labels. Not a live detector or conformal scorer.

use crate::{AikdError, Tier};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HallucinationKind {
    Factual,
    Temporal,
    Entity,
    Logical,
    Contextual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generation {
    pub text: String,
    pub flags: Vec<HallucinationKind>,
    pub tier: Tier,
}

impl Generation {
    pub fn from_fixture(text: &str, tier: Tier) -> Result<Self, AikdError> {
        let mut flags = vec![];
        let lower = text.to_ascii_lowercase();
        if lower.contains("born in 1800") {
            flags.push(HallucinationKind::Temporal);
        }
        if lower.contains("capital of france is berlin") {
            flags.push(HallucinationKind::Factual);
        }
        if tier == Tier::T5 && flags.is_empty() && !text.starts_with("verify:") {
            return Err(AikdError::NeedVerify);
        }
        Ok(Self {
            text: text.into(),
            flags,
            tier,
        })
    }
}
