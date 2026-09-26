//! Brute-force cosine rank over a [`FileChunkStore`]. Not ANN.

use gaia_ingest::embed::{EmbedError, EmbeddingModel, EmbeddingVector};
use gaia_ingest::hash_embed::cosine;
use gaia_ingest::FileChunkStore;

use crate::retrieve::embed_query;

#[derive(Debug, Clone, PartialEq)]
pub struct RankedHit {
    pub hex: String,
    pub text: String,
    pub score: f32,
}

/// Embed `query` and return the top `k` persisted rows by cosine.
///
/// Rows without an embedding, or with a dimension mismatch, are skipped.
/// Empty `k` or an empty store returns an empty vec.
pub fn rank_persisted(
    query: &str,
    store: &FileChunkStore,
    embedder: &dyn EmbeddingModel,
    k: usize,
) -> Result<Vec<RankedHit>, EmbedError> {
    if k == 0 {
        return Ok(Vec::new());
    }
    let q = embed_query(query, embedder)?;
    let mut hits = Vec::new();
    for row in store.rows() {
        let Some(raw) = row.embedding.as_ref() else {
            continue;
        };
        let Some(vec) = EmbeddingVector::new(raw.clone()) else {
            continue;
        };
        let Some(score) = cosine(&q, &vec) else {
            continue;
        };
        hits.push(RankedHit {
            hex: row.hex.clone(),
            text: row.text.clone(),
            score,
        });
    }
    hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    hits.truncate(k);
    Ok(hits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gaia_ingest::embed::EmbeddingModel;
    use gaia_ingest::HashingEmbedder;

    #[test]
    fn climate_query_ranks_climate_chunk_first() {
        let path = std::env::temp_dir().join(format!("gaia-rank-{}.jsonl", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let embedder = HashingEmbedder::new();
        let climate = "the earth twin observes climate";
        let piano = "purple piano recipes";
        let v = embedder.embed(&[climate, piano]).unwrap();
        let mut store = FileChunkStore::open(&path).unwrap();
        store.record(climate, embedder.model_id(), Some(&v[0])).unwrap();
        store.record(piano, embedder.model_id(), Some(&v[1])).unwrap();
        let hits = rank_persisted("earth twin climate observation", &store, &embedder, 2).unwrap();
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].text, climate);
        assert!(hits[0].score > hits[1].score);
    }

    #[test]
    fn k_zero_is_empty() {
        let path = std::env::temp_dir().join(format!("gaia-rank-empty-{}.jsonl", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let store = FileChunkStore::open(&path).unwrap();
        let hits = rank_persisted("q", &store, &HashingEmbedder::new(), 0).unwrap();
        assert!(hits.is_empty());
    }
}
