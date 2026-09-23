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

pub mod artifact;
pub mod document;
pub mod provenance;
pub mod schema;

pub use artifact::{ArtifactStore, IngestError, RawArtifactRef};
pub use document::{
    AccessTier, ConfidenceTier, DocumentChunk, DocumentKind,
};
pub use provenance::{ProvenanceBuilder, ProvenanceReceipt};
pub use schema::{
    DataSource, GeoBBox, GeoPoint, NormalizedObservation, ObservationKind,
};
