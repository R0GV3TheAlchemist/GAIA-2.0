//! `DocumentChunk` — the canonical text-document record for RAG retrieval.
//!
//! `DocumentChunk` is the text-document counterpart to `NormalizedObservation`.
//! Where `NormalizedObservation` carries sensor and Earth-observation data,
//! `DocumentChunk` carries the text passages that constitute GAIA's knowledge
//! base: canon tablets, research documents, episodic summaries, and spec files.
//!
//! See `gaia-spec/rag/chunking-standard.md` for the authoritative chunking
//! parameters (size, overlap, boundary rules, per-kind TTL policy).
//!
//! ## Design invariants
//! - `text` is the verbatim passage that will be embedded. Never empty.
//! - `char_count` must equal `text.chars().count()` at construction time.
//! - `provenance.sha256` is computed over the **raw source bytes** of the
//!   parent document, not over the chunk `text`.
//! - `artifact` is `Some` when the full source document lives in SFS.
//! - `access_tier` and `access_control` are enforced at retrieval time
//!   by `gaia-memos`; this crate only stores the policy.
//! - `lexicon_plane` defaults to `LexiconPlane::Bridge`. The ingestion
//!   pipeline promotes it to `Order` or `Chaos` via `classify_chunk()`.
//!   The RAG retrieval layer must reject implicit cross-plane lookups (C30).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    artifact::RawArtifactRef,
    lexicon::{LexiconPlane, LexiconVoice},
    provenance::ProvenanceReceipt,
};

// ── Enums ─────────────────────────────────────────────────────────────────────

/// The structural kind of the parent document being ingested for RAG.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentKind {
    /// A GAIA canon tablet (Markdown, sourced from `gaia-spec/` or the canon
    /// repository). These are eternal — TTL should be `None`.
    CanonTablet,
    /// A research or reference document (academic paper, technical report,
    /// external URL). Default TTL: 365 days.
    ResearchDocument,
    /// An episodic memory summary produced by `gaia-memos`. Default TTL: 30 days.
    EpisodeSummary,
    /// A `gaia-spec/` specification file. Eternal — versioned by git.
    SpecDocument,
    /// A source code file ingested for code-aware retrieval. Default TTL: 7 days.
    SourceCode,
    /// Any other text document. Default TTL: 90 days.
    Other,
}

impl DocumentKind {
    /// Default TTL in seconds for this kind. `None` means no expiry.
    ///
    /// These are policy defaults; callers may override per-chunk via
    /// `DocumentChunk::ttl_seconds`.
    pub fn default_ttl_seconds(self) -> Option<u64> {
        match self {
            Self::CanonTablet        => None,
            Self::ResearchDocument   => Some(365 * 24 * 3600),
            Self::EpisodeSummary     => Some(30  * 24 * 3600),
            Self::SpecDocument       => None,
            Self::SourceCode         => Some(7   * 24 * 3600),
            Self::Other              => Some(90  * 24 * 3600),
        }
    }

    /// Human-readable name for logging and UI.
    pub fn display_name(self) -> &'static str {
        match self {
            Self::CanonTablet      => "Canon Tablet",
            Self::ResearchDocument => "Research Document",
            Self::EpisodeSummary   => "Episode Summary",
            Self::SpecDocument     => "Spec Document",
            Self::SourceCode       => "Source Code",
            Self::Other            => "Other",
        }
    }
}

/// The access tier controlling which agents may retrieve this chunk at
/// query time. Enforcement is the responsibility of `gaia-memos`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessTier {
    /// Retrievable by any agent without restriction.
    Public,
    /// Retrievable only within the session that produced this chunk.
    Session,
    /// Retrievable only by agents explicitly listed in
    /// `DocumentChunk::access_control`.
    Restricted,
    /// Never retrievable via RAG. Stored only for audit and provenance.
    Private,
}

/// Confidence tier for the source material. Used at retrieval time to
/// weight results and at generation time to calibrate hedging language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceTier {
    /// Authoritative canon; highest retrieval weight. Only `CanonTablet`
    /// and `SpecDocument` kinds should use this tier.
    Canon,
    /// Verified reference material (peer-reviewed papers, official specs).
    Verified,
    /// Unverified or community-sourced content.
    Unverified,
    /// Synthetic or derived content (summaries, model outputs). Lowest
    /// retrieval weight.
    Synthetic,
}

