//! RAG retrieval pipeline — freshness annotation and provenance passthrough.
//!
//! ## What changed in Batch B
//!
//! * [`RetrievedChunk`] wraps every retrieved span with the full
//!   [`ChunkMetadata`] provenance block and a pre-computed
//!   [`freshness_score`].
//! * Chunks whose [`FreshnessVerdict`] is `Stale` are annotated with a
//!   `[STALE]` prefix in their text **before** they reach the generation
//!   layer, so the LLM call-site can see staleness without inspecting
//!   metadata.
//! * [`GenerationContext`] carries the full `Vec<RetrievedChunk>` so
//!   prompt templates can access `source`, `confidence`, `date`, etc.
//!
//! The lower-level [`QueryHit`] / [`Span`] types are retained unchanged
//! for backwards compatibility with existing callers.

use gaia_ingest::auth::{AgentId, ChunkMetadata, RetrievalFilter, RetrievalReport};
use gaia_ingest::freshness::{evaluate, freshness_score, FreshnessVerdict};

use crate::{AikdError, Layer};

// ── Legacy types (unchanged) ─────────────────────────────────────────────────

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

// ── RetrievedChunk ────────────────────────────────────────────────────────────

/// A single chunk after retrieval, freshness annotation, and provenance
/// passthrough.
///
/// `text` is the content as it will be presented to the generation layer.
/// When the chunk is stale, `text` is prefixed with `[STALE] ` so prompt
/// templates do not need to inspect `is_stale` unless they want to handle
/// staleness specially.
#[derive(Debug, Clone, PartialEq)]
pub struct RetrievedChunk {
    /// Chunk text, `[STALE] `-prefixed when `is_stale` is `true`.
    pub text: String,
    /// `true` when `FreshnessVerdict::Stale` was returned for this chunk.
    pub is_stale: bool,
    /// Linear freshness decay score in `[0.0, 1.0]`.
    /// `1.0` = just ingested; `0.0` = TTL expired; `None`-TTL chunks = `1.0`.
    pub freshness_score: f32,
    /// Full provenance block for prompt weighting and attribution.
    pub metadata: ChunkMetadata,
}

impl RetrievedChunk {
    /// Build a [`RetrievedChunk`] from raw chunk data.
    ///
    /// `raw_text`        — the chunk body before any annotation.
    /// `ttl_seconds`     — from `DocumentChunk::ttl_seconds`.
    /// `ingested_at`     — Unix timestamp when the chunk was stored.
    /// `now`             — current Unix timestamp.
    /// `metadata`        — provenance fields.
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
        Self {
            text,
            is_stale,
            freshness_score: score,
            metadata,
        }
    }
}

// ── GenerationContext ─────────────────────────────────────────────────────────

/// The context bundle handed to the generation layer.
///
/// `chunks` carries every retrieved chunk with its provenance and freshness
/// annotation.  Prompt templates can iterate `chunks` to inject `source`,
/// `confidence`, `date`, etc.  Stale chunks are already pre-annotated in
/// `chunk.text` — templates do not need to check `is_stale` unless they
/// want to handle them differently (e.g., display a warning UI).
#[derive(Debug, Clone, PartialEq)]
pub struct GenerationContext {
    /// Retrieved chunks, ordered by retrieval rank.
    pub chunks: Vec<RetrievedChunk>,
    /// The caller agent identity used for this retrieval pass.
    pub caller: AgentId,
    /// Authorization report: how many chunks were retrieved vs. excluded.
    pub report: RetrievalReport,
}

impl GenerationContext {
    /// Build a [`GenerationContext`] from a candidate chunk list.
    ///
    /// Applies [`RetrievalFilter`] authorization, freshness annotation, and
    /// `[STALE]` prefixing in a single pass.  Unauthorized chunks are
    /// silently dropped and counted in `report.unauthorized_excluded`.
    ///
    /// `candidates` — `(raw_text, ttl_seconds, ingested_at, now, metadata)`
    ///                tuples, one per candidate chunk.
    pub fn build(
        caller: AgentId,
        candidates: Vec<(String, Option<u64>, u64, u64, ChunkMetadata)>,
    ) -> Self {
        let filter = RetrievalFilter::new();
        let mut report = RetrievalReport::default();
        let mut chunks = Vec::with_capacity(candidates.len());

        for (raw_text, ttl, ingested_at, now, metadata) in candidates {
            if !filter.is_authorized(&caller, &metadata) {
                report.unauthorized_excluded += 1;
                continue;
            }
            report.retrieved += 1;
            chunks.push(RetrievedChunk::build(
                &raw_text,
                ttl,
                ingested_at,
                now,
                metadata,
            ));
        }

        Self { chunks, caller, report }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use gaia_ingest::auth::ChunkMetadata;

    fn meta(authorized_for: Vec<&str>) -> ChunkMetadata {
        ChunkMetadata {
            source: "test-source".into(),
            date: "2026-01-01".into(),
            author: "test-author".into(),
            domain: "test-domain".into(),
            confidence: 0.9,
            version: "1".into(),
            authorized_for: authorized_for
                .into_iter()
                .map(|s| AgentId::new(s))
                .collect(),
        }
    }

    const NOW: u64 = 1_000_000;
    const TTL: u64 = 3_600;

    // ── #941: stale flag ──────────────────────────────────────────────────

    #[test]
    fn stale_chunk_is_annotated() {
        let ingested = NOW - TTL - 1; // one second past TTL
        let chunk = RetrievedChunk::build("hello", Some(TTL), ingested, NOW, meta(vec![]));
        assert!(chunk.is_stale);
        assert!(chunk.text.starts_with("[STALE] "));
    }

    #[test]
    fn fresh_chunk_passes_clean() {
        let ingested = NOW - TTL / 2;
        let chunk = RetrievedChunk::build("hello", Some(TTL), ingested, NOW, meta(vec![]));
        assert!(!chunk.is_stale);
        assert!(!chunk.text.starts_with("[STALE]"));
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

    // ── #943: provenance passthrough ──────────────────────────────────────

    #[test]
    fn metadata_present_on_retrieval_output() {
        let chunk = RetrievedChunk::build("data", None, 0, NOW, meta(vec![]));
        assert_eq!(chunk.metadata.source, "test-source");
        assert_eq!(chunk.metadata.author, "test-author");
    }

    #[test]
    fn all_six_provenance_fields_populated() {
        let m = meta(vec![]);
        assert!(!m.source.is_empty());
        assert!(!m.date.is_empty());
        assert!(!m.author.is_empty());
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
        // The [STALE] prefix must only appear in .text, not in any metadata field
        let ingested = NOW - TTL - 1;
        let chunk = RetrievedChunk::build("content", Some(TTL), ingested, NOW, meta(vec![]));
        assert!(!chunk.metadata.source.contains("[STALE]"));
        assert!(!chunk.metadata.author.contains("[STALE]"));
    }

    // ── GenerationContext (auth + stale + provenance wired together) ───────

    #[test]
    fn unauthorized_chunk_excluded_from_context() {
        let caller = AgentId::new("agent-a");
        let candidates = vec![
            // unrestricted
            ("open".into(), None, 0, NOW, meta(vec![])),
            // restricted to agent-b only
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
}
