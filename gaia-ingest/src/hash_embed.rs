//! Offline 384-d signed hashing embedder. Not MiniLM. No downloads.

use crate::embed::{EmbedError, EmbeddingModel, EmbeddingVector};

const DIM: usize = 384;

#[derive(Debug, Clone)]
pub struct HashingEmbedder {
    dim: usize,
}

impl Default for HashingEmbedder {
    fn default() -> Self {
        Self { dim: DIM }
    }
}

impl HashingEmbedder {
    pub fn new() -> Self {
        Self::default()
    }
}

fn hash_feature(feat: &str) -> (usize, f32) {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in feat.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x100000001b3);
    }
    let bucket = (h as usize) % DIM;
    let sign = if h & 1 == 0 { 1.0_f32 } else { -1.0_f32 };
    (bucket, sign)
}

fn embed_one(text: &str) -> EmbeddingVector {
    let mut acc = vec![0.0_f32; DIM];
    let lower = text.to_ascii_lowercase();
    for tok in lower.split(|c: char| !c.is_ascii_alphanumeric()) {
        if tok.is_empty() {
            continue;
        }
        let (i, s) = hash_feature(tok);
        acc[i] += s;
        let bytes = tok.as_bytes();
        if bytes.len() >= 3 {
            for w in bytes.windows(3) {
                let feat = format!("#{:02x}{:02x}{:02x}", w[0], w[1], w[2]);
                let (i, s) = hash_feature(&feat);
                acc[i] += s;
            }
        }
    }
    let norm = acc.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in &mut acc {
            *x /= norm;
        }
    } else {
        acc[0] = 1.0;
    }
    EmbeddingVector::new(acc).expect("384-d non-empty")
}

impl EmbeddingModel for HashingEmbedder {
    fn embed(&self, texts: &[&str]) -> Result<Vec<EmbeddingVector>, EmbedError> {
        if texts.is_empty() {
            return Err(EmbedError::EmptyInput);
        }
        Ok(texts.iter().map(|t| embed_one(t)).collect())
    }

    fn dim(&self) -> Option<usize> {
        Some(self.dim)
    }

    fn model_id(&self) -> &str {
        "hashing-384-offline-v0"
    }
}

/// Cosine similarity for two equal-length vectors.
pub fn cosine(a: &EmbeddingVector, b: &EmbeddingVector) -> Option<f32> {
    if a.dim() != b.dim() {
        return None;
    }
    let dot: f32 = a
        .as_slice()
        .iter()
        .zip(b.as_slice())
        .map(|(x, y)| x * y)
        .sum();
    Some(dot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dim_is_384() {
        let e = HashingEmbedder::new();
        let v = e.embed(&["earth twin"]).unwrap();
        assert_eq!(v[0].dim(), 384);
        assert_eq!(e.dim(), Some(384));
        assert_eq!(e.model_id(), "hashing-384-offline-v0");
    }

    #[test]
    fn empty_input_errors() {
        assert_eq!(HashingEmbedder::new().embed(&[]), Err(EmbedError::EmptyInput));
    }

    #[test]
    fn similar_text_outranks_unrelated() {
        let e = HashingEmbedder::new();
        let vecs = e
            .embed(&[
                "the earth twin observes climate",
                "earth twin climate observation",
                "purple piano recipes",
            ])
            .unwrap();
        let close = cosine(&vecs[0], &vecs[1]).unwrap();
        let far = cosine(&vecs[0], &vecs[2]).unwrap();
        assert!(close > far, "close {close} should beat far {far}");
    }
}
