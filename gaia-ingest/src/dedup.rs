//! Ingest-time deduplication and content fingerprinting.

use std::collections::HashSet;

use crate::{chunk_id::ChunkId, document::DocumentChunk};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngestResult {
    Stored(ChunkId),
    Duplicate(ChunkId),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IngestReport {
    pub ingested: usize,
    pub duplicates: usize,
    pub errors: usize,
}

impl IngestReport {
    pub fn all_duplicates(&self) -> bool {
        self.ingested == 0 && self.errors == 0
    }

    pub fn total(&self) -> usize {
        self.ingested + self.duplicates + self.errors
    }
}

#[derive(Debug, Default)]
pub struct ChunkStore {
    seen: HashSet<ChunkId>,
}

impl ChunkStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check(&self, chunk: &DocumentChunk) -> IngestResult {
        let id = ChunkId::from_chunk(chunk);
        if self.seen.contains(&id) {
            IngestResult::Duplicate(id)
        } else {
            IngestResult::Stored(id)
        }
    }

    pub fn record(&mut self, id: ChunkId) {
        self.seen.insert(id);
    }

    pub fn len(&self) -> usize {
        self.seen.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seen.is_empty()
    }

    pub fn contains(&self, id: ChunkId) -> bool {
        self.seen.contains(&id)
    }

    pub fn contains_hex(&self, hex_id: &str) -> bool {
        let Ok(bytes) = hex::decode(hex_id) else {
            return false;
        };
        if bytes.len() != 32 {
            return false;
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        self.seen.contains(&ChunkId::from_bytes(arr))
    }
}

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
            embedding: None,
            artifact: None,
            epistemic_state: None,
        }
    }

    #[test]
    fn recorded_chunk_is_found_by_hex() {
        let mut store = ChunkStore::new();
        let chunk = make_chunk("hello world", "file:///a.md");
        let id = ChunkId::from_chunk(&chunk);
        store.record(id);
        assert!(store.contains(id));
        assert!(store.contains_hex(&id.to_hex()));
        assert!(!store.contains_hex("deadbeef"));
    }
}
