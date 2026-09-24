//! Integration tests for the epistemic state layer (#953).
//!
//! These tests exercise [`EpistemicState`] as it is wired into
//! [`DocumentChunk`] — the unit that flows through the ingest pipeline.
//! They complement the unit tests inside `epistemic.rs` and `document.rs`
//! by verifying the full public API surface and cross-module interactions.
//!
//! Coverage areas:
//! - Retrieval eligibility gate (C30: `Retracted` MUST NOT surface)
//! - Internal consistency rules for all [`ClaimStatus`] variants
//! - Serde round-trip of [`DocumentChunk`] with and without `epistemic_state`
//! - [`EpistemicConfidence`] boundary enforcement
//! - [`ContradictionRef`] construction and multi-ref accumulation
//! - [`EpistemicStateBuilder`] default safety guarantees
//! - [`classify_document_chunk`] does not clobber an existing `epistemic_state`
//!
//! Forbidden-role bind: docs/agents/FORBIDDEN-ROLE-TEST-BIND.md
//! Canon: C30 (no silent failures), C156 (KG + memory taxonomy)
//! Parent epic: #901

use gaia_ingest::{
    document::{AccessTier, ConfidenceTier, DocumentKind},
    epistemic::{
        ClaimStatus, ContradictionRef, EpistemicConfidence, EpistemicError,
        EpistemicState, EpistemicStateBuilder, EvidenceKind,
    },
    lexicon::{classify_document_chunk, LexiconPlane},
    provenance::ProvenanceBuilder,
    schema::DataSource,
    DocumentChunk,
};
use std::collections::BTreeMap;

// ── helpers ───────────────────────────────────────────────────────────────────

/// Build a minimal valid [`DocumentChunk`] with no epistemic state.
fn base_chunk() -> DocumentChunk {
    let text = "The global mean surface temperature has risen by 1.1 \u{00b0}C since 1850.".to_string();
    let char_count = text.chars().count();
    let receipt = ProvenanceBuilder::new(
        DataSource::InternalDocument,
        "gaia://test/source",
        "integration-test-0",
        1_700_000_001,
        1_700_000_000,
    )
    .seal(text.as_bytes())
    .expect("valid receipt");
    DocumentChunk {
        id: "00000000-0000-0000-0000-000000000099".into(),
        document_title: "Integration Test Document".into(),
        document_uri: "gaia://test/source".into(),
        kind: DocumentKind::CanonTablet,
        text,
        char_count,
        chunk_index: 0,
        total_chunks: 1,
        domain: "test".into(),
        language: "en".into(),
        authored_at_unix: Some(1_700_000_000),
        ttl_seconds: None,
        confidence: ConfidenceTier::Verified,
        access_tier: AccessTier::Public,
        access_control: Vec::new(),
        attributes: BTreeMap::new(),
        lexicon_plane: LexiconPlane::Bridge,
        lexicon_voice: None,
        provenance: receipt,
        embedding: None,
        artifact: None,
        epistemic_state: None,
    }
}

/// Build a minimal [`EpistemicState`] for a given [`ClaimStatus`] and confidence.
fn make_state(status: ClaimStatus, confidence: f32) -> EpistemicState {
    EpistemicStateBuilder::new()
        .claim_status(status)
        .evidence_kind(EvidenceKind::ModelOutput)
        .confidence(confidence)
        .build()
        .expect("valid state")
}

// ── retrieval eligibility gate (C30) ─────────────────────────────────────────

/// A chunk with no epistemic state must be retrieval-eligible.
/// The classify step has not yet run; absence is legal.
#[test]
fn no_epistemic_state_is_retrieval_eligible() {
    let chunk = base_chunk();
    assert!(chunk.epistemic_state.is_none());
    assert!(
        chunk.is_retrieval_eligible(),
        "chunk with no epistemic state must be retrieval-eligible"
    );
}

/// Asserted claim — the default for freshly ingested content — must pass.
#[test]
fn asserted_claim_is_retrieval_eligible() {
    let mut chunk = base_chunk();
    chunk.epistemic_state = Some(make_state(ClaimStatus::Asserted, 0.7));
    assert!(chunk.is_retrieval_eligible());
}