// ── DocumentChunk ─────────────────────────────────────────────────────────────

/// A single retrievable text chunk ready for embedding and RAG retrieval.
///
/// ## Field contract
/// | Field | Invariant |
/// |---|---|
/// | `text` | Non-empty. The verbatim passage sent to the embedding model. |
/// | `char_count` | Must equal `text.chars().count()` (Unicode scalar values, not UTF-8 bytes). |
/// | `chunk_index` | Zero-indexed. Must be `< total_chunks`. |
/// | `provenance.sha256` | SHA-256 of the raw source bytes, NOT the chunk text. |
/// | `artifact` | `Some` when the full source document is stored in SFS. |
/// | `lexicon_plane` | Defaults to `Bridge`. Promoted by the ingestion pipeline. |
///
/// ## Chunking parameters
/// See `gaia-spec/rag/chunking-standard.md` for the authoritative values:
/// 400–600 token target, 200 token minimum, 800 token maximum,
/// 10–20% sliding overlap, sentence-boundary required.
///
/// ## Lexicon plane
/// Every chunk carries a `lexicon_plane` tag (`Order`, `Chaos`, or `Bridge`)
/// so the RAG pipeline always knows which ontological plane it is pulling from.
/// `Bridge` is the safe default — it signals that provenance has not yet been
/// resolved. The pipeline calls `crate::lexicon::classify_chunk()` to promote
/// the field based on document metadata (author type, language, domain, sacred
/// flag). The retrieval layer in `gaia-memos` must refuse implicit cross-plane
/// lookups (C30: no silent failures).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    /// UUID v4 assigned at ingest time.
    /// Stable across re-embeddings — the embedding vector changes when the
    /// model changes, but the chunk identity does not.
    pub id: String,

    /// Human-readable title of the parent document.
    pub document_title: String,

    /// Stable URI identifying the parent document.
    /// May be a URL (`https://...`), an SFS path (`sfs://...`),
    /// or a GAIA-internal URI (`gaia://canon/...`).
    pub document_uri: String,

    /// Structural kind of the parent document.
    pub kind: DocumentKind,

    /// The verbatim text passage to be embedded. Must not be empty.
    pub text: String,

    /// Character count of `text` (Unicode scalar values).
    /// Must equal `text.chars().count()`. Stored to avoid re-counting
    /// at query time and to detect silent truncation.
    pub char_count: usize,

    /// Zero-indexed position of this chunk within the parent document.
    pub chunk_index: u32,

    /// Total number of chunks produced from the parent document.
    pub total_chunks: u32,

    /// Domain tag for retrieval filtering.
    /// Examples: `"rag"`, `"security"`, `"earth"`, `"canon"`, `"code"`.
    /// Non-empty; use `"general"` when no specific domain applies.
    pub domain: String,

    /// BCP 47 language code. Defaults to `"en"`.
    pub language: String,

    /// Unix timestamp (seconds) when the parent document was authored or
    /// last modified. `None` when unknown (e.g., external web pages).
    pub authored_at_unix: Option<u64>,

    /// TTL in seconds before this chunk is considered stale.
    /// `None` means no expiry. See `DocumentKind::default_ttl_seconds()`
    /// for per-kind policy defaults.
    pub ttl_seconds: Option<u64>,

    /// Confidence tier of the source material.
    pub confidence: ConfidenceTier,

    /// Access tier controlling retrieval authorisation.
    pub access_tier: AccessTier,

    /// Agent IDs permitted to retrieve this chunk when
    /// `access_tier == AccessTier::Restricted`. Ignored for other tiers.
    pub access_control: Vec<String>,

    /// Cryptographic provenance receipt. Always present.
    /// `sha256` is computed over the raw source bytes of the parent document.
    pub provenance: ProvenanceReceipt,

    /// Reference to the full source document stored in SFS.
    /// `None` for inline-only sources (e.g. runtime-generated summaries).
    pub artifact: Option<RawArtifactRef>,

    /// Open map for source-native metadata not covered by fixed fields.
    /// Examples: `"heading_path"`, `"git_commit"`, `"doi"`, `"section"`.
    pub attributes: BTreeMap<String, String>,

    /// Which ontological plane this chunk's vocabulary belongs to.
    ///
    /// - `Order`  — formal, machine-tractable, AI-authored or standards-body text.
    /// - `Chaos`  — lived, cultural, human-authored, contested, or sacred text.
    /// - `Bridge` — safe default; provenance not yet resolved.
    ///
    /// Promoted from `Bridge` by the ingestion pipeline via
    /// `crate::lexicon::classify_chunk()`. The RAG retrieval layer
    /// must refuse implicit cross-plane lookups (C30).
    pub lexicon_plane: LexiconPlane,

    /// Who speaks the vocabulary in this chunk.
    /// `None` until resolved by the ingestion pipeline classify step.
    pub lexicon_voice: Option<LexiconVoice>,
}

