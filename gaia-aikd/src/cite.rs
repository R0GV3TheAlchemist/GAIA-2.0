//! retrieve → filter → live citation ids. Not an answerer.

use gaia_ingest::embed::{EmbedError, EmbeddingModel};
use gaia_ingest::FileChunkStore;

use crate::citations::{citations_from_hits, CitationError};
use crate::rank::{rank_persisted, RankedHit};

#[derive(Debug, Clone, PartialEq)]
pub struct RetrievedCitations {
    pub hits: Vec<RankedHit>,
    pub citation_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RetrieveCiteError {
    Embed(EmbedError),
    Cite(CitationError),
}

impl From<EmbedError> for RetrieveCiteError {
    fn from(e: EmbedError) -> Self {
        Self::Embed(e)
    }
}

impl From<CitationError> for RetrieveCiteError {
    fn from(e: CitationError) -> Self {
        Self::Cite(e)
    }
}

/// Rank persisted chunks, drop hits below `min_score`, resolve surviving hex ids.
pub fn retrieve_and_cite(
    query: &str,
    store: &FileChunkStore,
    embedder: &dyn EmbeddingModel,
    k: usize,
    min_score: f32,
) -> Result<RetrievedCitations, RetrieveCiteError> {
    let ranked = rank_persisted(query, store, embedder, k)?;
    let hits = RankedHit::filter(ranked, min_score);
    let citation_ids = citations_from_hits(store.store(), &hits)?;
    Ok(RetrievedCitations {
        hits,
        citation_ids,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use gaia_ingest::embed::EmbeddingModel;
    use gaia_ingest::HashingEmbedder;

    #[test]
    fn retrieve_and_cite_returns_climate_id() {
        let path = std::env::temp_dir().join(format!("gaia-cite-{}.jsonl", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let embedder = HashingEmbedder::new();
        let climate = "the earth twin observes climate";
        let piano = "purple piano recipes";
        let v = embedder.embed(&[climate, piano]).unwrap();
        let mut store = FileChunkStore::open(&path).unwrap();
        store.record(climate, embedder.model_id(), Some(&v[0])).unwrap();
        store.record(piano, embedder.model_id(), Some(&v[1])).unwrap();
        let out = retrieve_and_cite("earth twin climate observation", &store, &embedder, 2, 0.0)
            .unwrap();
        assert_eq!(out.hits[0].text, climate);
        assert_eq!(out.citation_ids[0], out.hits[0].hex);
        assert_eq!(out.citation_ids[0].len(), 64);
    }

    #[test]
    fn high_min_score_can_empty() {
        let path = std::env::temp_dir().join(format!("gaia-cite-empty-{}.jsonl", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let store = FileChunkStore::open(&path).unwrap();
        let err = retrieve_and_cite("q", &store, &HashingEmbedder::new(), 3, 0.99).unwrap_err();
        assert!(matches!(err, RetrieveCiteError::Cite(CitationError::Empty)));
    }
}
