//! `SemanticChunker` — FM-1 text chunking for RAG retrieval.
//!
//! Splits a source document into [`DocumentChunk`] records conforming
//! to `gaia-spec/rag/chunking-standard.md`:
//!
//! - 400–600 token target  (~2 000 chars at 4 chars/token)
//! - 200 token hard minimum (~800 chars)
//! - 800 token hard maximum (~3 200 chars)
//! - 10–20 % sliding overlap
//! - Sentence-boundary split via `unicode-segmentation`
//! - Heading-prefix injection for Markdown documents
//!
//! The [`Chunker`] trait is the public interface. [`SlidingWindowChunker`]
//! is the reference implementation named in the chunking standard.

use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::document::DocumentChunk;
use crate::lexicon::classify_document_chunk;

// ── Error type ───────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum ChunkError {
    #[error("source text is empty")]
    EmptySource,
    #[error("template chunk is invalid: {0}")]
    InvalidTemplate(String),
    #[error("chunking produced zero valid chunks from non-empty source")]
    NoChunksProduced,
}

// ── Chunker trait ────────────────────────────────

/// A strategy for splitting a source document into [`DocumentChunk`] records.
///
/// Implementations **must** conform to `gaia-spec/rag/chunking-standard.md`.
///
/// ## Contract
/// - Every returned chunk passes [`DocumentChunk::is_valid()`].
/// - `chunk_index` is zero-based and contiguous (`0..total_chunks`).
/// - `total_chunks` is identical across all returned chunks.
/// - Each chunk receives a freshly generated UUID v4 `id`.
/// - `text` and `char_count` are set by the chunker; all other fields are
///   inherited from the `template` parameter.
/// - `lexicon_plane` and `lexicon_voice` are inherited from `template` and
///   then resolved by [`classify_document_chunk`] before the chunk vec is
///   returned. If the template already carries a non-Bridge plane (explicit
///   pre-classification), `classify_document_chunk` is a no-op for that chunk.
pub trait Chunker: Send + Sync {
    /// Split `text` into [`DocumentChunk`] records.
    ///
    /// `template` carries all metadata fields except `text`, `char_count`,
    /// `chunk_index`, `total_chunks`, and `id`. The chunker fills those five,
    /// inherits `lexicon_plane` and `lexicon_voice` from the template, then
    /// resolves Bridge chunks via [`classify_document_chunk`] before returning.
    fn chunk(
        &self,
        text: &str,
        template: DocumentChunk,
    ) -> Result<Vec<DocumentChunk>, ChunkError>;
}
