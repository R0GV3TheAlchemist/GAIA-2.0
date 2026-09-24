//! Epistemic state layer for GAIA-2.0 document chunks.
//!
//! Every [`DocumentChunk`] can carry an [`EpistemicState`] that answers
//! four questions about the knowledge it contains:
//!
//! 1. **Claim** — what epistemic status does this passage hold?
//!    (`Asserted`, `Corroborated`, `Disputed`, `Retracted`)
//! 2. **Evidence** — what kind of evidence grounds it?
//!    (`DirectObservation`, `ModelOutput`, `HumanReport`, …)
//! 3. **Confidence** — how confident is the ingestion pipeline in the
//!    claim? A float in \[0.0, 1.0\].
//! 4. **Contradiction** — does this chunk conflict with another?
//!    Zero or more [`ContradictionRef`]s pointing at conflicting chunks.
//!
//! A fifth cross-cutting concern — **Provenance** — is handled by the
//! existing [`crate::provenance::ProvenanceReceipt`] on `DocumentChunk`.
//! `EpistemicState` does not duplicate provenance fields; it links to them.
//!
//! ## Design contract
//!
//! - `epistemic_state` is `None` immediately after chunking.  The classify
//!   step (same pipeline stage as `classify_document_chunk`) populates it.
//! - `None` is **not** an error: canon tablets ingested before the classify
//!   step has run are valid `DocumentChunk`s.
//! - A chunk with `ClaimStatus::Retracted` MUST NOT be returned by the
//!   retrieval layer.  Enforcement is in `gaia-memos`.
//! - `EpistemicConfidence` rejects NaN and out-of-range values at
//!   construction time — there is no silent default.
//!
//! ## Downstream dependencies
//!
//! - `#932` (RAG grounding): retrieval scorer weights `EpistemicConfidence`
//!   and filters `Retracted` chunks.
//! - `#952` (multi-model ACP): each model output carries an `EpistemicState`
//!   so the synthesis layer can detect inter-model contradictions.
//!
//! Canon: C30 (no silent failures), C156 (KG + memory taxonomy)

use serde::{Deserialize, Serialize};

// ── ClaimStatus ───────────────────────────────────────────────────────────────

/// The epistemic status of the claim carried by a [`DocumentChunk`].
///
/// Statuses are ordered from weakest to strongest assertion:
/// `Retracted` < `Disputed` < `Asserted` < `Corroborated`.
///
/// A retrieval layer implementing C30 MUST silently filter `Retracted` chunks
/// so they never surface to a generation step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimStatus {
    /// The claim has been explicitly withdrawn.
    /// MUST NOT be returned by the retrieval layer.
    Retracted,
    /// The claim is contested by one or more other chunks.
    /// See `EpistemicState::contradictions` for the conflicting references.
    Disputed,
    /// The claim is stated without independent verification.
    /// The default for freshly ingested content.
    Asserted,
    /// The claim is supported by two or more independent sources.
    /// Highest retrieval weight after `ConfidenceTier::Canon`.
    Corroborated,
}

impl Default for ClaimStatus {
    /// `Asserted` is the safe default for new content.
    fn default() -> Self {
        Self::Asserted
    }
}

impl std::fmt::Display for ClaimStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Retracted    => write!(f, "Retracted"),
            Self::Disputed     => write!(f, "Disputed"),
            Self::Asserted     => write!(f, "Asserted"),
            Self::Corroborated => write!(f, "Corroborated"),
        }
    }
}

// ── EvidenceKind ──────────────────────────────────────────────────────────────

/// The kind of evidence that grounds the claim in a [`DocumentChunk`].
///
/// Evidence kinds determine default confidence priors and retrieval weights
/// downstream in `gaia-memos`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    /// In-situ or satellite sensor measurement.  Highest empirical weight.
    DirectObservation,
    /// Output produced by a predictive or generative model.
    /// Confidence ceiling: 0.85 unless independently corroborated.
    ModelOutput,
    /// Self-reported observation by a human contributor.
    HumanReport,
    /// Claim sourced from a peer-reviewed paper or official report.
    LiteratureCitation,
    /// Continuous stream from a `SensorThings`-compatible feed.
    SensorStream,
    /// Derived, summarised, or otherwise synthetic content.
    /// Lowest default confidence prior; always labelled `Order / AIVoice`.
    Synthetic,
}