/// Disputed claim must still reach the retrieval layer (human review decides).
#[test]
fn disputed_claim_is_retrieval_eligible() {
    let mut chunk = base_chunk();
    chunk.epistemic_state = Some(
        EpistemicStateBuilder::new()
            .claim_status(ClaimStatus::Disputed)
            .evidence_kind(EvidenceKind::HumanReport)
            .confidence(0.4)
            .contradiction(ContradictionRef::new("gaia://test/other", 0))
            .build()
            .expect("valid"),
    );
    assert!(
        chunk.is_retrieval_eligible(),
        "Disputed chunks must surface for human review, not be silently suppressed"
    );
}

/// Corroborated claim — highest epistemic weight — must pass.
#[test]
fn corroborated_claim_is_retrieval_eligible() {
    let mut chunk = base_chunk();
    chunk.epistemic_state = Some(
        EpistemicStateBuilder::new()
            .claim_status(ClaimStatus::Corroborated)
            .evidence_kind(EvidenceKind::LiteratureCitation)
            .confidence(0.95)
            .grounding_uri("https://doi.org/10.1038/s41558-023-01799-8")
            .build()
            .expect("valid"),
    );
    assert!(chunk.is_retrieval_eligible());
}

/// Retracted chunk MUST NOT be retrieval-eligible (C30: no silent failures).
#[test]
fn retracted_claim_is_not_retrieval_eligible() {
    let mut chunk = base_chunk();
    chunk.epistemic_state = Some(make_state(ClaimStatus::Retracted, 0.0));
    assert!(
        !chunk.is_retrieval_eligible(),
        "C30: Retracted chunk must never surface to the retrieval layer"
    );
}

// ── internal consistency rules ────────────────────────────────────────────────

/// Asserted with no extra refs is internally consistent.
#[test]
fn asserted_no_refs_is_consistent() {
    let s = make_state(ClaimStatus::Asserted, 0.7);
    assert!(s.is_internally_consistent());
}

/// Retracted with zero confidence is internally consistent.
#[test]
fn retracted_zero_confidence_is_consistent() {
    let s = make_state(ClaimStatus::Retracted, 0.0);
    assert!(s.is_internally_consistent());
}

/// Disputed without ContradictionRefs is *inconsistent*:
/// the pipeline must have set at least one ref when promoting to Disputed.
#[test]
fn disputed_without_contradiction_refs_is_inconsistent() {
    let s = make_state(ClaimStatus::Disputed, 0.4);
    assert!(
        !s.is_internally_consistent(),
        "Disputed with no ContradictionRefs violates the consistency rule"
    );
}

/// Disputed with at least one ContradictionRef is consistent.
#[test]
fn disputed_with_contradiction_ref_is_consistent() {
    let s = EpistemicStateBuilder::new()
        .claim_status(ClaimStatus::Disputed)
        .evidence_kind(EvidenceKind::HumanReport)
        .confidence(0.35)
        .contradiction(ContradictionRef::new("gaia://canon/rival-doc", 2))
        .build()
        .expect("valid");
    assert!(s.is_internally_consistent());
}

/// Corroborated without grounding URIs is *inconsistent*.
#[test]
fn corroborated_without_grounding_uris_is_inconsistent() {
    let s = make_state(ClaimStatus::Corroborated, 0.9);
    assert!(
        !s.is_internally_consistent(),
        "Corroborated with no grounding URIs violates the consistency rule"
    );
}

/// Corroborated with at least one grounding URI is consistent.
#[test]
fn corroborated_with_grounding_uri_is_consistent() {
    let s = EpistemicStateBuilder::new()
        .claim_status(ClaimStatus::Corroborated)
        .evidence_kind(EvidenceKind::LiteratureCitation)
        .confidence(0.93)
        .grounding_uri("https://doi.org/10.1038/s41558-023-01799-8")
        .build()
        .expect("valid");
    assert!(s.is_internally_consistent());
}

// ── EpistemicConfidence boundaries ───────────────────────────────────────────

#[test]
fn confidence_zero_is_valid() {
    assert!(EpistemicConfidence::new(0.0).is_ok());
}

#[test]
fn confidence_one_is_valid() {
    assert!(EpistemicConfidence::new(1.0).is_ok());
}

