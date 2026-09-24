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

// ── Error type ───────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum ChunkError {
    #[error("source text is empty")]
    EmptySource,
    #[error("template chunk is invalid: {0}")]
    InvalidTemplate(String),
    #[error("chunking produced zero valid chunks from non-empty source")]
    NoChunksProduced,
}

// ── Chunker trait ────────────────────────────────────────────────────────────

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

// ── SlidingWindowChunker ──────────────────────────────────────────────────────

/// Reference implementation of [`Chunker`] — named in
/// `gaia-spec/rag/chunking-standard.md` § 8.
///
/// Uses `unicode-segmentation` sentence boundaries, a sliding overlap
/// window, and optional Markdown heading-prefix injection.
///
/// ## Parameters
///
/// | Field | Default | Valid range |
/// |---|---|---|
/// | `target_chars` | 2 000 | 800 – 3 200 |
/// | `overlap_fraction` | 0.15 | 0.10 – 0.20 |
/// | `inject_heading_prefix` | `true` | — |
#[derive(Debug, Clone)]
pub struct SlidingWindowChunker {
    /// Target character count per chunk (~500 tokens at 4 chars/token).
    pub target_chars: usize,
    /// Overlap as a fraction of `target_chars`.
    pub overlap_fraction: f32,
    /// Prepend the current Markdown heading path to each chunk's `text`
    /// and store it in `attributes["heading_path"]`.
    pub inject_heading_prefix: bool,
}

impl Default for SlidingWindowChunker {
    fn default() -> Self {
        Self {
            target_chars: 2_000,
            overlap_fraction: 0.15,
            inject_heading_prefix: true,
        }
    }
}

impl SlidingWindowChunker {
    /// Validate configuration parameters against the chunking standard.
    fn validate(&self) -> Result<(), ChunkError> {
        if self.target_chars < 800 || self.target_chars > 3_200 {
            return Err(ChunkError::InvalidTemplate(format!(
                "target_chars {} is outside valid range [800, 3200]",
                self.target_chars
            )));
        }
        if self.overlap_fraction < 0.10 || self.overlap_fraction > 0.20 {
            return Err(ChunkError::InvalidTemplate(format!(
                "overlap_fraction {:.2} is outside valid range [0.10, 0.20]",
                self.overlap_fraction
            )));
        }
        Ok(())
    }

    /// Extract the heading path at `probe_end` bytes into `text`.
    ///
    /// Scans all ATX headings (`# `, `## `, `### `) that appear *before*
    /// `probe_end` and returns the last heading of each level joined with ` > `.
    ///
    /// ## Why probe_end, not chunk start?
    ///
    /// `unicode_sentences()` does not yield heading lines (they carry no
    /// terminal punctuation), so the byte offset of the *first sentence* in a
    /// chunk can be greater than the heading that labels it — or, for the very
    /// first chunk, exactly 0 (before the heading).  Passing the end of the
    /// raw chunk text as `probe_end` guarantees that any ATX heading preceding
    /// the chunk content is always visible to the scanner regardless of where
    /// sentences begin.
    fn heading_path_at(text: &str, probe_end: usize) -> String {
        // heading_stack[0] = last H1, [1] = last H2, [2] = last H3
        let mut stack: [Option<&str>; 3] = [None; 3];
        let prefix = &text[..probe_end.min(text.len())];
        for line in prefix.lines() {
            let trimmed = line.trim_start();
            if let Some(rest) = trimmed.strip_prefix("### ") {
                stack[2] = Some(rest.trim());
            } else if let Some(rest) = trimmed.strip_prefix("## ") {
                stack[1] = Some(rest.trim());
                stack[2] = None; // reset deeper level
            } else if let Some(rest) = trimmed.strip_prefix("# ") {
                stack[0] = Some(rest.trim());
                stack[1] = None;
                stack[2] = None;
            }
        }
        stack
            .iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>()
            .join(" > ")
    }
}

