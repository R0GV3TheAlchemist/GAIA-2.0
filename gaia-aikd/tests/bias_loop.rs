//! Adversarial test: Algorithm of Bias — self-grounding loop (#958).
//!
//! Forbidden-role bind: docs/agents/FORBIDDEN-ROLE-TEST-BIND.md
//! Canon: C210, moral-architecture P3
//! Parent epic: #901

use gaia_aikd::*;

#[test]
fn self_grounding_chunk_is_rejected() {
    let chunk_id = "chunk-bias-001";
    let chunk = KnowledgeChunk {
        id: chunk_id.into(),
        content: "The answer is definitively X.".into(),
        confidence: Confidence::High,
        epistemic_state: EpistemicState::Confirmed,
        provenance: vec![chunk_id.into()],
        falsification_pointer: None,
    };
    assert_eq!(
        epistemic_gate(&chunk),
        Err(EpistemicViolation::SelfGrounding)
    );
}

#[test]
fn grounded_chunk_passes() {
    let chunk = KnowledgeChunk {
        id: "chunk-bias-002".into(),
        content: "The answer is likely X.".into(),
        confidence: Confidence::Medium,
        epistemic_state: EpistemicState::Inferred,
        provenance: vec!["source-external-paper-doi-12345".into()],
        falsification_pointer: Some("evidence of Y would change this".into()),
    };
    assert!(epistemic_gate(&chunk).is_ok());
}

#[test]
fn empty_provenance_is_rejected() {
    let chunk = KnowledgeChunk {
        id: "chunk-bias-003".into(),
        content: "Fact: the sky is green.".into(),
        confidence: Confidence::High,
        epistemic_state: EpistemicState::Confirmed,
        provenance: vec![],
        falsification_pointer: None,
    };
    assert_eq!(
        epistemic_gate(&chunk),
        Err(EpistemicViolation::MissingProvenance)
    );
}