impl DocumentChunk {
    /// Returns `true` if the chunk satisfies all structural invariants:
    /// - `text` is non-empty
    /// - `char_count` equals `text.chars().count()`
    /// - `chunk_index < total_chunks`
    /// - `document_uri` is non-empty
    /// - `domain` is non-empty
    /// - `provenance.is_valid()` passes
    ///
    /// Note: `lexicon_plane == Bridge` is valid — it means the pipeline
    /// classify step has not yet run, not that the chunk is broken.
    pub fn is_valid(&self) -> bool {
        !self.text.is_empty()
            && self.char_count == self.text.chars().count()
            && self.chunk_index < self.total_chunks
            && !self.document_uri.is_empty()
            && !self.domain.is_empty()
            && self.provenance.is_valid()
    }

    /// Returns `true` if the chunk should be considered stale given
    /// the current Unix timestamp `now_unix`.
    ///
    /// Staleness is computed against `authored_at_unix` when present,
    /// falling back to `provenance.fetched_at_unix` otherwise.
    /// A chunk without `ttl_seconds` never expires.
    pub fn is_stale(&self, now_unix: u64) -> bool {
        match self.ttl_seconds {
            None => false,
            Some(ttl) => {
                let baseline = self
                    .authored_at_unix
                    .unwrap_or(self.provenance.fetched_at_unix);
                now_unix > baseline.saturating_add(ttl)
            }
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        lexicon::{LexiconPlane, LexiconVoice},
        provenance::ProvenanceReceipt,
        schema::DataSource,
    };
    use std::collections::BTreeMap;

    /// Build a minimal valid `DocumentChunk` for use in tests.
    fn valid_chunk() -> DocumentChunk {
        let text = "GAIA maintains a cryptographic provenance receipt for every \
                    ingested byte, enabling full auditability.".to_string();
        let char_count = text.chars().count();
        DocumentChunk {
            id: "00000000-0000-0000-0000-000000000001".into(),
            document_title: "GAIA Ingest Design".into(),
            document_uri: "gaia://canon/ingest-design".into(),
            kind: DocumentKind::CanonTablet,
            char_count,
            text,
            chunk_index: 0,
            total_chunks: 1,
            domain: "canon".into(),
            language: "en".into(),
            authored_at_unix: Some(1_700_000_000),
            ttl_seconds: None,
            confidence: ConfidenceTier::Canon,
            access_tier: AccessTier::Public,
            access_control: vec![],
            provenance: ProvenanceReceipt {
                source: DataSource::CanonTablet,
                source_url: "gaia://canon/ingest-design".into(),
                external_id: "ingest-design-v1".into(),
                fetched_at_unix: 1_700_000_001,
                observed_at_unix: 1_700_000_000,
                sha256: "a".repeat(64),
                license: "CC-BY-4.0".into(),
            },
            artifact: None,
            attributes: BTreeMap::new(),
            lexicon_plane: LexiconPlane::Bridge,
            lexicon_voice: None,
        }
    }

    // ── is_valid ──

    #[test]
    fn valid_chunk_passes_invariants() {
        assert!(valid_chunk().is_valid());
    }

    #[test]
    fn rejects_empty_text() {
        let mut c = valid_chunk();
        c.text = String::new();
        c.char_count = 0;
        assert!(!c.is_valid());
    }

    #[test]
    fn rejects_mismatched_char_count() {
        let mut c = valid_chunk();
        c.char_count = c.char_count + 1;
        assert!(!c.is_valid());
    }

    #[test]
    fn rejects_chunk_index_out_of_range() {
        let mut c = valid_chunk();
        c.chunk_index = c.total_chunks;
        assert!(!c.is_valid());
    }

    #[test]
    fn rejects_empty_document_uri() {
        let mut c = valid_chunk();
        c.document_uri = String::new();
        assert!(!c.is_valid());
    }

    #[test]
    fn rejects_empty_domain() {
        let mut c = valid_chunk();
        c.domain = String::new();
        assert!(!c.is_valid());
    }

    // ── lexicon plane ──

    #[test]
    fn chunk_default_plane_is_bridge() {
        assert_eq!(valid_chunk().lexicon_plane, LexiconPlane::Bridge);
    }

    #[test]
    fn chunk_lexicon_voice_none_by_default() {
        assert_eq!(valid_chunk().lexicon_voice, None);
    }

    #[test]
    fn chunk_accepts_order_plane() {
        let mut c = valid_chunk();
        c.lexicon_plane = LexiconPlane::Order;
        c.lexicon_voice = Some(LexiconVoice::AIVoice);
        assert!(c.is_valid());
        assert_eq!(c.lexicon_plane, LexiconPlane::Order);
    }

    #[test]
    fn chunk_accepts_chaos_plane_with_sacred_voice() {
        let mut c = valid_chunk();
        c.lexicon_plane = LexiconPlane::Chaos;
        c.lexicon_voice = Some(LexiconVoice::Sacred);
        assert!(c.is_valid());
    }

    // ── is_stale ──

    #[test]
    fn no_ttl_never_stale() {
        let c = valid_chunk();
        assert!(!c.is_stale(u64::MAX));
    }

    #[test]
    fn stale_when_ttl_elapsed() {
        let mut c = valid_chunk();
        c.ttl_seconds = Some(3600);
        assert!(c.is_stale(1_700_000_000 + 3600 + 1));
    }

    #[test]
    fn not_stale_before_ttl_elapsed() {
        let mut c = valid_chunk();
        c.ttl_seconds = Some(3600);
        assert!(!c.is_stale(1_700_000_000 + 3600 - 1));
    }

    // ── serde round-trip ──

    #[test]
    fn serde_roundtrip() {
        let c = valid_chunk();
        let json = serde_json::to_string(&c).expect("serialize");
        let back: DocumentChunk = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(c.id, back.id);
        assert_eq!(c.text, back.text);
        assert_eq!(c.char_count, back.char_count);
        assert_eq!(c.chunk_index, back.chunk_index);
        assert_eq!(c.total_chunks, back.total_chunks);
        assert_eq!(c.domain, back.domain);
        assert_eq!(c.provenance.sha256, back.provenance.sha256);
        assert_eq!(c.lexicon_plane, back.lexicon_plane);
        assert_eq!(c.lexicon_voice, back.lexicon_voice);
        assert!(back.is_valid());
    }

    // ── DocumentKind helpers ──

    #[test]
    fn canon_tablet_ttl_is_none() {
        assert_eq!(DocumentKind::CanonTablet.default_ttl_seconds(), None);
    }

    #[test]
    fn source_code_ttl_is_seven_days() {
        assert_eq!(
            DocumentKind::SourceCode.default_ttl_seconds(),
            Some(7 * 24 * 3600)
        );
    }

    #[test]
    fn all_kinds_have_display_name() {
        let kinds = [
            DocumentKind::CanonTablet,
            DocumentKind::ResearchDocument,
            DocumentKind::EpisodeSummary,
            DocumentKind::SpecDocument,
            DocumentKind::SourceCode,
            DocumentKind::Other,
        ];
        for k in kinds {
            assert!(!k.display_name().is_empty(), "{k:?} has empty display_name");
        }
    }

    // ── AccessTier ordering ──

    #[test]
    fn access_tier_ordering() {
        assert!(AccessTier::Public < AccessTier::Session);
        assert!(AccessTier::Session < AccessTier::Restricted);
        assert!(AccessTier::Restricted < AccessTier::Private);
    }

    // ── unicode char_count correctness ──

    #[test]
    fn char_count_counts_unicode_scalars_not_bytes() {
        let text = "caf\u{00e9}".to_string();
        assert_eq!(text.len(), 5);
        assert_eq!(text.chars().count(), 4);

        let mut c = valid_chunk();
        c.text = text;
        c.char_count = 4;
        assert!(c.is_valid());

        c.char_count = 5;
        assert!(!c.is_valid());
    }
}