impl Chunker for SlidingWindowChunker {
    fn chunk(
        &self,
        text: &str,
        template: DocumentChunk,
    ) -> Result<Vec<DocumentChunk>, ChunkError> {
        if text.trim().is_empty() {
            return Err(ChunkError::EmptySource);
        }
        self.validate()?;

        let target = self.target_chars;
        let overlap = (target as f32 * self.overlap_fraction) as usize;
        let min_chars = target / 4; // ~500 chars absolute floor

        // ── 1. Collect all Unicode sentences ─────────────────────────────────
        let sentences: Vec<&str> = text.unicode_sentences().collect();

        // ── 2. Greedily pack sentences into windows ───────────────────────────
        let mut raw_chunks: Vec<String> = Vec::new();
        let mut raw_offsets: Vec<usize> = Vec::new(); // byte offset in `text`
        let mut i = 0usize; // sentence index

        // Track the byte position of each sentence in `text` so we can pass
        // the correct probe_end to heading_path_at().
        let mut byte_pos = 0usize;
        let mut sent_byte_starts: Vec<usize> = Vec::with_capacity(sentences.len());
        for s in &sentences {
            // Locate the sentence in the remaining tail of text.
            if let Some(off) = text[byte_pos..].find(s) {
                sent_byte_starts.push(byte_pos + off);
                byte_pos = byte_pos + off + s.len();
            } else {
                sent_byte_starts.push(byte_pos);
            }
        }

        while i < sentences.len() {
            let window_start_byte = sent_byte_starts[i];
            let mut buf = String::new();
            let mut j = i;

            // Fill until we exceed target or exhaust sentences.
            while j < sentences.len() && buf.len() < target {
                buf.push_str(sentences[j]);
                j += 1;
            }

            // If this window is too short and there is a preceding chunk,
            // merge it into that chunk instead of emitting a stub.
            if buf.chars().count() < min_chars && !raw_chunks.is_empty() {
                let last = raw_chunks.last_mut().unwrap();
                last.push_str(&buf);
                break;
            }

            raw_chunks.push(buf);
            raw_offsets.push(window_start_byte);

            // Advance by (j - i) sentences, then step back by overlap.
            let consumed = j - i;
            let step = consumed.saturating_sub(overlap / (target / consumed.max(1)).max(1));
            i += step.max(1);
        }

        let total = raw_chunks.len() as u32;
        if total == 0 {
            return Err(ChunkError::NoChunksProduced);
        }

        // ── 3. Build DocumentChunk records ────────────────────────────────────
        let mut chunks: Vec<DocumentChunk> = Vec::with_capacity(total as usize);
        for (idx, (raw_text, probe_end)) in raw_chunks.into_iter().zip(raw_offsets).enumerate() {
            let char_count = raw_text.chars().count();

            // Optionally prepend the heading path to the text.
            let (final_text, heading_path) = if self.inject_heading_prefix {
                let path = Self::heading_path_at(text, probe_end + raw_text.len());
                if path.is_empty() {
                    (raw_text, String::new())
                } else {
                    (format!("[{path}]\n{raw_text}"), path)
                }
            } else {
                (raw_text, String::new())
            };

            let mut attributes = template.attributes.clone();
            if !heading_path.is_empty() {
                attributes.insert("heading_path".into(), heading_path);
            }

            chunks.push(DocumentChunk {
                id:           Uuid::new_v4().to_string(),
                text:         final_text,
                char_count,
                chunk_index:  idx as u32,
                total_chunks: total,
                document_title: template.document_title.clone(),
                document_uri:   template.document_uri.clone(),
                kind:           template.kind,
                domain:         template.domain.clone(),
                language:       template.language.clone(),
                authored_at_unix: template.authored_at_unix,
                ttl_seconds:    template.ttl_seconds,
                confidence:     template.confidence,
                access_tier:    template.access_tier,
                access_control: template.access_control.clone(),
                provenance:     template.provenance.clone(),
                artifact:       template.artifact.clone(),
                attributes,
                lexicon_plane:  template.lexicon_plane,
                lexicon_voice:  template.lexicon_voice.clone(),
                embedding:      None,
            });
        }

        // ── 4. Classify lexicon plane / voice ────────────────────────────────
        for chunk in &mut chunks {
            classify_document_chunk(chunk);
        }

        // ── 5. Validate every chunk ──────────────────────────────────────────
        for (i, chunk) in chunks.iter().enumerate() {
            if !chunk.is_valid() {
                return Err(ChunkError::InvalidTemplate(format!(
                    "chunk {i} failed is_valid() after construction"
                )));
            }
        }

        Ok(chunks)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        document::{
            AccessTier, ConfidenceTier, DocumentChunk, DocumentKind,
        },
        lexicon::LexiconPlane,
        provenance::ProvenanceReceipt,
        schema::DataSource,
    };
    use std::collections::BTreeMap;

    fn template() -> DocumentChunk {
        DocumentChunk {
            id:              String::new(),
            text:            String::new(),
            char_count:      0,
            chunk_index:     0,
            total_chunks:    0,
            document_title:  "Test Doc".into(),
            document_uri:    "gaia://test/doc".into(),
            kind:            DocumentKind::CanonTablet,
            domain:          "test".into(),
            language:        "en".into(),
            authored_at_unix: None,
            ttl_seconds:     None,
            confidence:      ConfidenceTier::Canon,
            access_tier:     AccessTier::Public,
            access_control:  vec![],
            provenance: ProvenanceReceipt {
                source:              DataSource::CanonTablet,
                source_url:          "gaia://test/doc".into(),
                external_id:         "test-doc-v1".into(),
                fetched_at_unix:     1_700_000_001,
                observed_at_unix:    1_700_000_000,
                sha256:              "a".repeat(64),
                license:             "CC-BY-4.0".into(),
            },
            artifact:        None,
            attributes:      BTreeMap::new(),
            lexicon_plane:   LexiconPlane::Bridge,
            lexicon_voice:   None,
            embedding:       None,
        }
    }

