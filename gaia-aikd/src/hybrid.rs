//! Hybrid retrieval: BM25-lite lexical + hashing cosine, fused with RRF.
//! Not Lucene. Not MiniLM. Linear scan over JSONL.

use std::collections::{HashMap, HashSet};

use gaia_ingest::embed::{EmbedError, EmbeddingModel};
use gaia_ingest::FileChunkStore;

use crate::rank::{rank_persisted, RankedHit};

const RRF_K: f32 = 60.0;
const BM25_K1: f32 = 1.2;
const BM25_B: f32 = 0.75;

fn tokens(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| t.len() > 2)
        .map(|t| t.to_ascii_lowercase())
        .collect()
}

fn token_set(text: &str) -> HashSet<String> {
    tokens(text).into_iter().collect()
}

/// BM25-lite over persisted chunk text. Rows with empty text score 0.
pub fn rank_lexical(query: &str, store: &FileChunkStore, k: usize) -> Vec<RankedHit> {
    if k == 0 {
        return Vec::new();
    }
    let q = token_set(query);
    if q.is_empty() {
        return Vec::new();
    }
    let rows = store.rows();
    let n = rows.len() as f32;
    if n == 0.0 {
        return Vec::new();
    }
    let docs: Vec<Vec<String>> = rows.iter().map(|r| tokens(&r.text)).collect();
    let avgdl = docs.iter().map(|d| d.len() as f32).sum::<f32>() / n.max(1.0);
    let mut df: HashMap<String, f32> = HashMap::new();
    for doc in &docs {
        let mut seen = HashSet::new();
        for t in doc {
            if seen.insert(t.clone()) {
                *df.entry(t.clone()).or_insert(0.0) += 1.0;
            }
        }
    }
    let mut hits = Vec::new();
    for (row, doc) in rows.iter().zip(docs.iter()) {
        let dl = doc.len() as f32;
        let mut tf: HashMap<&str, f32> = HashMap::new();
        for t in doc {
            *tf.entry(t.as_str()).or_insert(0.0) += 1.0;
        }
        let mut score = 0.0_f32;
        for term in &q {
            let n_t = *df.get(term).unwrap_or(&0.0);
            // Lucene IDF: ln(1 + (N - n + 0.5) / (n + 0.5))
            let idf = (1.0 + (n - n_t + 0.5) / (n_t + 0.5)).ln();
            let f = *tf.get(term.as_str()).unwrap_or(&0.0);
            let denom = f + BM25_K1 * (1.0 - BM25_B + BM25_B * (dl / avgdl.max(1.0)));
            if denom > 0.0 {
                score += idf * (f * (BM25_K1 + 1.0)) / denom;
            }
        }
        hits.push(RankedHit {
            hex: row.hex.clone(),
            text: row.text.clone(),
            score,
        });
    }
    hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    hits.truncate(k);
    hits
}

fn rrf_merge(lex: &[RankedHit], dense: &[RankedHit], k: usize) -> Vec<RankedHit> {
    let mut fused: HashMap<String, (String, f32)> = HashMap::new();
    for (rank, hit) in lex.iter().enumerate() {
        let entry = fused.entry(hit.hex.clone()).or_insert((hit.text.clone(), 0.0));
        entry.1 += 1.0 / (RRF_K + rank as f32 + 1.0);
        entry.1 += hit.score * 1e-4;
    }
    for (rank, hit) in dense.iter().enumerate() {
        let entry = fused.entry(hit.hex.clone()).or_insert((hit.text.clone(), 0.0));
        entry.1 += 1.0 / (RRF_K + rank as f32 + 1.0);
    }
    let mut out: Vec<RankedHit> = fused
        .into_iter()
        .map(|(hex, (text, score))| RankedHit { hex, text, score })
        .collect();
    out.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.hex.cmp(&b.hex))
    });
    out.truncate(k);
    out
}

/// Fuse lexical BM25-lite and hashing-cosine lists with RRF (k=60).
pub fn rank_hybrid(
    query: &str,
    store: &FileChunkStore,
    embedder: &dyn EmbeddingModel,
    k: usize,
) -> Result<Vec<RankedHit>, EmbedError> {
    if k == 0 {
        return Ok(Vec::new());
    }
    let pool = k.max(store.len()).max(1);
    let lex = rank_lexical(query, store, pool);
    let dense = rank_persisted(query, store, embedder, pool)?;
    Ok(rrf_merge(&lex, &dense, k))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gaia_ingest::embed::EmbeddingModel;
    use gaia_ingest::HashingEmbedder;

    fn store_two(label: &str, a: &str, b: &str) -> (FileChunkStore, HashingEmbedder) {
        let path = std::env::temp_dir().join(format!(
            "gaia-hybrid-{}-{}.jsonl",
            std::process::id(),
            label
        ));
        let _ = std::fs::remove_file(&path);
        let embedder = HashingEmbedder::new();
        let v = embedder.embed(&[a, b]).unwrap();
        let mut store = FileChunkStore::open(&path).unwrap();
        store.record(a, embedder.model_id(), Some(&v[0])).unwrap();
        store.record(b, embedder.model_id(), Some(&v[1])).unwrap();
        (store, embedder)
    }

    #[test]
    fn lexical_exact_token_beats_unrelated() {
        let drug = "patient prescribed lisinopril twenty milligrams daily";
        let piano = "purple piano recipes";
        let (store, _) = store_two("lex", drug, piano);
        let hits = rank_lexical("lisinopril dose", &store, 2);
        assert_eq!(hits[0].text, drug);
        assert!(hits[0].score > hits[1].score, "got {} vs {}", hits[0].score, hits[1].score);
    }

    #[test]
    fn hybrid_keeps_exact_token_first() {
        let drug = "patient prescribed lisinopril twenty milligrams daily";
        let climate = "the earth twin observes climate";
        let (store, embedder) = store_two("hyb-drug", drug, climate);
        let hits = rank_hybrid("lisinopril", &store, &embedder, 2).unwrap();
        assert_eq!(hits[0].text, drug);
    }

    #[test]
    fn hybrid_climate_still_ranks_climate() {
        let climate = "the earth twin observes climate";
        let piano = "purple piano recipes";
        let (store, embedder) = store_two("hyb-cli", climate, piano);
        let hits = rank_hybrid("earth twin climate observation", &store, &embedder, 2).unwrap();
        assert_eq!(hits[0].text, climate);
    }
}
