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

pub mod artifact;
pub mod chunker;
pub mod document;
pub mod lexicon;
pub mod provenance;
pub mod schema;

pub use artifact::{ArtifactStore, IngestError, RawArtifactRef};
pub use chunker::{ChunkError, Chunker, SlidingWindowChunker};
pub use document::{
    AccessTier, ConfidenceTier, DocumentChunk, DocumentKind,
};
pub use lexicon::{
    classify_chunk, classify_document_chunk, LexiconPlane, LexiconSignals, LexiconVoice,
};
pub use provenance::{ProvenanceBuilder, ProvenanceReceipt};
pub use schema::{
    DataSource, GeoBBox, GeoPoint, NormalizedObservation, ObservationKind,
};
