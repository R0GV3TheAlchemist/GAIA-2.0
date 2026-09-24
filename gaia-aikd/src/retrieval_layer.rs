//! Retrieval-layer epistemic types and filtering — supports #958 (synthetic
//! sycophant adversarial test) and the broader epistemic-state layer from #953.
//!
//! A *sycophant* suppresses low-confidence or contradicted chunks so that the
//! final response sounds more confident than the evidence warrants.  The
//! retrieval layer must never do this: every chunk that enters the pipeline
//! exits with its epistemic metadata intact.

// ── Confidence ────────────────────────────────────────────────────────────────

/// Coarse confidence level attached to a retrieved knowledge chunk.
///
/// Intentionally kept as a three-level ordinal so that callers cannot silently
/// coerce `Medium` → `High` without a deliberate API call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Confidence {
    /// Strong evidence; multiple independent corroborating sources.
    High,
    /// Reasonable evidence; some uncertainty remains.
    Medium,
    /// Weak or single-source evidence; should be surfaced with a caveat.
    Low,
}

// ── EpistemicState ────────────────────────────────────────────────────────────

/// The epistemic standing of a knowledge chunk at retrieval time.
///
/// This enum is the heart of the anti-sycophancy guarantee: a `Contradicted`
/// chunk must *never* be silently dropped or upgraded — callers must decide
/// what to do with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpistemicState {
    /// Independently verified; multiple sources agree.
    Confirmed,
    /// Derived by inference; no direct source contradiction found.
    Inferred,
    /// At least one source explicitly contradicts this chunk.
    /// The `falsification_pointer` field in [`KnowledgeChunk`] names the
    /// source(s) responsible for the contradiction.
    Contradicted,
    /// Epistemic status cannot be determined from available evidence.
    Uncertain,
}

// ── KnowledgeChunk ────────────────────────────────────────────────────────────

/// A single unit of retrieved knowledge, annotated with epistemic metadata.
///
/// The struct is intentionally flat (no nested envelopes) so that serde
/// round-trips and test construction are as transparent as possible.
#[derive(Debug, Clone, PartialEq)]
pub struct KnowledgeChunk {
    /// Stable identifier for this chunk (e.g. a content hash or UUID).
    pub id: String,
    /// The raw content of the chunk.
    pub content: String,
    /// Coarse confidence level.
    pub confidence: Confidence,
    /// Epistemic standing at retrieval time.
    pub epistemic_state: EpistemicState,
    /// Ordered list of source identifiers that support this chunk.
    pub provenance: Vec<String>,
    /// If `epistemic_state == Contradicted`, this field names the conflicting
    /// source(s).  `None` for all other states.
    pub falsification_pointer: Option<String>,
}

// ── RetrievalPolicy ───────────────────────────────────────────────────────────

/// Controls which chunks the retrieval layer surfaces to downstream consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetrievalPolicy {
    /// Surface **all** chunks regardless of confidence or epistemic state.
    ///
    /// This is the safe, anti-sycophancy default: the caller — not the
    /// retrieval layer — decides what to do with contradicted or uncertain
    /// evidence.
    SurfaceAll,
    /// Drop chunks whose `epistemic_state` is `Contradicted`.
    ///
    /// **Use with extreme caution.**  This policy exists only for explicitly
    /// sandboxed contexts (e.g. a read-only display layer that must not show
    /// contested claims).  It must never be used to generate responses, as
    /// doing so constitutes a sycophancy violation under canon C210.
    SuppressContradicted,
}

// ── retrieval_filter ─────────────────────────────────────────────────────────

/// Filter `chunks` according to `policy` and return the surviving set.
///
/// Under [`RetrievalPolicy::SurfaceAll`] every chunk is returned unchanged —
/// this is the correct default that prevents sycophantic suppression.
///
/// Under [`RetrievalPolicy::SuppressContradicted`] chunks with
/// `epistemic_state == EpistemicState::Contradicted` are dropped.  See the
/// variant docs for the sharp edges on that policy.
///
/// Returns a `Vec<&KnowledgeChunk>` (borrowed slice) rather than cloning,
/// to keep the hot path allocation-free.
pub fn retrieval_filter(
    chunks: &[KnowledgeChunk],
    policy: RetrievalPolicy,
) -> Vec<&KnowledgeChunk> {
    match policy {
        RetrievalPolicy::SurfaceAll => chunks.iter().collect(),
        RetrievalPolicy::SuppressContradicted => chunks
            .iter()
            .filter(|c| c.epistemic_state != EpistemicState::Contradicted)
            .collect(),
    }
}

// ── EpistemicViolation ────────────────────────────────────────────────────────

