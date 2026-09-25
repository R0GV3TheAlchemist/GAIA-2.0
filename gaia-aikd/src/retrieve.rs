//! RAG retrieval pipeline — freshness annotation, provenance passthrough,
//! and optional embedding attachment.
//!
//! ## What changed in Batch B
//!
//! * [`RetrievedChunk`] wraps every retrieved span with the full
//!   [`ChunkMetadata`] provenance block and a pre-computed freshness score.
//! * Chunks whose [`FreshnessVerdict`] is `Stale` are annotated with a
//!   `[STALE]` prefix in their text **before** they reach the generation
//!   layer.
//! * [`GenerationContext`] carries the full `Vec<RetrievedChunk>` so
//!   prompt templates can access `source`, `confidence`, `date`, etc.
//!
//! ## What changed in Batch C (embedding)
//!
//! * [`RetrievedChunk`] gains an `embedding: Option<EmbeddingVector>` field
//!   that carries the chunk's dense vector when one is available.
//! * [`embed_query`] provides the query-side counterpart: given a query
//!   string and an [`EmbeddingModel`] it returns the query vector ready for
//!   cosine similarity ranking.
//! * [`GenerationContext::build_with_embeddings`] passes stored chunk
//!   embeddings through authorization and freshness annotation.
//!
//! The lower-level [`QueryHit`] / [`Span`] types are retained unchanged
//! for backwards compatibility with existing callers.

use gaia_ingest::auth::{AgentId, ChunkMetadata, RetrievalFilter, RetrievalReport};
use gaia_ingest::embed::{EmbedError, EmbeddingModel, EmbeddingVector};
use gaia_ingest::freshness::{evaluate, freshness_score, FreshnessVerdict};

use crate::{AikdError, Layer};

// ── Legacy types (unchanged) ───────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub layer: Layer,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryHit {
    pub parametric: Span,
    pub retrieved: Span,
    pub used_network: bool,
}

impl QueryHit {
    pub fn offline(question: &str) -> Result<Self, AikdError> {
        if question.is_empty() {
            return Err(AikdError::CannotKnow);
        }
        Ok(Self {
            parametric: Span {
                layer: Layer::Weights,
                text: "parametric-fixture".into(),
            },
            retrieved: Span {
                layer: Layer::Retrieved,
                text: format!("bundled-corpus:{question}"),
            },
            used_network: false,
        })
    }
}

// ── RetrievedChunk ──────────────────────

/// A single chunk after retrieval, freshness annotation, provenance
/// passthrough, and optional embedding attachment.
#[derive(Debug, Clone, PartialEq)]
pub struct RetrievedChunk {
    /// Chunk text, `[STALE] `-prefixed when `is_stale` is `true`.
    pub text: String,
    /// `true` when `FreshnessVerdict::Stale` was returned for this chunk.
    pub is_stale: bool,
    /// Linear freshness decay score in `[0.0, 1.0]`.
    pub freshness_score: f32,
    /// Full provenance block for prompt weighting and attribution.
    pub metadata: ChunkMetadata,
    /// Dense embedding vector for this chunk, when available.
    ///
    /// `None` when the retrieval path did not embed the chunk (e.g. keyword
    /// search, BM25).  `Some` when the chunk was retrieved via ANN / vector
    /// search and the vector was retained, or when the pipeline explicitly
    /// embedded the chunk post-retrieval.
    pub embedding: Option<EmbeddingVector>,
}

impl RetrievedChunk {
    /// Build a [`RetrievedChunk`] from raw retrieval output.
    ///
    /// Pass `embedding: None` when no vector is available (e.g. keyword
    /// retrieval).  Pass `embedding: Some(v)` when the vector is known.
    pub fn build(
        raw_text: &str,
        ttl_seconds: Option<u64>,
        ingested_at: u64,
        now: u64,
        metadata: ChunkMetadata,
    ) -> Self {
        let verdict = evaluate(ttl_seconds, ingested_at, now);
        let is_stale = verdict == FreshnessVerdict::Stale;
        let score = freshness_score(ttl_seconds, ingested_at, now);
        let text = if is_stale {
            format!("[STALE] {raw_text}")
        } else {
            raw_text.to_string()
        };
        Self { text, is_stale, freshness_score: score, metadata, embedding: None }
    }