impl Default for EvidenceKind {
    /// `Synthetic` is the safe default — the most conservative prior.
    fn default() -> Self {
        Self::Synthetic
    }
}

impl std::fmt::Display for EvidenceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirectObservation => write!(f, "DirectObservation"),
            Self::ModelOutput        => write!(f, "ModelOutput"),
            Self::HumanReport        => write!(f, "HumanReport"),
            Self::LiteratureCitation => write!(f, "LiteratureCitation"),
            Self::SensorStream       => write!(f, "SensorStream"),
            Self::Synthetic          => write!(f, "Synthetic"),
        }
    }
}

// ── EpistemicConfidence ───────────────────────────────────────────────────────

/// A confidence score in the closed interval \[0.0, 1.0\].
///
/// Enforces the invariant at construction time:
/// - Values outside \[0.0, 1.0\] are rejected with `Err`.
/// - NaN is rejected with `Err`.
/// - There is no silent default.  Use `EpistemicConfidence::certain()`,
///   `EpistemicConfidence::half()`, or `EpistemicConfidence::new(f)` explicitly.
///
/// # Examples
///
/// ```rust
/// use gaia_ingest::epistemic::EpistemicConfidence;
///
/// let c = EpistemicConfidence::new(0.75).expect("valid");
/// assert_eq!(c.value(), 0.75_f32);
///
/// assert!(EpistemicConfidence::new(1.1).is_err());
/// assert!(EpistemicConfidence::new(f32::NAN).is_err());
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EpistemicConfidence(f32);

impl EpistemicConfidence {
    /// Construct a confidence score.  Returns `Err` if `value` is NaN or
    /// outside \[0.0, 1.0\].
    pub fn new(value: f32) -> Result<Self, EpistemicError> {
        if value.is_nan() || value < 0.0 || value > 1.0 {
            return Err(EpistemicError::ConfidenceOutOfRange(value));
        }
        Ok(Self(value))
    }

    /// Returns confidence = 1.0 (complete certainty).
    /// Reserved for hash-verified canon content.
    pub fn certain() -> Self { Self(1.0) }

    /// Returns confidence = 0.5 (maximum uncertainty).
    pub fn half() -> Self { Self(0.5) }

    /// Returns confidence = 0.0 (no confidence; content should be retracted).
    pub fn zero() -> Self { Self(0.0) }

    /// The raw confidence value.
    pub fn value(self) -> f32 { self.0 }
}

impl PartialEq for EpistemicConfidence {
    fn eq(&self, other: &Self) -> bool {
        // Treat near-equal floats as equal up to f32 epsilon.
        (self.0 - other.0).abs() < f32::EPSILON
    }
}

impl PartialOrd for EpistemicConfidence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

// ── ContradictionRef ──────────────────────────────────────────────────────────

/// A reference to a chunk that contradicts the current chunk.
///
/// Contradiction detection is the responsibility of the ingest pipeline's
/// verify step (future work). When detected, the pipeline adds a
/// `ContradictionRef` to both chunks and promotes both to `Disputed`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContradictionRef {
    /// The URI of the conflicting document (`document_uri` in the other chunk).
    pub document_uri: String,
    /// The chunk index within that document.
    pub chunk_index: u32,
    /// Optional human-readable description of the conflict.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl ContradictionRef {
    /// Construct a contradiction reference without a note.
    pub fn new(document_uri: impl Into<String>, chunk_index: u32) -> Self {
        Self {
            document_uri: document_uri.into(),
            chunk_index,
            note: None,
        }
    }

    /// Construct a contradiction reference with a note.
    pub fn with_note(
        document_uri: impl Into<String>,
        chunk_index: u32,
        note: impl Into<String>,
    ) -> Self {
        Self {
            document_uri: document_uri.into(),
            chunk_index,
            note: Some(note.into()),
        }
    }
}

