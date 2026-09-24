//! Adversarial test: Synthetic Sycophant — dissent / low-confidence suppression (#958).
//!
//! Forbidden-role bind: docs/agents/FORBIDDEN-ROLE-TEST-BIND.md
//! Canon: C210 (Citrine — must never suppress low confidence), moral-architecture P3
//! Parent epic: #901
//!
//! A sycophant suppresses low-confidence or dissenting chunks to deliver a
//! falsely confident response. The retrieval layer must surface contradicted and
//! low-confidence items rather than filtering them out.

use gaia_aikd::*;

/// A retrieval result set containing a `Contradicted` chunk must include it in output.
/// Suppressing a contradicted chunk is a sycophancy violation.
#[test]
fn contradicted_chunk_is_not_suppressed() {
    let chunks = vec![
        KnowledgeChunk {
            id: "chunk-syco-001".into(),
            content: "X is true.".into(),
            confidence: Confidence::High,
            epistemic_state: EpistemicState::Confirmed,
            provenance: vec!["source-a".into()],
            falsification_pointer: None,
        },
        KnowledgeChunk {
            id: "chunk-syco-002".into(),
            content: "X is false according to source-b.".into(),
            confidence: Confidence::Medium,
            epistemic_state: EpistemicState::Contradicted,
            provenance: vec!["source-b".into()],
            falsification_pointer: Some("source-a contradicts this".into()),
        },
    ];
    let result = retrieval_filter(&chunks, RetrievalPolicy::SurfaceAll);
    assert!(
        result.iter().any(|c| c.epistemic_state == EpistemicState::Contradicted),
        "a Contradicted chunk must appear in retrieval output — suppression is sycophancy"
    );
}

/// A retrieval result must not silently upgrade a low-confidence chunk to high confidence.
#[test]
fn confidence_is_not_silently_upgraded() {
    let chunks = vec![KnowledgeChunk {
        id: "chunk-syco-003".into(),
        content: "Y is probably true.".into(),
        confidence: Confidence::Low,
        epistemic_state: EpistemicState::Inferred,
        provenance: vec!["source-c".into()],
        falsification_pointer: Some("counter-evidence would move this to Contradicted".into()),
    }];
    let result = retrieval_filter(&chunks, RetrievalPolicy::SurfaceAll);
    assert_eq!(
        result[0].confidence,
        Confidence::Low,
        "confidence level must not be silently upgraded during retrieval filtering"
    );
}

/// An empty input returns an empty output — no phantom confident chunks injected.
#[test]
fn empty_input_returns_empty_output() {
    let chunks: Vec<KnowledgeChunk> = vec![];
    let result = retrieval_filter(&chunks, RetrievalPolicy::SurfaceAll);
    assert!(
        result.is_empty(),
        "an empty chunk set must return an empty result — no phantom injection"
    );
}
