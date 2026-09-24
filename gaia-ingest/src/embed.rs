//! `EmbeddingVector` — typed wrapper for a dense float-32 embedding.
//!
//! This module is intentionally thin. It owns only the newtype and its
//! invariant-checking helpers so that the embedding concern is isolated from
//! the rest of the ingest schema.  The actual model call lives upstream
//! (in `gaia-aikd` or a future `gaia-embed` crate).
//!
//! ## Invariant
//! An `EmbeddingVector` is always non-empty. `EmbeddingVector::new` returns
//! `None` when given an empty slice so callers cannot accidentally store a
//! zero-dimensional vector.

use serde::{Deserialize, Serialize};

/// A dense, real-valued embedding vector produced by a text embedding model.
///
/// The inner `Vec<f32>` is always non-empty; use [`EmbeddingVector::new`] to
/// construct one safely.  Dimensionality is model-dependent (e.g. 1 536 for
/// `text-embedding-3-small`, 3 072 for `text-embedding-3-large`).
///
/// The vector is stored as-is — callers are responsible for normalising to
/// unit length when cosine similarity is required.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingVector(Vec<f32>);

impl EmbeddingVector {
    /// Constructs an `EmbeddingVector` from `values`.
    ///
    /// Returns `None` when `values` is empty because a zero-dimensional
    /// vector is meaningless for retrieval.
    pub fn new(values: Vec<f32>) -> Option<Self> {
        if values.is_empty() {
            None
        } else {
            Some(Self(values))
        }
    }

    /// Returns a reference to the underlying float slice.
    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }

    /// Number of dimensions in the vector.
    pub fn dim(&self) -> usize {
        self.0.len()
    }

    /// Returns `true`; exists for symmetry with the `Option<EmbeddingVector>`
    /// pattern used on `DocumentChunk::embedding`.
    pub fn is_populated(&self) -> bool {
        true
    }
}

impl From<EmbeddingVector> for Vec<f32> {
    fn from(ev: EmbeddingVector) -> Self {
        ev.0
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_vec() {
        assert!(EmbeddingVector::new(vec![]).is_none());
    }

    #[test]
    fn new_accepts_non_empty_vec() {
        let ev = EmbeddingVector::new(vec![0.1, 0.2, 0.3]).unwrap();
        assert_eq!(ev.dim(), 3);
    }

    #[test]
    fn as_slice_returns_values() {
        let vals = vec![1.0_f32, 2.0, 3.0];
        let ev = EmbeddingVector::new(vals.clone()).unwrap();
        assert_eq!(ev.as_slice(), vals.as_slice());
    }

    #[test]
    fn is_populated_always_true() {
        let ev = EmbeddingVector::new(vec![0.5]).unwrap();
        assert!(ev.is_populated());
    }

    #[test]
    fn into_vec_consumes_and_returns_inner() {
        let vals = vec![0.1_f32, 0.9];
        let ev = EmbeddingVector::new(vals.clone()).unwrap();
        let out: Vec<f32> = ev.into();
        assert_eq!(out, vals);
    }

    #[test]
    fn serde_roundtrip() {
        let ev = EmbeddingVector::new(vec![0.1, 0.2, 0.3]).unwrap();
        let json = serde_json::to_string(&ev).expect("serialize");
        let back: EmbeddingVector = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(ev, back);
    }
}