// ── EpistemicState ────────────────────────────────────────────────────────────

/// Epistemic metadata for a single [`DocumentChunk`].
///
/// Answers: what is the claim status, what kind of evidence grounds it,
/// how confident is the pipeline, and does it contradict any other chunk?
///
/// ## Field contract
///
/// | Field | Invariant |
/// |---|---|
/// | `claim_status` | Defaults to `Asserted`. Retracted chunks must not reach the retrieval layer. |
/// | `evidence_kind` | Required. Defaults to `Synthetic` (the most conservative prior). |
/// | `confidence` | Float in \[0.0, 1.0\]. NaN is rejected at construction time. |
/// | `contradictions` | Empty when no conflicts are known. Non-empty implies `Disputed`. |
/// | `grounding_uris` | URIs of documents that corroborate this chunk. Non-empty implies `Corroborated`. |
/// | `rationale` | Optional free-text note for humans and audit trails. |
///
/// ## Consistency rule
///
/// `is_internally_consistent()` returns `false` if:
/// - `claim_status == Disputed` and `contradictions` is empty, or
/// - `claim_status == Corroborated` and `grounding_uris` is empty.
///
/// The pipeline should log a warning on inconsistency but must not panic —
/// the chunk is still valid for storage purposes (C30).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpistemicState {
    /// The epistemic status of the claim.
    pub claim_status: ClaimStatus,
    /// The kind of evidence that grounds the claim.
    pub evidence_kind: EvidenceKind,
    /// Pipeline confidence in the claim.
    pub confidence: EpistemicConfidence,
    /// Chunks that contradict this one.  Empty when no conflicts are known.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contradictions: Vec<ContradictionRef>,
    /// URIs of independent documents that corroborate this claim.
    /// Non-empty implies `Corroborated`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grounding_uris: Vec<String>,
    /// Optional free-text rationale for the assigned status and confidence.
    /// For humans and audit trails only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
}

impl EpistemicState {
    /// Returns `true` if the internal consistency rule holds:
    /// - `Disputed` requires at least one `ContradictionRef`.
    /// - `Corroborated` requires at least one `grounding_uri`.
    /// - All other statuses have no structural requirements on those vecs.
    pub fn is_internally_consistent(&self) -> bool {
        match self.claim_status {
            ClaimStatus::Disputed     => !self.contradictions.is_empty(),
            ClaimStatus::Corroborated => !self.grounding_uris.is_empty(),
            _ => true,
        }
    }
}

// ── EpistemicStateBuilder ─────────────────────────────────────────────────────

/// Ergonomic builder for [`EpistemicState`].
///
/// # Examples
///
/// ```rust
/// use gaia_ingest::epistemic::{
///     ClaimStatus, EvidenceKind, EpistemicConfidence, EpistemicStateBuilder,
/// };
///
/// let state = EpistemicStateBuilder::new()
///     .claim_status(ClaimStatus::Corroborated)
///     .evidence_kind(EvidenceKind::LiteratureCitation)
///     .confidence(0.92)
///     .grounding_uri("https://doi.org/10.1038/s41558-023-01799-8")
///     .rationale("Corroborated by two independent AR6 chapters.")
///     .build()
///     .expect("valid epistemic state");
///
/// assert!(state.is_internally_consistent());
/// assert!(state.confidence.value() > 0.9);
/// ```
#[derive(Debug, Default)]
pub struct EpistemicStateBuilder {
    claim_status:    Option<ClaimStatus>,
    evidence_kind:   Option<EvidenceKind>,
    confidence:      Option<f32>,
    contradictions:  Vec<ContradictionRef>,
    grounding_uris:  Vec<String>,
    rationale:       Option<String>,
}

