//! RAG grounding enforcement (#932). Scaffold only — no LLM judge.

use serde::{Deserialize, Serialize};

/// Stable citation handle. String form of ingest `ChunkId` / document chunk id.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GenerationMode {
    Grounded,
    Ungrounded,
    Fallback,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroundedResponse {
    pub content: String,
    pub source_ids: Vec<ChunkId>,
    pub confidence: f32,
    pub generation_mode: GenerationMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroundingError {
    GroundingViolation,
    UngroundedOptInRequired,
}

impl GroundedResponse {
    pub fn grounded(
        content: String,
        source_ids: Vec<ChunkId>,
        confidence: f32,
    ) -> Result<Self, GroundingError> {
        if source_ids.is_empty() {
            return Err(GroundingError::GroundingViolation);
        }
        Ok(Self {
            content,
            source_ids,
            confidence,
            generation_mode: GenerationMode::Grounded,
        })
    }

    pub fn ungrounded(
        content: String,
        opt_in: bool,
        confidence: f32,
    ) -> Result<Self, GroundingError> {
        if !opt_in {
            return Err(GroundingError::UngroundedOptInRequired);
        }
        Ok(Self {
            content,
            source_ids: Vec::new(),
            confidence,
            generation_mode: GenerationMode::Ungrounded,
        })
    }
}

/// ACP-facing gate: text tools that set `grounding_required` must cite sources.
pub fn enforce_grounding(
    grounding_required: bool,
    source_ids: &[ChunkId],
) -> Result<(), GroundingError> {
    if grounding_required && source_ids.is_empty() {
        return Err(GroundingError::GroundingViolation);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grounding_required_empty_sources_is_violation() {
        let err = enforce_grounding(true, &[]).expect_err("empty sources must violate");
        assert_eq!(err, GroundingError::GroundingViolation);
    }

    #[test]
    fn grounded_constructor_rejects_empty_sources() {
        let err = GroundedResponse::grounded("hello".into(), vec![], 0.9)
            .expect_err("Grounded mode requires source_ids");
        assert_eq!(err, GroundingError::GroundingViolation);
    }

    #[test]
    fn ungrounded_requires_opt_in() {
        let err = GroundedResponse::ungrounded("hello".into(), false, 0.1)
            .expect_err("silent ungrounded is forbidden");
        assert_eq!(err, GroundingError::UngroundedOptInRequired);
    }
}
