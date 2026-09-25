//! `EmbeddingVector` and `EmbeddingModel` — typed embedding primitives.
//!
//! This module owns:
//! - [`EmbeddingVector`]: the dense-float-32 newtype (always non-empty).
//! - [`EmbeddingModel`]: the trait that any embedder must implement.
//! - [`EmbedError`]: the error type returned by [`EmbeddingModel::embed`].
//! - [`PassthroughEmbedder`]: a deterministic offline stub for tests.
//!
//! ## Invariant
//! An `EmbeddingVector` is always non-empty. `EmbeddingVector::new` returns
//! `None` when given an empty slice so callers cannot accidentally store a
//! zero-dimensional vector.
//!
//! ## Design note
//! The actual model call lives upstream (in `gaia-aikd` or a future
//! `gaia-embed` crate). This module stays thin: only the shared contract
//! belongs here so that every crate that _produces_ or _consumes_ an
//! embedding does not need to take a transitive dependency on heavy ML libs.

use serde::{Deserialize, Serialize};

// ── EmbeddingVector ───────────────────────────────────────────────────────────

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

// ── EmbedError ────────────────────────────────────────────────────────────────

/// Errors returned by [`EmbeddingModel::embed`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EmbedError {
    /// The underlying model binary or weights are not loaded / available.
    #[error("embedding model not loaded")]
    ModelNotLoaded,
    /// The backend returned a vector with an unexpected number of dimensions.
    #[error("dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    /// An empty slice of texts was passed to `embed`.
    #[error("embed called with an empty input slice")]
    EmptyInput,
    /// A backend-specific failure (ONNX runtime, HTTP, etc.).
    #[error("embedding backend error: {0}")]
    Backend(String),
}

// ── EmbeddingModel ────────────────────────────────────────────────────────────

/// Contract for any type that can embed a slice of text strings.
///
/// Implement this trait on your model adapter and pass it to
/// [`crate::ingest::IngestPipeline`] to populate `DocumentChunk::embedding`
/// during ingestion, or to `gaia_aikd::retrieve::embed_query` for
/// query-side embedding.
///
/// ## Contract
/// - `embed` must return exactly `texts.len()` vectors in the same order.
/// - Every returned vector must have the same dimensionality for a given
///   model instance.
/// - `embed(&[])` must return `Err(EmbedError::EmptyInput)`.
///
/// ## Example
/// ```rust
/// use gaia_ingest::embed::{EmbeddingModel, EmbeddingVector, EmbedError};
///
/// let embedder = gaia_ingest::embed::PassthroughEmbedder::default();
/// let vecs = embedder.embed(&["hello", "world"]).unwrap();
/// assert_eq!(vecs.len(), 2);
/// ```
pub trait EmbeddingModel: std::fmt::Debug + Send + Sync {
    /// Embed `texts` and return one [`EmbeddingVector`] per input string,
    /// in the same order.
    ///
    /// Returns `Err(EmbedError::EmptyInput)` when `texts` is empty.
    fn embed(&self, texts: &[&str]) -> Result<Vec<EmbeddingVector>, EmbedError>;

    /// Number of dimensions this model produces.
    ///
    /// Returning `None` means "unknown until first call" (acceptable for
    /// dynamic / API-backed models).
    fn dim(&self) -> Option<usize> {
        None
    }

    /// Human-readable model identifier (e.g. `"text-embedding-3-small"`).
    fn model_id(&self) -> &str;
}

// ── PassthroughEmbedder ───────────────────────────────────────────────────────

/// Deterministic offline stub that returns a single-dimensional vector whose
/// sole component is the normalised byte-sum of the input text.
///
/// Useful in unit tests and CI where no real embedding model is available.
/// **Never use in production** — the vectors carry no semantic meaning.
#[derive(Debug, Default, Clone)]
pub struct PassthroughEmbedder;

impl EmbeddingModel for PassthroughEmbedder {
    fn embed(&self, texts: &[&str]) -> Result<Vec<EmbeddingVector>, EmbedError> {
        if texts.is_empty() {
            return Err(EmbedError::EmptyInput);
        }
        texts
            .iter()
            .map(|t| {
                let byte_sum: u64 = t.bytes().map(u64::from).sum();
                let val = if t.is_empty() {
                    0.0_f32
                } else {
                    (byte_sum as f32) / (t.len() as f32 * 255.0)
                };
                EmbeddingVector::new(vec![val])
                    .ok_or(EmbedError::Backend("passthrough produced empty vector".into()))
            })
            .collect()
    }

    fn dim(&self) -> Option<usize> {
        Some(1)
    }

    fn model_id(&self) -> &str {
        "passthrough-stub-v0"
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // -- EmbeddingVector ------------------------------------------------------

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

    // -- EmbeddingModel / PassthroughEmbedder ---------------------------------

    #[test]
    fn passthrough_returns_one_vec_per_input() {
        let e = PassthroughEmbedder;
        let vecs = e.embed(&["hello", "world", "GAIA"]).unwrap();
        assert_eq!(vecs.len(), 3);
    }

    #[test]
    fn passthrough_dim_is_1() {
        let e = PassthroughEmbedder;
        let vecs = e.embed(&["test"]).unwrap();
        assert_eq!(vecs[0].dim(), 1);
    }

    #[test]
    fn passthrough_empty_input_errors() {
        let e = PassthroughEmbedder;
        assert_eq!(e.embed(&[]), Err(EmbedError::EmptyInput));
    }

    #[test]
    fn passthrough_model_id() {
        assert_eq!(PassthroughEmbedder.model_id(), "passthrough-stub-v0");
    }

    #[test]
    fn passthrough_dim_hint() {
        assert_eq!(PassthroughEmbedder.dim(), Some(1));
    }

    #[test]
    fn passthrough_value_in_unit_range() {
        let e = PassthroughEmbedder;
        let vecs = e.embed(&["hello"]).unwrap();
        let v = vecs[0].as_slice()[0];
        assert!((0.0..=1.0).contains(&v), "value {v} not in [0, 1]");
    }

    #[test]
    fn passthrough_empty_string_gives_zero() {
        let e = PassthroughEmbedder;
        let vecs = e.embed(&[""]).unwrap();
        assert!((vecs[0].as_slice()[0]).abs() < f32::EPSILON);
    }

    #[test]
    fn embed_error_display_empty_input() {
        assert_eq!(EmbedError::EmptyInput.to_string(), "embed called with an empty input slice");
    }

    #[test]
    fn embed_error_display_model_not_loaded() {
        assert_eq!(EmbedError::ModelNotLoaded.to_string(), "embedding model not loaded");
    }

    #[test]
    fn embed_error_display_dimension_mismatch() {
        let e = EmbedError::DimensionMismatch { expected: 1536, actual: 768 };
        assert!(e.to_string().contains("1536"));
        assert!(e.to_string().contains("768"));
    }
}