#[test]
fn confidence_just_above_one_is_rejected() {
    let err = EpistemicConfidence::new(1.001);
    assert!(
        matches!(err, Err(EpistemicError::ConfidenceOutOfRange(_))),
        "value > 1.0 must be rejected"
    );
}

#[test]
fn confidence_negative_is_rejected() {
    assert!(EpistemicConfidence::new(-0.001).is_err());
}

#[test]
fn confidence_nan_is_rejected() {
    assert!(EpistemicConfidence::new(f32::NAN).is_err());
}

#[test]
fn confidence_infinity_is_rejected() {
    assert!(EpistemicConfidence::new(f32::INFINITY).is_err());
    assert!(EpistemicConfidence::new(f32::NEG_INFINITY).is_err());
}

// ── ContradictionRef ──────────────────────────────────────────────────────────

#[test]
fn contradiction_ref_fields_are_preserved() {
    let r = ContradictionRef::new("gaia://rival/doc", 7);
    assert_eq!(r.document_uri, "gaia://rival/doc");
    assert_eq!(r.chunk_index, 7);
    assert!(r.note.is_none());
}

#[test]
fn contradiction_ref_with_note_is_preserved() {
    let r = ContradictionRef::with_note(
        "gaia://rival/doc",
        3,
        "Conflicting sea-level rise estimate",
    );
    assert_eq!(r.note.as_deref(), Some("Conflicting sea-level rise estimate"));
}

/// Multiple ContradictionRefs can be accumulated on a single state.
#[test]
fn multi_contradiction_refs_accumulate() {
    let s = EpistemicStateBuilder::new()
        .claim_status(ClaimStatus::Disputed)
        .evidence_kind(EvidenceKind::ModelOutput)
        .confidence(0.3)
        .contradiction(ContradictionRef::new("gaia://rival/doc-a", 0))
        .contradiction(ContradictionRef::new("gaia://rival/doc-b", 1))
        .build()
        .expect("valid");
    assert_eq!(s.contradictions.len(), 2);
    assert!(s.is_internally_consistent());
}

// ── builder defaults ──────────────────────────────────────────────────────────

/// The builder's safe defaults must produce a consistent, retrievable state.
#[test]
fn builder_defaults_produce_safe_state() {
    let s = EpistemicStateBuilder::new()
        .build()
        .expect("default build must succeed");
    assert_eq!(s.claim_status, ClaimStatus::Asserted);
    assert_eq!(s.evidence_kind, EvidenceKind::Synthetic);
    assert!((s.confidence.value() - 0.5_f32).abs() < f32::EPSILON);
    assert!(s.contradictions.is_empty());
    assert!(s.grounding_uris.is_empty());
    assert!(s.rationale.is_none());
    assert!(s.is_internally_consistent());
}

/// A builder with out-of-range confidence must fail at build() time, not panic.
#[test]
fn builder_rejects_out_of_range_confidence() {
    let result = EpistemicStateBuilder::new().confidence(2.0).build();
    assert!(result.is_err(), "confidence > 1.0 must be rejected at build time");
}

// ── serde round-trip through DocumentChunk ────────────────────────────────────

/// A DocumentChunk with no epistemic state serialises without the key and
/// deserialises back to the same value.
#[test]
fn document_chunk_serde_no_epistemic_state() {
    let chunk = base_chunk();
    let json = serde_json::to_string(&chunk).expect("serialize");
    assert!(
        !json.contains("epistemic_state"),
        "epistemic_state key must be omitted when None (compact payload)"
    );
    let back: DocumentChunk = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(chunk.id, back.id);
    assert!(back.epistemic_state.is_none());
    assert!(back.is_valid());
}