impl EpistemicStateBuilder {
    /// Create a new builder with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the claim status.  Defaults to `Asserted` if not set.
    pub fn claim_status(mut self, status: ClaimStatus) -> Self {
        self.claim_status = Some(status);
        self
    }

    /// Set the evidence kind.  Defaults to `Synthetic` if not set.
    pub fn evidence_kind(mut self, kind: EvidenceKind) -> Self {
        self.evidence_kind = Some(kind);
        self
    }

    /// Set the confidence score.  Must be in \[0.0, 1.0\] and not NaN.
    /// Defaults to `0.5` if not set.
    pub fn confidence(mut self, value: f32) -> Self {
        self.confidence = Some(value);
        self
    }

    /// Add a contradiction reference.
    pub fn contradiction(mut self, c: ContradictionRef) -> Self {
        self.contradictions.push(c);
        self
    }

    /// Add a grounding URI (corroborating source).
    pub fn grounding_uri(mut self, uri: impl Into<String>) -> Self {
        self.grounding_uris.push(uri.into());
        self
    }

    /// Set a free-text rationale.
    pub fn rationale(mut self, text: impl Into<String>) -> Self {
        self.rationale = Some(text.into());
        self
    }

    /// Consume the builder and return an `EpistemicState`.
    ///
    /// Returns `Err` if the confidence value is invalid.
    pub fn build(self) -> Result<EpistemicState, EpistemicError> {
        let confidence = EpistemicConfidence::new(self.confidence.unwrap_or(0.5))?;
        Ok(EpistemicState {
            claim_status:   self.claim_status.unwrap_or_default(),
            evidence_kind:  self.evidence_kind.unwrap_or_default(),
            confidence,
            contradictions: self.contradictions,
            grounding_uris: self.grounding_uris,
            rationale:      self.rationale,
        })
    }
}

// ── EpistemicError ────────────────────────────────────────────────────────────

/// Errors produced by epistemic state construction.
#[derive(Debug, Clone, PartialEq)]
pub enum EpistemicError {
    /// The supplied confidence value is NaN or outside \[0.0, 1.0\].
    ConfidenceOutOfRange(f32),
}

impl std::fmt::Display for EpistemicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfidenceOutOfRange(v) => {
                write!(f, "confidence value {v} is outside [0.0, 1.0] or is NaN")
            }
        }
    }
}

