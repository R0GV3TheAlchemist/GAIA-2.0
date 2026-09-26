//! Live citation lookup against the ingest [`ChunkStore`].

use gaia_ingest::{ChunkId, ChunkStore};

use crate::rank::RankedHit;
use crate::retrieve::{GenerationContext, RetrievedChunk};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CitationError {
    Empty,
    UnknownChunk(String),
}

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

fn raw_text(text: &str) -> &str {
    text.strip_prefix("[STALE] ").unwrap_or(text)
}

pub fn citations_from_chunks(chunks: &[RetrievedChunk]) -> Vec<String> {
    chunks
        .iter()
        .map(|c| ChunkId::from_text(raw_text(&c.text)).to_hex())
        .collect()
}

pub fn citations_from_context(ctx: &GenerationContext) -> Vec<String> {
    citations_from_chunks(&ctx.chunks)
}

/// Resolve ranked retrieval hits against the ingest store.
pub fn citations_from_hits(
    store: &ChunkStore,
    hits: &[RankedHit],
) -> Result<Vec<String>, CitationError> {
    let ids: Vec<String> = hits.iter().map(|h| h.hex.clone()).collect();
    lookup(store, &ids)
}

pub fn id_for_text(text: &str) -> String {
    ChunkId::from_text(text).to_hex()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gaia_ingest::auth::{AgentId, ChunkMetadata};

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
        let id = ChunkId::from_text("treaty signed in 1992");
        store.record(id);
        let hex = id.to_hex();
        assert_eq!(lookup(&store, &[hex.clone()]).unwrap(), vec![hex]);
    }

    #[test]
    fn lookup_rejects_invented_id() {
        let store = ChunkStore::new();
        assert!(matches!(
            lookup(&store, &["deadbeef".into()]),
            Err(CitationError::UnknownChunk(_))
        ));
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

    #[test]
    fn hits_empty_is_empty_error() {
        let store = ChunkStore::new();
        assert_eq!(citations_from_hits(&store, &[]), Err(CitationError::Empty));
    }
}