    /// Build a [`RetrievedChunk`] and attach a pre-computed embedding.
    pub fn build_with_embedding(
        raw_text: &str,
        ttl_seconds: Option<u64>,
        ingested_at: u64,
        now: u64,
        metadata: ChunkMetadata,
        embedding: EmbeddingVector,
    ) -> Self {
        let mut chunk = Self::build(raw_text, ttl_seconds, ingested_at, now, metadata);
        chunk.embedding = Some(embedding);
        chunk
    }
}

// ── embed_query ─────────────────────────

/// Embed a query string using `embedder` and return the resulting vector.
///
/// This is the query-side counterpart of the document embedding done inside
/// [`gaia_ingest::IngestPipeline`].  The returned vector is ready for
/// cosine similarity ranking against stored chunk embeddings.
///
/// # Errors
/// Propagates any [`EmbedError`] returned by `embedder.embed`.
///
/// # Example
/// ```rust
/// use gaia_ingest::embed::PassthroughEmbedder;
/// use gaia_aikd::embed_query;
///
/// let embedder = PassthroughEmbedder;
/// let vec = embed_query("what is the Earth Twin?", &embedder).unwrap();
/// assert_eq!(vec.dim(), 1);
/// ```
pub fn embed_query(
    query: &str,
    embedder: &dyn EmbeddingModel,
) -> Result<EmbeddingVector, EmbedError> {
    let mut vecs = embedder.embed(&[query])?;
    Ok(vecs.remove(0))
}

// ── GenerationContext ─────────────────────

/// Candidate row for [`GenerationContext::build_with_embeddings`].
///
/// Fields: `(raw_text, ttl_seconds, ingested_at, now, metadata, embedding)`.
pub type EmbeddedCandidate = (
    String,
    Option<u64>,
    u64,
    u64,
    ChunkMetadata,
    Option<EmbeddingVector>,
);

#[derive(Debug, Clone, PartialEq)]
pub struct GenerationContext {
    pub chunks: Vec<RetrievedChunk>,
    pub caller: AgentId,
    pub report: RetrievalReport,
}

impl GenerationContext {
    /// Build a [`GenerationContext`] from a candidate chunk list.
    ///
    /// Applies [`RetrievalFilter`] authorization, freshness annotation, and
    /// `[STALE]` prefixing in a single pass. Embeddings default to `None`.
    ///
    /// `candidates` — `(raw_text, ttl_seconds, ingested_at, now, metadata)` tuples.
    pub fn build(
        caller: AgentId,
        candidates: Vec<(String, Option<u64>, u64, u64, ChunkMetadata)>,
    ) -> Self {
        Self::build_with_embeddings(
            caller,
            candidates
                .into_iter()
                .map(|(text, ttl, ingested_at, now, metadata)| {
                    (text, ttl, ingested_at, now, metadata, None)
                })
                .collect(),
        )
    }

    /// Build a [`GenerationContext`] and pass through optional embeddings.
    ///
    /// Same authorization and freshness rules as [`GenerationContext::build`].
    /// Use this when ingest already populated `DocumentChunk.embedding`.
    pub fn build_with_embeddings(
        caller: AgentId,
        candidates: Vec<EmbeddedCandidate>,
    ) -> Self {
        let filter = RetrievalFilter::new(caller.clone());
        let mut report = RetrievalReport::default();
        let mut chunks = Vec::with_capacity(candidates.len());

        for (raw_text, ttl, ingested_at, now, metadata, embedding) in candidates {
            if !filter.is_authorized(&metadata, &mut report) {
                continue;
            }
            report.retrieved += 1;
            let mut chunk = RetrievedChunk::build(
                &raw_text,
                ttl,
                ingested_at,
                now,
                metadata,
            );
            chunk.embedding = embedding;
            chunks.push(chunk);
        }

        Self { chunks, caller, report }
    }
}