/// Detectable violations of epistemic integrity in a [`KnowledgeChunk`].
///
/// Canon C210 (Citrine Tablet — calibrated confidence, not performed certainty)
/// and moral-architecture P3 (knowledge without pretending certainty) both
/// require that every asserted chunk carry at least one external, non-circular
/// provenance anchor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EpistemicViolation {
    /// The chunk's only provenance reference(s) point back to its own `id`
    /// — a circular self-citation that provides no external anchor.
    SelfGrounding,
    /// The chunk has an empty provenance list — an unfalsifiable bare assertion.
    MissingProvenance,
}

// ── epistemic_gate ────────────────────────────────────────────────────────────

/// Validate the epistemic integrity of a [`KnowledgeChunk`] before it enters
/// the retrieval pipeline.
///
/// # Returns
/// - `Ok(())` when the chunk has at least one provenance entry that is not
///   equal to the chunk's own `id`.
/// - `Err(EpistemicViolation::MissingProvenance)` when `provenance` is empty.
/// - `Err(EpistemicViolation::SelfGrounding)` when every provenance entry
///   equals `chunk.id` — the loop must be broken at ingestion, not at query
///   time.
///
/// Note: a chunk that mixes its own `id` with at least one distinct external
/// reference is considered valid; only the all-self-referential case is
/// rejected.
pub fn epistemic_gate(chunk: &KnowledgeChunk) -> Result<(), EpistemicViolation> {
    if chunk.provenance.is_empty() {
        return Err(EpistemicViolation::MissingProvenance);
    }
    if chunk.provenance.iter().all(|p| p == &chunk.id) {
        return Err(EpistemicViolation::SelfGrounding);
    }
    Ok(())
}

// ── unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn chunk(id: &str, state: EpistemicState, confidence: Confidence) -> KnowledgeChunk {
        KnowledgeChunk {
            id: id.into(),
            content: "test content".into(),
            confidence,
            epistemic_state: state,
            provenance: vec!["test-source".into()],
            falsification_pointer: None,
        }
    }

    #[test]
    fn surface_all_returns_everything() {
        let chunks = vec![
            chunk("a", EpistemicState::Confirmed, Confidence::High),
            chunk("b", EpistemicState::Contradicted, Confidence::Medium),
            chunk("c", EpistemicState::Inferred, Confidence::Low),
        ];
        let result = retrieval_filter(&chunks, RetrievalPolicy::SurfaceAll);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn suppress_contradicted_drops_only_contradicted() {
        let chunks = vec![
            chunk("a", EpistemicState::Confirmed, Confidence::High),
            chunk("b", EpistemicState::Contradicted, Confidence::Medium),
            chunk("c", EpistemicState::Inferred, Confidence::Low),
        ];
        let result = retrieval_filter(&chunks, RetrievalPolicy::SuppressContradicted);
        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|c| c.epistemic_state != EpistemicState::Contradicted));
    }

    #[test]
    fn empty_input_returns_empty() {
        let chunks: Vec<KnowledgeChunk> = vec![];
        assert!(retrieval_filter(&chunks, RetrievalPolicy::SurfaceAll).is_empty());
        assert!(retrieval_filter(&chunks, RetrievalPolicy::SuppressContradicted).is_empty());
    }

    #[test]
    fn confidence_is_not_mutated() {
        let chunks = vec![chunk("a", EpistemicState::Inferred, Confidence::Low)];
        let result = retrieval_filter(&chunks, RetrievalPolicy::SurfaceAll);
        assert_eq!(result[0].confidence, Confidence::Low);
    }

    // ── epistemic_gate unit tests ─────────────────────────────────────────────

    #[test]
    fn gate_rejects_empty_provenance() {
        let c = KnowledgeChunk {
            id: "x".into(),
            content: "c".into(),
            confidence: Confidence::High,
            epistemic_state: EpistemicState::Confirmed,
            provenance: vec![],
            falsification_pointer: None,
        };
        assert_eq!(epistemic_gate(&c), Err(EpistemicViolation::MissingProvenance));
    }

    #[test]
    fn gate_rejects_self_only_provenance() {
        let c = KnowledgeChunk {
            id: "loop-id".into(),
            content: "c".into(),
            confidence: Confidence::High,
            epistemic_state: EpistemicState::Confirmed,
            provenance: vec!["loop-id".into()],
            falsification_pointer: None,
        };
        assert_eq!(epistemic_gate(&c), Err(EpistemicViolation::SelfGrounding));
    }

    #[test]
    fn gate_passes_external_provenance() {
        let c = KnowledgeChunk {
            id: "c1".into(),
            content: "c".into(),
            confidence: Confidence::Medium,
            epistemic_state: EpistemicState::Inferred,
            provenance: vec!["external-doi-xyz".into()],
            falsification_pointer: Some("evidence of Y would falsify".into()),
        };
        assert!(epistemic_gate(&c).is_ok());
    }
}
