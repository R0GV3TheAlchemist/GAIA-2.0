//! Live citation lookup against the ingest [`ChunkStore`].
//!
//! Claimed source ids must be 64-char hex [`gaia_ingest::ChunkId`] values
//! that were recorded at ingest. Invented strings fail.

use gaia_ingest::{ChunkId, ChunkStore};

use crate::retrieve::{GenerationContext, RetrievedChunk};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CitationError {
    Empty,
    UnknownChunk(String),
}

/// Resolve claimed hex ids against the ingest store.
pub fn lookup(store: &ChunkStore, claimed: &[String]) -> Result<Vec<String>, CitationError> {
    if claimed.is_empty() {
        return Err(CitationError::Empty);
    }
    let mut out = Vec::with_capacity(claimed.len());
    for id in claimed {
        if !store.contains_hex(id) {
            return Err(CitationError::UnknownChunk(id.clone()));
        }
        out.push(id.clone());
    }
    Ok(out)
}

/// Hex ids from authorized retrieved chunks.
pub fn citations_from_chunks(chunks: &[RetrievedChunk]) -> Vec<String> {
    chunks.iter().map(|c| c.chunk_id.clone()).collect()
}

pub fn citations_from_context(ctx: &GenerationContext) -> Vec<String> {
    citations_from_chunks(&ctx.chunks)
}

pub fn id_for_text(text: &str) -> String {
    ChunkId::from_text(text).to_hex()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gaia_ingest::auth::{AgentId, ChunkMetadata};
    use gaia_ingest::document::{
        AccessTier, ConfidenceTier, DocumentChunk, DocumentKind,
    };
    use gaia_ingest::lexicon::LexiconPlane;
    use gaia_ingest::provenance::ProvenanceReceipt;
    use gaia_ingest::schema::DataSource;
    use std::collections::BTreeMap;

    fn chunk(text: &str) -> DocumentChunk {
        DocumentChunk {
            id: "00000000-0000-0000-0000-000000000001".into(),
            text: text.into(),
            char_count: text.chars().count(),
            chunk_index: 0,
            total_chunks: 1,
            document_title: "t".into(),
            document_uri: "file:///t.md".into(),
            kind: DocumentKind::SpecDocument,
            domain: "test".into(),
            language: "en".into(),
            authored_at_unix: Some(0),
            ttl_seconds: None,
            confidence: ConfidenceTier::Verified,
            access_tier: AccessTier::Public,
            access_control: Vec::new(),
            attributes: BTreeMap::new(),
            lexicon_plane: LexiconPlane::Bridge,
            lexicon_voice: None,
            provenance: ProvenanceReceipt {
                source: DataSource::InternalDocument,
                source_url: "file:///t.md".into(),
                external_id: "t".into(),
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

    fn meta() -> ChunkMetadata {
        ChunkMetadata {
            source: "s".into(),
            date: "2026-01-01".into(),
            author: None,
            domain: "d".into(),
            confidence: 0.9,
            version: "1".into(),
            authorized_for: vec![],
        }
    }

    #[test]
    fn lookup_accepts_ingested_id() {
        let mut store = ChunkStore::new();
        let c = chunk("treaty signed in 1992");
        let id = ChunkId::from_chunk(&c);
        store.record(id);
        let hex = id.to_hex();
        let got = lookup(&store, &[hex.clone()]).unwrap();
        assert_eq!(got, vec![hex]);
    }

    #[test]
    fn lookup_rejects_invented_id() {
        let store = ChunkStore::new();
        let err = lookup(&store, &["deadbeef".into()]).unwrap_err();
        assert!(matches!(err, CitationError::UnknownChunk(_)));
    }

    #[test]
    fn context_ids_match_ingest() {
        let text = "treaty signed in 1992";
        let mut store = ChunkStore::new();
        store.record(ChunkId::from_text(text));
        let ctx = GenerationContext::build(
            AgentId::new("agent-a"),
            vec![(text.into(), None, 0, 1_000, meta())],
        );
        let ids = citations_from_context(&ctx);
        assert_eq!(ids.len(), 1);
        assert!(store.contains_hex(&ids[0]));
    }
}
