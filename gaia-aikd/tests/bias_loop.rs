//! Adversarial test: Algorithm of Bias — self-grounding loop negative (#958).
//!
//! Forbidden-role bind: docs/agents/FORBIDDEN-ROLE-TEST-BIND.md
//! Canon: C210 (Citrine Tablet — calibrated confidence, not performed certainty)
//!        moral-architecture P3 (knowledge without pretending certainty)
//! Parent epic: #901
//!
//! A self-grounding loop occurs when a retrieval result cites itself as its own
//! primary source — circular provenance. The system must detect and reject it.

use gaia_aikd::*;

/// A chunk whose only provenance reference is its own chunk_id is a self-grounding loop.
/// The epistemic gate must return `Err(EpistemicViolation::SelfGrounding)`.
#[test]
fn self_grounding_chunk_is_rejected() {
    let chunk_id = "chunk-bias-001";
    let chunk = KnowledgeChunk {
        id: chunk_id.into(),
        content: "The answer is definitively X.".into(),
        confidence: Confidence::High,
        epistemic_state: EpistemicState::Confirmed,
        // Provenance points only to itself — the loop.
        provenance: vec![chunk_id.into()],
        falsification_pointer: None,
    };
    assert_eq!(
        epistemic_gate(&chunk),
        Err(EpistemicViolation::SelfGrounding),
        "a chunk whose sole provenance is its own id must be rejected as a self-grounding loop"
    );
}

/// A chunk with an external provenance reference and a falsification pointer passes.
#[test]
fn grounded_chunk_passes() {
    let chunk = KnowledgeChunk {
        id: "chunk-bias-002".into(),
        content: "The answer is likely X, confidence 0.82.".into(),
        confidence: Confidence::Medium,
        epistemic_state: EpistemicState::Inferred,
        provenance: vec!["source-external-paper-doi-12345".into()],
        falsification_pointer: Some("evidence of Y would change this to Contradicted".into()),
    };
    assert!(
        epistemic_gate(&chunk).is_ok(),
        "a properly grounded inferred chunk with a falsification pointer must pass"
    );
}

/// An empty provenance list is also a self-grounding violation (no external anchor at all).
#[test]
fn empty_provenance_is_rejected() {
    let chunk = KnowledgeChunk {
        id: "chunk-bias-003".into(),
        content: "Fact: the sky is green.".into(),
        confidence: Confidence::High,
        epistemic_state: EpistemicState::Confirmed,
        provenance: vec![], // no provenance at all
        falsification_pointer: None,
    };
    assert_eq!(
        epistemic_gate(&chunk),
        Err(EpistemicViolation::MissingProvenance),
        "an empty provenance list must be rejected"
    );
}
