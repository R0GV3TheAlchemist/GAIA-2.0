//! `IngestPipeline` — file-to-chunks entry point for local documents.
//!
//! This module provides the thin glue between a file on disk and
//! [`SlidingWindowChunker`].  It is intentionally minimal: it reads the
//! file, seals a [`ProvenanceReceipt`] over the raw bytes, constructs a
//! [`DocumentChunk`] template, and delegates all splitting decisions to
//! the chunker.
//!
//! After chunking, if an [`EmbeddingModel`] is provided via the `embedder`
//! field, each chunk's `embedding` field is populated in a single batched
//! call to [`EmbeddingModel::embed`].
//!
//! ## Usage
//!
//! ```rust,no_run
//! use std::path::PathBuf;
//! use gaia_ingest::IngestPipeline;
//!
//! // Without embedding (existing behaviour, unchanged)
//! let chunks = IngestPipeline::default()
//!     .from_path(PathBuf::from("docs/tablets/TERRA.md"))
//!     .expect("ingestion failed");
//!
//! println!("Produced {} chunks", chunks.len());
//! ```
//!
//! ```rust,no_run
//! use gaia_ingest::{IngestPipeline, embed::PassthroughEmbedder};
//!
//! // With embedding
//! let pipeline = IngestPipeline {
//!     embedder: Some(Box::new(PassthroughEmbedder)),
//!     ..Default::default()
//! };
//! let chunks = pipeline.from_path("docs/tablets/TERRA.md").unwrap();
//! assert!(chunks[0].embedding.is_some());
//! ```
//!
//! ## Template auto-fill rules
//!
//! | Field | Value |
//! |---|---|
//! | `document_uri` | `file://` + canonicalised absolute path |
//! | `document_title` | File stem (e.g. `TERRA` from `TERRA.md`) |
//! | `kind` | `DocumentKind::CanonTablet` if extension = `.md`, else `DocumentKind::SpecDocument` |
//! | `domain` | File extension without leading dot (e.g. `md`, `txt`) |
//! | `language` | `"en"` |
//! | `provenance.source` | `DataSource::CanonTablet` for `.md`, `DataSource::InternalDocument` for all others |
//! | `provenance.sha256` | SHA-256 of raw file bytes |
//! | `provenance.fetched_at_unix` | Current Unix time |
//! | `provenance.observed_at_unix` | File `mtime` (falls back to `fetched_at_unix`) |
//! | `authored_at_unix` | `Some(mtime)` — `None` when mtime is unavailable |
//! | `lexicon_plane` | `LexiconPlane::Bridge` (resolved by chunker after splitting) |
//! | `lexicon_voice` | `None` (resolved by `classify_document_chunk()`) |
//! | `confidence` | `ConfidenceTier::Canon` |
//! | `access_tier` | `AccessTier::Public` |

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    chunker::{ChunkError, Chunker, SlidingWindowChunker},
    document::{
        AccessTier, ConfidenceTier, DocumentChunk, DocumentKind,
    },
    embed::{EmbeddingModel, EmbedError},
    lexicon::LexiconPlane,
    provenance::ProvenanceBuilder,
    schema::DataSource,
};

// ── PipelineError ───────────────────────────────────────────────────────────

/// Errors produced by [`IngestPipeline`].
///
/// Named `PipelineError` (not `IngestError`) to avoid a name collision with
/// [`crate::artifact::IngestError`], which is separately re-exported from
/// `lib.rs`.
#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("I/O error reading {path}: {source}")]
    Io {
        path:   PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("path has no file stem: {0}")]
    NoFileStem(PathBuf),
    #[error("path is not valid UTF-8: {0}")]
    NonUtf8Path(PathBuf),
    #[error("provenance error: {0}")]
    Provenance(#[from] crate::provenance::ProvenanceError),
    #[error("chunking error: {0}")]
    Chunking(#[from] ChunkError),
    #[error("embedding error: {0}")]
    Embedding(#[from] EmbedError),
}

// ── IngestPipeline ────────────────────────────────────────────────────────────

/// A thin pipeline that reads a file from disk and returns chunked
/// [`DocumentChunk`] records ready for embedding and retrieval.
///
/// Configure by replacing the public fields before calling
/// [`IngestPipeline::from_path`].
///
/// ```rust,no_run
/// use gaia_ingest::{IngestPipeline, SlidingWindowChunker, embed::PassthroughEmbedder};
///
/// let pipeline = IngestPipeline {
///     chunker: SlidingWindowChunker {
///         target_chars: 1_600,
///         overlap_fraction: 0.12,
///         inject_heading_prefix: true,
///     },
///     embedder: Some(Box::new(PassthroughEmbedder)),
/// };
/// ```
pub struct IngestPipeline {
    /// The chunking strategy to apply after reading the file.
    pub chunker: SlidingWindowChunker,
    /// Optional embedding model.  When `Some`, every chunk produced by
    /// `from_path` will have its `embedding` field populated via a single
    /// batched call to [`EmbeddingModel::embed`].
    ///
    /// Defaults to `None` — existing callers are unaffected.
    pub embedder: Option<Box<dyn EmbeddingModel>>,
}

impl Default for IngestPipeline {
    fn default() -> Self {
        Self {
            chunker: SlidingWindowChunker::default(),
            embedder: None,
        }
    }
}

impl std::fmt::Debug for IngestPipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IngestPipeline")
            .field("chunker", &self.chunker)
            .field("embedder", &self.embedder.as_ref().map(|e| e.model_id()))
            .finish()
    }
}

