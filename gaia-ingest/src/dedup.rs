//! Ingest-time deduplication and content fingerprinting.
//!
//! A [`ChunkStore`] tracks every [`ChunkId`] that has already been ingested.
//! Before persisting a chunk the caller checks [`ChunkStore::check`]; if the
//! id is already present the chunk is skipped and the duplicate counter on
//! [`IngestReport`] is incremented instead.
//!
//! Deduplication is **content-addressed**: two chunks with identical
//! normalised text produce the same [`ChunkId`] regardless of the source
//! path.  This is intentional — it prevents re-ingesting the same prose
//! that arrived via two different file paths.
//!
//! ## Usage
//!
//! ```rust,ignore
//! let mut store = ChunkStore::default();
//! let mut report = IngestReport::default();
//!
//! for chunk in chunks {
//!     match store.check(&chunk) {
//!         IngestResult::Stored(id) => {
//!             store.record(id);
//!             report.ingested += 1;
//!         }
//!         IngestResult::Duplicate(_) => {
//!             report.duplicates += 1;
//!         }
//!     }
//! }
//! ```

use std::collections::HashSet;

use crate::{chunk_id::ChunkId, document::DocumentChunk};

// ── IngestResult ──────────────────────────────────────────────────────────────

/// The outcome of checking a single chunk against the deduplication store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngestResult {
    /// The chunk has not been seen before — safe to store.
    Stored(ChunkId),
    /// The chunk's content fingerprint already exists in the store.
    Duplicate(ChunkId),
}

// ── IngestReport ─────────────────────────────────────────────────────────────

/// Aggregate statistics for a completed ingest run.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IngestReport {
    /// Number of chunks successfully stored (new content).
    pub ingested: usize,
    /// Number of chunks skipped due to duplicate content fingerprint.
    pub duplicates: usize,
    /// Number of chunks that could not be processed due to an error.
    pub errors: usize,
}

impl IngestReport {
    /// Returns `true` when no chunks were stored or errored — every chunk
    /// was a duplicate.
    pub fn all_duplicates(&self) -> bool {
        self.ingested == 0 && self.errors == 0
    }

    /// Total chunks seen (ingested + duplicates + errors).
    pub fn total(&self) -> usize {
        self.ingested + self.duplicates + self.errors
    }
}

// ── ChunkStore ────────────────────────────────────────────────────────────────

/// An in-memory set of [`ChunkId`]s representing already-ingested content.
///
/// This is a pure logic layer — it does not talk to any database or vector
/// store.  Persistence is the responsibility of the caller.
#[derive(Debug, Default)]
pub struct ChunkStore {
    seen: HashSet<ChunkId>,
}

impl ChunkStore {
    /// Create an empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check whether `chunk` is a duplicate of previously ingested content.
    ///
    /// Returns [`IngestResult::Stored`] when the chunk is new,
    /// [`IngestResult::Duplicate`] when the fingerprint is already present.
    /// Does **not** mutate the store — call [`ChunkStore::record`] to commit.
    pub fn check(&self, chunk: &DocumentChunk) -> IngestResult {
        let id = ChunkId::from_chunk(chunk);
        if self.seen.contains(&id) {
            IngestResult::Duplicate(id)
        } else {
            IngestResult::Stored(id)
        }
    }

    /// Record a [`ChunkId`] as ingested.  Idempotent — recording an
    /// already-present id is a no-op.
    pub fn record(&mut self, id: ChunkId) {
        self.seen.insert(id);
    }

    /// Number of distinct chunk fingerprints currently tracked.
    pub fn len(&self) -> usize {
        self.seen.len()
    }

    /// Returns `true` when no chunks have been recorded yet.
    pub fn is_empty(&self) -> bool {
        self.seen.is_empty()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        document::{AccessTier, ConfidenceTier, DocumentChunk, DocumentKind},
        lexicon::LexiconPlane,
        provenance::ProvenanceReceipt,
        schema::DataSource,
    };
    use std::collections::BTreeMap;

    fn make_chunk(text: &str, uri: &str) -> DocumentChunk {
        DocumentChunk {
            id: uuid::Uuid::new_v4().to_string(),
            text: text.into(),
            char_count: text.chars().count(),
            chunk_index: 0,
            total_chunks: 1,
            document_title: "test".into(),
            document_uri: uri.into(),
            kind: DocumentKind::SpecDocument,
            domain: "test".into(),
            language: "en".into(),
            authored_at_unix: None,
            ttl_seconds: None,
            confidence: ConfidenceTier::Verified,
            access_tier: AccessTier::Public,
            access_control: Vec::new(),
            attributes: BTreeMap::new(),
            lexicon_plane: LexiconPlane::Bridge,
            lexicon_voice: None,
            provenance: ProvenanceReceipt {
                source: DataSource::InternalDocument,
                source_url: uri.into(),
                external_id: "test".into(),
                fetched_at_unix: 1,
                observed_at_unix: 1,
                sha256: "a".repeat(64),
                license: "proprietary".into(),
            },
            artifact: None,
        }
    }

    #[test]
    fn new_chunk_is_stored() {
        let store = ChunkStore::new();
        let chunk = make_chunk("hello world", "file:///a.md");
        assert!(matches!(store.check(&chunk), IngestResult::Stored(_)));
    }

    #[test]
    fn recorded_chunk_is_duplicate() {
        let mut store = ChunkStore::new();
        let chunk = make_chunk("hello world", "file:///a.md");
        if let IngestResult::Stored(id) = store.check(&chunk) {
            store.record(id);
        }
        assert!(matches!(store.check(&chunk), IngestResult::Duplicate(_)));
    }

    #[test]
    fn same_content_different_path_is_duplicate() {
        let mut store = ChunkStore::new();
        let a = make_chunk("identical content", "file:///a.md");
        let b = make_chunk("identical content", "file:///b.md");
        if let IngestResult::Stored(id) = store.check(&a) {
            store.record(id);
        }
        assert!(matches!(store.check(&b), IngestResult::Duplicate(_)));
    }

    #[test]
    fn different_content_different_id() {
        let mut store = ChunkStore::new();
        let a = make_chunk("content A", "file:///a.md");
        let b = make_chunk("content B", "file:///b.md");
        if let IngestResult::Stored(id) = store.check(&a) {
            store.record(id);
        }
        assert!(matches!(store.check(&b), IngestResult::Stored(_)));
    }

    #[test]
    fn report_all_duplicates() {
        let report = IngestReport { ingested: 0, duplicates: 3, errors: 0 };
        assert!(report.all_duplicates());
    }

    #[test]
    fn report_total() {
        let report = IngestReport { ingested: 2, duplicates: 1, errors: 1 };
        assert_eq!(report.total(), 4);
    }

    #[test]
    fn store_len_tracks_records() {
        let mut store = ChunkStore::new();
        assert!(store.is_empty());
        let chunk = make_chunk("some text", "file:///x.md");
        if let IngestResult::Stored(id) = store.check(&chunk) {
            store.record(id);
        }
        assert_eq!(store.len(), 1);
    }
}