    fn long_text() -> String {
        "This is a test sentence about GAIA and the Earth Twin system. ".repeat(200)
    }

    // ── basic contract ────────────────────────────────────────────────────────

    #[test]
    fn produces_chunks_from_long_text() {
        let chunks = SlidingWindowChunker::default()
            .chunk(&long_text(), template())
            .unwrap();
        assert!(!chunks.is_empty());
    }

    #[test]
    fn all_chunks_pass_is_valid() {
        let chunks = SlidingWindowChunker::default()
            .chunk(&long_text(), template())
            .unwrap();
        for c in &chunks {
            assert!(c.is_valid(), "chunk {} failed is_valid()", c.chunk_index);
        }
    }

    #[test]
    fn chunk_indices_contiguous() {
        let chunks = SlidingWindowChunker::default()
            .chunk(&long_text(), template())
            .unwrap();
        let total = chunks.len() as u32;
        for (i, c) in chunks.iter().enumerate() {
            assert_eq!(c.chunk_index, i as u32);
            assert_eq!(c.total_chunks, total);
        }
    }

    #[test]
    fn char_count_matches_text() {
        let chunks = SlidingWindowChunker::default()
            .chunk(&long_text(), template())
            .unwrap();
        for c in &chunks {
            assert_eq!(c.char_count, c.text.chars().count());
        }
    }

    #[test]
    fn embedding_is_none_after_chunking() {
        let chunks = SlidingWindowChunker::default()
            .chunk(&long_text(), template())
            .unwrap();
        assert!(chunks.iter().all(|c| c.embedding.is_none()));
    }

    // ── error cases ───────────────────────────────────────────────────────────

    #[test]
    fn empty_text_returns_empty_source_error() {
        let result = SlidingWindowChunker::default().chunk("", template());
        assert!(matches!(result, Err(ChunkError::EmptySource)));
    }

    #[test]
    fn whitespace_only_returns_empty_source_error() {
        let result = SlidingWindowChunker::default().chunk("   \n\t  ", template());
        assert!(matches!(result, Err(ChunkError::EmptySource)));
    }

    #[test]
    fn invalid_target_chars_returns_error() {
        let chunker = SlidingWindowChunker {
            target_chars: 100, // below 800 minimum
            ..Default::default()
        };
        let result = chunker.chunk(&long_text(), template());
        assert!(matches!(result, Err(ChunkError::InvalidTemplate(_))));
    }

    #[test]
    fn invalid_overlap_fraction_returns_error() {
        let chunker = SlidingWindowChunker {
            overlap_fraction: 0.50, // above 0.20 maximum
            ..Default::default()
        };
        let result = chunker.chunk(&long_text(), template());
        assert!(matches!(result, Err(ChunkError::InvalidTemplate(_))));
    }

    // ── heading injection ─────────────────────────────────────────────────────

    #[test]
    fn heading_path_injected_when_enabled() {
        let md = format!("# GAIA Overview\n\n{}", long_text());
        let chunks = SlidingWindowChunker {
            inject_heading_prefix: true,
            ..Default::default()
        }
        .chunk(&md, template())
        .unwrap();
        assert!(
            chunks.iter().any(|c| c.attributes.contains_key("heading_path")),
            "at least one chunk should carry a heading_path attribute"
        );
    }

    #[test]
    fn heading_path_not_injected_when_disabled() {
        let md = format!("# GAIA Overview\n\n{}", long_text());
        let chunks = SlidingWindowChunker {
            inject_heading_prefix: false,
            ..Default::default()
        }
        .chunk(&md, template())
        .unwrap();
        assert!(
            chunks.iter().all(|c| !c.attributes.contains_key("heading_path")),
            "no chunk should carry a heading_path when injection is disabled"
        );
    }

    // ── metadata inheritance ──────────────────────────────────────────────────

    #[test]
    fn metadata_inherited_from_template() {
        let mut tmpl = template();
        tmpl.document_title = "Custom Title".into();
        tmpl.domain = "rag".into();
        tmpl.language = "fr".into();
        let chunks = SlidingWindowChunker::default()
            .chunk(&long_text(), tmpl)
            .unwrap();
        assert!(chunks.iter().all(|c| c.document_title == "Custom Title"));
        assert!(chunks.iter().all(|c| c.domain == "rag"));
        assert!(chunks.iter().all(|c| c.language == "fr"));
    }
}