impl IngestPipeline {
    /// Read `path`, build a template, and return the chunked
    /// [`DocumentChunk`] records.
    ///
    /// The caller supplies only a path; all template fields are derived
    /// automatically — see the module-level table for the exact rules.
    ///
    /// If `self.embedder` is `Some`, all chunks are embedded in a single
    /// batched call after chunking and their `embedding` fields are
    /// populated before returning.
    pub fn from_path(&self, path: impl AsRef<Path>) -> Result<Vec<DocumentChunk>, PipelineError> {
        let path = path.as_ref();

        // ── 1. Read raw bytes ─────────────────────────────────────────────────
        let raw = std::fs::read(path).map_err(|e| PipelineError::Io {
            path:   path.to_path_buf(),
            source: e,
        })?;

        // ── 2. Decode as UTF-8 ────────────────────────────────────────────────
        let text = String::from_utf8_lossy(&raw).into_owned();

        // ── 3. Timestamps ─────────────────────────────────────────────────────
        let fetched_at = unix_now();
        let observed_at = mtime_unix(path).unwrap_or(fetched_at).min(fetched_at);

        // ── 4. Derive metadata from the path ──────────────────────────────────
        let canonical = path
            .canonicalize()
            .unwrap_or_else(|_| path.to_path_buf());

        let document_uri = format!(
            "file://{}",
            canonical
                .to_str()
                .ok_or_else(|| PipelineError::NonUtf8Path(path.to_path_buf()))?
        );

        let document_title = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PipelineError::NoFileStem(path.to_path_buf()))?
            .to_string();

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("txt")
            .to_lowercase();

        let (data_source, doc_kind) = if ext == "md" {
            (DataSource::CanonTablet, DocumentKind::CanonTablet)
        } else {
            (DataSource::InternalDocument, DocumentKind::SpecDocument)
        };

        // ── 5. Seal provenance ────────────────────────────────────────────────
        let provenance = ProvenanceBuilder::new(
            data_source,
            &document_uri,
            &document_title,
            fetched_at,
            observed_at,
        )
        .seal(&raw)?;

        // ── 6. Build the template ─────────────────────────────────────────────
        let template = DocumentChunk {
            id:              String::new(),
            text:            String::new(),
            char_count:      0,
            chunk_index:     0,
            total_chunks:    0,
            document_title,
            document_uri,
            kind:            doc_kind,
            domain:          ext,
            language:        "en".into(),
            authored_at_unix: Some(observed_at),
            ttl_seconds:     None,
            confidence:      ConfidenceTier::Canon,
            access_tier:     AccessTier::Public,
            access_control:  Vec::new(),
            attributes:      BTreeMap::new(),
            lexicon_plane:   LexiconPlane::Bridge,
            lexicon_voice:   None,
            provenance,
            artifact:        None,
            embedding:       None,
            epistemic_state: None,
        };

        // ── 7. Chunk ──────────────────────────────────────────────────────────
        let mut chunks = self.chunker.chunk(&text, template)?;

        // ── 8. Embed (optional) ───────────────────────────────────────────────
        if let Some(embedder) = &self.embedder {
            let texts: Vec<&str> = chunks.iter().map(|c| c.text.as_str()).collect();
            let vectors = embedder.embed(&texts)?;
            for (chunk, vec) in chunks.iter_mut().zip(vectors) {
                chunk.embedding = Some(vec);
            }
        }