/// A DocumentChunk with a full epistemic state round-trips without data loss.
#[test]
fn document_chunk_serde_with_epistemic_state() {
    let mut chunk = base_chunk();
    chunk.epistemic_state = Some(
        EpistemicStateBuilder::new()
            .claim_status(ClaimStatus::Corroborated)
            .evidence_kind(EvidenceKind::LiteratureCitation)
            .confidence(0.92)
            .grounding_uri("https://doi.org/10.1038/s41558-023-01799-8")
            .rationale("IPCC AR6 WG1 Chapter 2")
            .build()
            .expect("valid"),
    );
    let json = serde_json::to_string(&chunk).expect("serialize");
    assert!(json.contains("epistemic_state"), "key must be present when Some");
    let back: DocumentChunk = serde_json::from_str(&json).expect("deserialize");
    // Borrow epistemic_state by reference so `back` is not partially moved
    // and remains usable for the is_valid() / is_retrieval_eligible() checks below.
    let es = back
        .epistemic_state
        .as_ref()
        .expect("epistemic_state must survive round-trip");
    assert_eq!(es.claim_status, ClaimStatus::Corroborated);
    assert_eq!(es.evidence_kind, EvidenceKind::LiteratureCitation);
    assert!((es.confidence.value() - 0.92_f32).abs() < 0.001_f32);
    assert_eq!(es.grounding_uris.len(), 1);
    assert_eq!(es.rationale.as_deref(), Some("IPCC AR6 WG1 Chapter 2"));
    assert!(back.is_valid());
    assert!(back.is_retrieval_eligible());
}

/// A Retracted chunk round-trips and remains retrieval-ineligible after
/// deserialisation — the gate must survive a storage/retrieval cycle.
#[test]
fn retracted_chunk_remains_ineligible_after_serde() {
    let mut chunk = base_chunk();
    chunk.epistemic_state = Some(make_state(ClaimStatus::Retracted, 0.0));
    let json = serde_json::to_string(&chunk).expect("serialize");
    let back: DocumentChunk = serde_json::from_str(&json).expect("deserialize");
    assert!(
        !back.is_retrieval_eligible(),
        "Retracted chunk must remain ineligible after a serde round-trip"
    );
}

// ── classify_document_chunk interop ──────────────────────────────────────────

/// classify_document_chunk() populates lexicon_plane and lexicon_voice but
/// must not clobber an epistemic_state that was already set by a prior step.
#[test]
fn classify_does_not_clobber_existing_epistemic_state() {
    let mut chunk = base_chunk();
    let existing = EpistemicStateBuilder::new()
        .claim_status(ClaimStatus::Corroborated)
        .evidence_kind(EvidenceKind::LiteratureCitation)
        .confidence(0.95)
        .grounding_uri("https://doi.org/10.1038/s41558-023-01799-8")
        .build()
        .expect("valid");
    chunk.epistemic_state = Some(existing.clone());

    classify_document_chunk(&mut chunk);

    let _ = chunk.lexicon_plane;

    let es = chunk
        .epistemic_state
        .expect("classify must not clear epistemic_state");
    assert_eq!(
        es.claim_status,
        ClaimStatus::Corroborated,
        "classify_document_chunk must not overwrite claim_status"
    );
    assert_eq!(
        es.evidence_kind,
        EvidenceKind::LiteratureCitation,
        "classify_document_chunk must not overwrite evidence_kind"
    );
    assert!(
        (es.confidence.value() - 0.95_f32).abs() < 0.001_f32,
        "classify_document_chunk must not alter confidence"
    );
}

/// classify_document_chunk() on a chunk with no epistemic state must leave
/// epistemic_state as None — the classify step only touches lexicon fields.
#[test]
fn classify_does_not_inject_epistemic_state() {
    let mut chunk = base_chunk();
    assert!(chunk.epistemic_state.is_none());
    classify_document_chunk(&mut chunk);
    assert!(
        chunk.epistemic_state.is_none(),
        "classify_document_chunk must not inject an epistemic_state where none existed"
    );
}

// ── ClaimStatus ordering ──────────────────────────────────────────────────────

/// Statuses are ordered Retracted < Disputed < Asserted < Corroborated.
/// The ordering is used by downstream retrieval scorers to rank chunks.
#[test]
fn claim_status_ordering_is_correct() {
    assert!(ClaimStatus::Retracted  < ClaimStatus::Disputed);
    assert!(ClaimStatus::Disputed   < ClaimStatus::Asserted);
    assert!(ClaimStatus::Asserted   < ClaimStatus::Corroborated);
}

// ── EpistemicError display ────────────────────────────────────────────────────

#[test]
fn epistemic_error_display_contains_value() {
    let e = EpistemicError::ConfidenceOutOfRange(1.42);
    let s = e.to_string();
    assert!(s.contains("1.42"), "error message must echo the offending value");
    assert!(s.contains("[0.0, 1.0]"), "error message must state the valid range");
}
