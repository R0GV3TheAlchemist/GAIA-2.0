//! `gaia-ingest` — Earth Twin ingestion schema.
//!
//! This crate defines the **canonical data contract** for every byte that
//! enters the GAIA Earth Twin from an external source.  It is intentionally
//! schema-only in slice 1: no HTTP clients, no database drivers, no live
//! network calls.  Adapter crates (Copernicus, NOAA, GBIF, SensorThings)
//! will depend on this crate and implement the `ArtifactStore` trait.
//!
//! ## Dependency rule
//! `gaia-ingest` MAY depend on `gaia-earth` (for `SystemTwin` / `SourceKind`)
//! and `gaia-sfs` (for artifact storage).  It MUST NOT be depended on by
//! `gaia-earth` — the dependency arrow is one-way.
//!
//! ## Safety contract
//! Every `NormalizedObservation` that leaves this crate carries a
//! `ProvenanceReceipt` with a SHA-256 hash of the raw bytes it was derived
//! from.  This makes every ingested datum auditable back to its exact source
//! payload, which is a hard requirement of #729.
//!
//! Every `DocumentChunk` seals provenance over the raw source bytes of the
//! **parent document**, not the chunk text, for the same auditability guarantee
//! (#909).
//!
//! ## Lexicon plane contract
//! Every `DocumentChunk` carries a `lexicon_plane` field (`Order`, `Chaos`,
//! or `Bridge`) so the RAG pipeline always knows which ontological plane it
//! is pulling from. `Bridge` is the safe default — it signals unresolved
//! provenance, not an error. `SlidingWindowChunker::chunk()` calls
//! `classify_document_chunk()` on every emitted chunk so that `lexicon_plane`
//! and `lexicon_voice` are resolved at ingest time. The retrieval layer must
//! refuse implicit cross-plane lookups (C30: no silent failures).
//!
//! ## Epistemic state contract (#953)
//! Every `DocumentChunk` MAY carry an `epistemic_state` field populated by
//! the classify step. `None` is valid at ingest time. When present, the inner
//! `claim_status` determines retrieval eligibility: `Retracted` chunks MUST
//! NOT be returned by the retrieval layer (C30). Downstream consumers:
//! - `#932` (RAG grounding): weight retrieval by `EpistemicConfidence`.
//! - `#952` (multi-model ACP): detect inter-model contradictions via
//!   `ContradictionRef`.
//!
//! ## Text ingestion
//! Use [`IngestPipeline::from_path`] to ingest a local file in one call.
//! Use [`ChunkId`] to fingerprint individual chunks for deduplication and
//! cache keying.  Use [`dedup::ChunkStore`] to track already-ingested chunks
//! and prevent duplicates.  Use [`freshness`] to evaluate TTL staleness.
//! Use [`auth::RetrievalFilter`] to enforce per-chunk authorization at
//! retrieval time.  Use [`chunking::MarkdownChunker`] for structure-preserving
//! Markdown splitting.
//!
//! ## Embedding
//! `DocumentChunk::embedding` is `None` immediately after chunking and is
//! populated by the embed step before the chunk is inserted into the vector
//! store.  Use [`EmbeddingVector`] to construct and validate embedding values.

pub mod artifact;
pub mod auth;
pub mod chunk_id;
pub mod chunker;
pub mod chunking;
pub mod dedup;
pub mod document;
pub mod embed;
pub mod epistemic;
pub mod freshness;
pub mod ingest;
pub mod lexicon;
pub mod provenance;
pub mod schema;

// artifact::IngestError is the store-layer error (EmptyPayload, InvalidPath, Store).
// ingest::PipelineError is the file-ingestion error (Io, NoFileStem, Provenance, Chunking).
// They are distinct types with distinct names to avoid any ambiguity at the call site.
pub use artifact::{ArtifactStore, IngestError, RawArtifactRef};
pub use auth::{AgentId, ChunkMetadata, RetrievalFilter, RetrievalReport};
pub use chunk_id::ChunkId;
pub use chunker::{ChunkError, Chunker, SlidingWindowChunker};
pub use chunking::{ChunkingStrategy, MarkdownChunker, StructuredChunk};
pub use dedup::{ChunkStore, IngestReport, IngestResult};
pub use document::{
    AccessTier, ConfidenceTier, DocumentChunk, DocumentKind,
};
pub use embed::EmbeddingVector;
pub use epistemic::{
    ClaimStatus, ContradictionRef, EpistemicConfidence, EpistemicError,
    EpistemicState, EpistemicStateBuilder, EvidenceKind,
};
pub use freshness::{evaluate as evaluate_freshness, freshness_score, FreshnessVerdict};
pub use ingest::{IngestPipeline, PipelineError};
pub use lexicon::{
    classify_chunk, classify_document_chunk, LexiconPlane, LexiconSignals, LexiconVoice,
};
pub use provenance::{ProvenanceBuilder, ProvenanceReceipt};
pub use schema::{
    DataSource, GeoBBox, GeoPoint, NormalizedObservation, ObservationKind,
};