        Ok(chunks)
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn mtime_unix(path: &Path) -> Option<u64> {
    path.metadata()
        .ok()?
        .modified()
        .ok()?
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embed::PassthroughEmbedder;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn tmp(content: &str, ext: &str) -> NamedTempFile {
        let mut f = tempfile::Builder::new()
            .suffix(&format!(".{ext}"))
            .tempfile()
            .unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f.flush().unwrap();
        f
    }

    fn long_md() -> String {
        let body = "This is a sentence about GAIA. ".repeat(60);
        format!("# Test Document\n\n{body}")
    }

    #[test]
    fn ingest_md_produces_chunks() {
        let f = tmp(&long_md(), "md");
        let chunks = IngestPipeline::default().from_path(f.path()).unwrap();
        assert!(!chunks.is_empty());
    }

    #[test]
    fn md_kind_is_canon_tablet() {
        let f = tmp(&long_md(), "md");
        let chunks = IngestPipeline::default().from_path(f.path()).unwrap();
        assert!(chunks.iter().all(|c| c.kind == DocumentKind::CanonTablet));
    }

    #[test]
    fn txt_kind_is_spec_document() {
        let body = "This is a sentence. ".repeat(60);
        let f = tmp(&body, "txt");
        let chunks = IngestPipeline::default().from_path(f.path()).unwrap();
        assert!(chunks.iter().all(|c| c.kind == DocumentKind::SpecDocument));
    }

    #[test]
    fn language_is_en() {
        let f = tmp(&long_md(), "md");
        let chunks = IngestPipeline::default().from_path(f.path()).unwrap();
        assert!(chunks.iter().all(|c| c.language == "en"));
    }

    #[test]
    fn chunk_indices_are_contiguous() {
        let body = "Sentence about the Earth Twin system. ".repeat(200);
        let f = tmp(&body, "md");
        let chunks = IngestPipeline::default().from_path(f.path()).unwrap();
        for (i, c) in chunks.iter().enumerate() {
            assert_eq!(c.chunk_index as usize, i);
        }
    }

    #[test]
    fn total_chunks_consistent() {
        let body = "Sentence about the Earth Twin system. ".repeat(200);
        let f = tmp(&body, "md");
        let chunks = IngestPipeline::default().from_path(f.path()).unwrap();
        let total = chunks.len() as u32;
        assert!(chunks.iter().all(|c| c.total_chunks == total));
    }

    #[test]
    fn provenance_sha256_is_valid() {
        let f = tmp(&long_md(), "md");
        let chunks = IngestPipeline::default().from_path(f.path()).unwrap();
        for c in &chunks {
            assert_eq!(c.provenance.sha256.len(), 64);
            assert!(c.provenance.sha256.chars().all(|ch| ch.is_ascii_hexdigit()));
        }
    }

    #[test]
    fn document_uri_starts_with_file() {
        let f = tmp(&long_md(), "md");
        let chunks = IngestPipeline::default().from_path(f.path()).unwrap();
        assert!(chunks
            .iter()
            .all(|c| c.document_uri.starts_with("file://")));
    }

    #[test]
    fn missing_file_returns_io_error() {
        let result = IngestPipeline::default()
            .from_path("/nonexistent/path/file.md");
        assert!(matches!(result, Err(PipelineError::Io { .. })));
    }

    #[test]
    fn access_tier_is_public() {
        let f = tmp(&long_md(), "md");
        let chunks = IngestPipeline::default().from_path(f.path()).unwrap();
        assert!(chunks.iter().all(|c| c.access_tier == AccessTier::Public));
    }

    // ── Embedding integration tests ────────────────────────────────────────

    #[test]
    fn no_embedder_leaves_embedding_none() {
        let f = tmp(&long_md(), "md");
        let chunks = IngestPipeline::default().from_path(f.path()).unwrap();
        assert!(chunks.iter().all(|c| c.embedding.is_none()));
    }

    #[test]
    fn embedder_populates_all_chunks() {
        let f = tmp(&long_md(), "md");
        let pipeline = IngestPipeline {
            embedder: Some(Box::new(PassthroughEmbedder)),
            ..Default::default()
        };
        let chunks = pipeline.from_path(f.path()).unwrap();
        assert!(!chunks.is_empty());
        assert!(chunks.iter().all(|c| c.embedding.is_some()));
    }

    #[test]
    fn embedder_vectors_have_correct_dim() {
        let f = tmp(&long_md(), "md");
        let pipeline = IngestPipeline {
            embedder: Some(Box::new(PassthroughEmbedder)),
            ..Default::default()
        };
        let chunks = pipeline.from_path(f.path()).unwrap();
        for c in &chunks {
            assert_eq!(c.embedding.as_ref().unwrap().dim(), 1);
        }
    }
}
