//! `gaia-ingest` — Earth Twin ingestion schema.

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
pub mod hash_embed;
pub mod ingest;
pub mod lexicon;
pub mod persist;
pub mod provenance;
pub mod schema;

pub use artifact::{ArtifactStore, IngestError, RawArtifactRef};
pub use auth::{AgentId, ChunkMetadata, RetrievalFilter, RetrievalReport};
pub use chunk_id::ChunkId;
pub use chunker::{ChunkError, Chunker, SlidingWindowChunker};
pub use chunking::{ChunkingStrategy, MarkdownChunker, StructuredChunk};
pub use dedup::{ChunkStore, IngestReport, IngestResult};
pub use document::{
    AccessTier, ConfidenceTier, DocumentChunk, DocumentKind,
};
pub use embed::{EmbedError, EmbeddingModel, EmbeddingVector, PassthroughEmbedder};
pub use epistemic::{
    ClaimStatus, ContradictionRef, EpistemicConfidence, EpistemicError,
    EpistemicState, EpistemicStateBuilder, EvidenceKind,
};
pub use freshness::{evaluate as evaluate_freshness, freshness_score, FreshnessVerdict};
pub use hash_embed::{cosine as hashing_cosine, HashingEmbedder};
pub use ingest::{IngestPipeline, PipelineError};
pub use lexicon::{
    classify_chunk, classify_document_chunk, LexiconPlane, LexiconSignals, LexiconVoice,
};
pub use persist::{FileChunkStore, PersistedChunk};
pub use provenance::{ProvenanceBuilder, ProvenanceReceipt};
pub use schema::{
    DataSource, GeoBBox, GeoPoint, NormalizedObservation, ObservationKind,
};