// ── Tests ─────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use gaia_ingest::auth::ChunkMetadata;
    use gaia_ingest::embed::{EmbeddingVector, PassthroughEmbedder};

    fn meta(authorized_for: Vec<&str>) -> ChunkMetadata {
        ChunkMetadata {
            source:         "test-source".into(),
            date:           "2026-01-01".into(),
            author:         None,
            domain:         "test-domain".into(),
            confidence:     0.9,
            version:        "1".into(),
            authorized_for: authorized_for
                .into_iter()
                .map(|s| AgentId::new(s))
                .collect(),
        }
    }

    const NOW: u64 = 1_000_000;
    const TTL: u64 = 3_600;

    // ── #941: stale flag ──────────────────

    #[test]
    fn stale_chunk_is_annotated() {
        let ingested = NOW - TTL - 1;
        let chunk = RetrievedChunk::build("hello", Some(TTL), ingested, NOW, meta(vec![]));
        assert!(chunk.is_stale);
        assert!(chunk.text.starts_with("[STALE] "));
    }

    #[test]
    fn fresh_chunk_passes_clean() {
        let ingested = NOW - TTL / 2;
        let chunk = RetrievedChunk::build("hello", Some(TTL), ingested, NOW, meta(vec![]));
        assert!(!chunk.is_stale);
        assert_eq!(chunk.text, "hello");
    }

    #[test]
    fn no_ttl_chunk_passes_clean() {
        let chunk = RetrievedChunk::build("hello", None, 0, NOW, meta(vec![]));
        assert!(!chunk.is_stale);
        assert_eq!(chunk.text, "hello");
        assert!((chunk.freshness_score - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn freshness_score_is_attached() {
        let ingested = NOW - TTL / 2;
        let chunk = RetrievedChunk::build("hi", Some(TTL), ingested, NOW, meta(vec![]));
        assert!((chunk.freshness_score - 0.5).abs() < 1e-5);
    }

    // ── #943: provenance passthrough ───────────────

    #[test]
    fn metadata_present_on_retrieval_output() {
        let chunk = RetrievedChunk::build("data", None, 0, NOW, meta(vec![]));
        assert_eq!(chunk.metadata.source, "test-source");
        assert_eq!(chunk.metadata.domain, "test-domain");
    }

    #[test]
    fn all_provenance_fields_populated() {
        let m = meta(vec![]);
        assert!(!m.source.is_empty());
        assert!(!m.date.is_empty());
        assert!(!m.domain.is_empty());
        assert!(m.confidence >= 0.0 && m.confidence <= 1.0);
        assert!(!m.version.is_empty());
    }

    #[test]
    fn confidence_accessible_from_chunk() {
        let chunk = RetrievedChunk::build("data", None, 0, NOW, meta(vec![]));
        assert!((chunk.metadata.confidence - 0.9).abs() < f32::EPSILON);
    }

    #[test]
    fn stale_annotation_does_not_bleed_metadata() {
        let ingested = NOW - TTL - 1;
        let chunk = RetrievedChunk::build("content", Some(TTL), ingested, NOW, meta(vec![]));
        assert!(!chunk.metadata.source.contains("[STALE]"));
        assert!(!chunk.metadata.domain.contains("[STALE]"));
    }

    // ── embedding field ───────────────────

    #[test]
    fn build_embedding_defaults_none() {
        let chunk = RetrievedChunk::build("data", None, 0, NOW, meta(vec![]));
        assert!(chunk.embedding.is_none());
    }

    #[test]
    fn build_with_embedding_attaches_vector() {
        let ev = EmbeddingVector::new(vec![0.5_f32, 0.5]).unwrap();
        let chunk = RetrievedChunk::build_with_embedding(
            "data", None, 0, NOW, meta(vec![]), ev.clone(),
        );
        assert_eq!(chunk.embedding.as_ref().unwrap().as_slice(), ev.as_slice());
    }

    #[test]
    fn build_with_embedding_does_not_affect_staleness() {
        let ev = EmbeddingVector::new(vec![0.1]).unwrap();
        let ingested = NOW - TTL - 1;
        let chunk = RetrievedChunk::build_with_embedding(
            "content", Some(TTL), ingested, NOW, meta(vec![]), ev,
        );
        assert!(chunk.is_stale);
        assert!(chunk.text.starts_with("[STALE] "));
    }

    // ── embed_query ──────────────────────

    #[test]
    fn embed_query_returns_vector() {
        let e = PassthroughEmbedder;
        let v = embed_query("what is the Earth Twin?", &e).unwrap();
        assert_eq!(v.dim(), 1);
    }

    #[test]
    fn embed_query_empty_string_does_not_error() {
        // Empty string is a valid single text; EmptyInput only fires on &[]
        let e = PassthroughEmbedder;
        let v = embed_query("", &e).unwrap();
        assert_eq!(v.dim(), 1);
    }

    // ── GenerationContext ─────────────────

    #[test]
    fn unauthorized_chunk_excluded_from_context() {
        let caller = AgentId::new("agent-a");
        let candidates = vec![
            ("open".into(), None, 0, NOW, meta(vec![])),
            ("secret".into(), None, 0, NOW, meta(vec!["agent-b"])),
        ];
        let ctx = GenerationContext::build(caller, candidates);
        assert_eq!(ctx.chunks.len(), 1);
        assert_eq!(ctx.report.retrieved, 1);
        assert_eq!(ctx.report.unauthorized_excluded, 1);
        assert_eq!(ctx.chunks[0].text, "open");
    }

    #[test]
    fn authorized_chunk_included_in_context() {
        let caller = AgentId::new("agent-a");
        let candidates = vec![
            ("data".into(), None, 0, NOW, meta(vec!["agent-a"])),
        ];
        let ctx = GenerationContext::build(caller, candidates);
        assert_eq!(ctx.chunks.len(), 1);
        assert_eq!(ctx.report.unauthorized_excluded, 0);
    }

    #[test]
    fn build_leaves_embedding_none() {
        let caller = AgentId::new("agent-a");
        let candidates = vec![("data".into(), None, 0, NOW, meta(vec![]))];
        let ctx = GenerationContext::build(caller, candidates);
        assert!(ctx.chunks[0].embedding.is_none());
    }

    #[test]
    fn build_with_embeddings_passthrough() {
        let caller = AgentId::new("agent-a");
        let ev = EmbeddingVector::new(vec![0.25_f32, 0.75]).unwrap();
        let candidates = vec![(
            "data".into(),
            None,
            0,
            NOW,
            meta(vec![]),
            Some(ev.clone()),
        )];
        let ctx = GenerationContext::build_with_embeddings(caller, candidates);
        assert_eq!(ctx.chunks.len(), 1);
        assert_eq!(ctx.chunks[0].embedding.as_ref().unwrap().as_slice(), ev.as_slice());
    }

    #[test]
    fn build_with_embeddings_still_excludes_unauthorized() {
        let caller = AgentId::new("agent-a");
        let ev = EmbeddingVector::new(vec![0.1]).unwrap();
        let candidates = vec![
            ("open".into(), None, 0, NOW, meta(vec![]), Some(ev.clone())),
            ("secret".into(), None, 0, NOW, meta(vec!["agent-b"]), Some(ev)),
        ];
        let ctx = GenerationContext::build_with_embeddings(caller, candidates);
        assert_eq!(ctx.chunks.len(), 1);
        assert_eq!(ctx.chunks[0].text, "open");
        assert!(ctx.chunks[0].embedding.is_some());
        assert_eq!(ctx.report.unauthorized_excluded, 1);
    }
}