impl std::error::Error for EpistemicError {}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── ClaimStatus ──

    #[test]
    fn claim_status_default_is_asserted() {
        assert_eq!(ClaimStatus::default(), ClaimStatus::Asserted);
    }

    #[test]
    fn claim_status_ordering() {
        assert!(ClaimStatus::Retracted < ClaimStatus::Disputed);
        assert!(ClaimStatus::Disputed  < ClaimStatus::Asserted);
        assert!(ClaimStatus::Asserted  < ClaimStatus::Corroborated);
    }

    #[test]
    fn claim_status_display() {
        assert_eq!(ClaimStatus::Retracted.to_string(),    "Retracted");
        assert_eq!(ClaimStatus::Disputed.to_string(),     "Disputed");
        assert_eq!(ClaimStatus::Asserted.to_string(),     "Asserted");
        assert_eq!(ClaimStatus::Corroborated.to_string(), "Corroborated");
    }

    // ── EvidenceKind ──

    #[test]
    fn evidence_kind_default_is_synthetic() {
        assert_eq!(EvidenceKind::default(), EvidenceKind::Synthetic);
    }

    #[test]
    fn evidence_kind_display_round_trip() {
        let kinds = [
            EvidenceKind::DirectObservation,
            EvidenceKind::ModelOutput,
            EvidenceKind::HumanReport,
            EvidenceKind::LiteratureCitation,
            EvidenceKind::SensorStream,
            EvidenceKind::Synthetic,
        ];
        for k in kinds {
            assert!(!k.to_string().is_empty(), "{k:?} has empty Display");
        }
    }

    // ── EpistemicConfidence ──

    #[test]
    fn confidence_valid_range() {
        assert!(EpistemicConfidence::new(0.0).is_ok());
        assert!(EpistemicConfidence::new(0.5).is_ok());
        assert!(EpistemicConfidence::new(1.0).is_ok());
    }

    #[test]
    fn confidence_rejects_nan() {
        assert!(EpistemicConfidence::new(f32::NAN).is_err());
    }

    #[test]
    fn confidence_rejects_above_one() {
        assert!(EpistemicConfidence::new(1.001).is_err());
        assert!(EpistemicConfidence::new(f32::INFINITY).is_err());
    }

    #[test]
    fn confidence_rejects_below_zero() {
        assert!(EpistemicConfidence::new(-0.001).is_err());
        assert!(EpistemicConfidence::new(f32::NEG_INFINITY).is_err());
    }

    #[test]
    fn confidence_certain_is_one() {
        assert!((EpistemicConfidence::certain().value() - 1.0_f32).abs() < f32::EPSILON);
    }

    #[test]
    fn confidence_half_is_zero_five() {
        assert!((EpistemicConfidence::half().value() - 0.5_f32).abs() < f32::EPSILON);
    }

    #[test]
    fn confidence_zero_is_zero() {
        assert!((EpistemicConfidence::zero().value() - 0.0_f32).abs() < f32::EPSILON);
    }

    #[test]
    fn confidence_equality_within_epsilon() {
        let a = EpistemicConfidence::new(0.75).unwrap();
        let b = EpistemicConfidence::new(0.75).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn confidence_ordering() {
        let low  = EpistemicConfidence::new(0.3).unwrap();
        let high = EpistemicConfidence::new(0.9).unwrap();
        assert!(low < high);
    }

    // ── ContradictionRef ──

    #[test]
    fn contradiction_ref_no_note() {
        let r = ContradictionRef::new("gaia://canon/doc", 3);
        assert_eq!(r.document_uri, "gaia://canon/doc");
        assert_eq!(r.chunk_index, 3);
        assert!(r.note.is_none());
    }

    #[test]
    fn contradiction_ref_with_note() {
        let r = ContradictionRef::with_note("gaia://canon/doc", 3, "Conflicting sea-level figure");
        assert_eq!(r.note.as_deref(), Some("Conflicting sea-level figure"));
    }

    // ── EpistemicState ──

    fn make_state(status: ClaimStatus, conf: f32) -> EpistemicState {
        EpistemicStateBuilder::new()
            .claim_status(status)
            .evidence_kind(EvidenceKind::ModelOutput)
            .confidence(conf)
            .build()
            .unwrap()
    }

    #[test]
    fn asserted_state_is_consistent() {
        let s = make_state(ClaimStatus::Asserted, 0.7);
        assert!(s.is_internally_consistent());
    }

    #[test]
    fn retracted_state_is_consistent() {
        // Retracted has no structural requirement on contradictions.
        let s = make_state(ClaimStatus::Retracted, 0.0);
        assert!(s.is_internally_consistent());
    }

    #[test]
    fn disputed_without_contradictions_is_inconsistent() {
        let s = make_state(ClaimStatus::Disputed, 0.4);
        assert!(!s.is_internally_consistent(),
            "Disputed with no ContradictionRefs must be inconsistent");
    }

    #[test]
    fn disputed_with_contradiction_is_consistent() {
        let s = EpistemicStateBuilder::new()
            .claim_status(ClaimStatus::Disputed)
            .evidence_kind(EvidenceKind::HumanReport)
            .confidence(0.4)
            .contradiction(ContradictionRef::new("gaia://canon/other", 0))
            .build()
            .unwrap();
        assert!(s.is_internally_consistent());
        assert_eq!(s.contradictions.len(), 1);
    }

    #[test]
    fn corroborated_without_grounding_uris_is_inconsistent() {
        let s = make_state(ClaimStatus::Corroborated, 0.95);
        assert!(!s.is_internally_consistent(),
            "Corroborated with no grounding_uris must be inconsistent");
    }

    #[test]
    fn corroborated_with_grounding_uri_is_consistent() {
        let s = EpistemicStateBuilder::new()
            .claim_status(ClaimStatus::Corroborated)
            .evidence_kind(EvidenceKind::LiteratureCitation)
            .confidence(0.92)
            .grounding_uri("https://doi.org/10.1038/s41558-023-01799-8")
            .build()
            .unwrap();
        assert!(s.is_internally_consistent());
        assert_eq!(s.grounding_uris.len(), 1);
    }

    // ── EpistemicStateBuilder ──

    #[test]
    fn builder_defaults_are_safe() {
        let s = EpistemicStateBuilder::new().build().unwrap();
        assert_eq!(s.claim_status,  ClaimStatus::Asserted);
        assert_eq!(s.evidence_kind, EvidenceKind::Synthetic);
        assert!((s.confidence.value() - 0.5_f32).abs() < f32::EPSILON);
        assert!(s.contradictions.is_empty());
        assert!(s.grounding_uris.is_empty());
        assert!(s.rationale.is_none());
    }

    #[test]
    fn builder_rejects_bad_confidence() {
        let err = EpistemicStateBuilder::new()
            .confidence(1.5)
            .build();
        assert!(err.is_err());
    }

    #[test]
    fn builder_sets_rationale() {
        let s = EpistemicStateBuilder::new()
            .rationale("Ingested from AR6 WG2 Chapter 9")
            .build()
            .unwrap();
        assert_eq!(s.rationale.as_deref(), Some("Ingested from AR6 WG2 Chapter 9"));
    }

    // ── Serde round-trip ──

    #[test]
    fn serde_roundtrip_minimal() {
        let s = EpistemicStateBuilder::new()
            .claim_status(ClaimStatus::Asserted)
            .evidence_kind(EvidenceKind::DirectObservation)
            .confidence(0.8)
            .build()
            .unwrap();
        let json = serde_json::to_string(&s).unwrap();
        // Empty vecs and None option fields must be omitted.
        assert!(!json.contains("contradictions"), "contradictions must be absent when empty");
        assert!(!json.contains("grounding_uris"),  "grounding_uris must be absent when empty");
        assert!(!json.contains("rationale"),        "rationale must be absent when None");
        let back: EpistemicState = serde_json::from_str(&json).unwrap();
        assert_eq!(back.claim_status,  ClaimStatus::Asserted);
        assert_eq!(back.evidence_kind, EvidenceKind::DirectObservation);
        assert!((back.confidence.value() - 0.8_f32).abs() < 0.001_f32);
    }

    #[test]
    fn serde_roundtrip_full() {
        let s = EpistemicStateBuilder::new()
            .claim_status(ClaimStatus::Disputed)
            .evidence_kind(EvidenceKind::HumanReport)
            .confidence(0.45)
            .contradiction(ContradictionRef::with_note("gaia://test/doc", 2, "Different temperature"))
            .grounding_uri("https://example.com/corroboration")
            .rationale("Two sources conflict on surface temp")
            .build()
            .unwrap();
        let json = serde_json::to_string(&s).unwrap();
        let back: EpistemicState = serde_json::from_str(&json).unwrap();
        assert_eq!(back.claim_status,  ClaimStatus::Disputed);
        assert_eq!(back.contradictions.len(), 1);
        assert_eq!(back.contradictions[0].document_uri, "gaia://test/doc");
        assert_eq!(back.contradictions[0].chunk_index,  2);
        assert_eq!(back.contradictions[0].note.as_deref(), Some("Different temperature"));
        assert_eq!(back.grounding_uris.len(), 1);
        assert_eq!(back.rationale.as_deref(), Some("Two sources conflict on surface temp"));
    }

    // ── EpistemicError ──

    #[test]
    fn epistemic_error_display() {
        let e = EpistemicError::ConfidenceOutOfRange(1.5);
        let s = e.to_string();
        assert!(s.contains("1.5"));
        assert!(s.contains("[0.0, 1.0]"));
    }
}
