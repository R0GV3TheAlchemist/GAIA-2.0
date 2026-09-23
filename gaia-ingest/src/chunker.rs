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
use crate::lexicon::{classify_document_chunk, LexiconPlane};

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
/// - `lexicon_plane` and `lexicon_voice` are resolved by
///   [`classify_document_chunk`] before the chunk vec is returned — every
///   chunk exits `chunk()` with a real plane (`Order`, `Chaos`, or the
///   Bridge fallback for genuinely unknown provenance).
pub trait Chunker: Send + Sync {
    /// Split `text` into [`DocumentChunk`] records.
    ///
    /// `template` carries all metadata fields except `text`, `char_count`,
    /// `chunk_index`, `total_chunks`, and `id`. The chunker fills those five,
    /// then resolves `lexicon_plane` and `lexicon_voice` via
    /// [`classify_document_chunk`] before returning.
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
                stack[2] = None;
            } else if let Some(rest) = trimmed.strip_prefix("# ") {
                stack[0] = Some(rest.trim());
                stack[1] = None;
                stack[2] = None;
            }
        }
        stack
            .iter()
            .filter_map(|h| *h)
            .collect::<Vec<_>>()
            .join(" > ")
    }

    /// Returns `true` if `line` is the start of a fenced code block.
    fn is_fence(line: &str) -> bool {
        let t = line.trim_start();
        t.starts_with("```") || t.starts_with("~~~")
    }
}

impl Chunker for SlidingWindowChunker {
    fn chunk(
        &self,
        text: &str,
        template: DocumentChunk,
    ) -> Result<Vec<DocumentChunk>, ChunkError> {
        // ── Guards ──
        if text.trim().is_empty() {
            return Err(ChunkError::EmptySource);
        }
        self.validate()?;

        let min_chars   = 800_usize;
        let overlap_chars = ((self.target_chars as f32) * self.overlap_fraction) as usize;

        // ── Step 1: detect code-fence spans ──
        // We track byte offsets of code fence toggles so we never split inside
        // a fenced block. Stored as (start_byte, end_byte) pairs.
        let mut fence_spans: Vec<(usize, usize)> = Vec::new();
        {
            let mut in_fence = false;
            let mut fence_start = 0_usize;
            let mut byte_pos = 0_usize;
            for line in text.lines() {
                if Self::is_fence(line) {
                    if in_fence {
                        // closing fence — include the fence line itself
                        let end = byte_pos + line.len();
                        fence_spans.push((fence_start, end));
                        in_fence = false;
                    } else {
                        fence_start = byte_pos;
                        in_fence = true;
                    }
                }
                byte_pos += line.len() + 1; // +1 for the stripped newline
            }
            // unclosed fence — treat rest of document as fenced
            if in_fence {
                fence_spans.push((fence_start, text.len()));
            }
        }

        let in_fence = |byte: usize| -> bool {
            fence_spans.iter().any(|(s, e)| byte >= *s && byte < *e)
        };

        // ── Step 2: collect sentences (unicode word-boundary segmentation) ──
        // Each entry is (byte_offset_in_text, sentence_str)
        let sentences: Vec<(usize, &str)> = {
            let mut v = Vec::new();
            let mut cursor = 0_usize;
            for sentence in text.unicode_sentences() {
                // find actual byte position of this sentence in `text`
                let pos = text[cursor..]
                    .find(sentence)
                    .map(|rel| cursor + rel)
                    .unwrap_or(cursor);
                v.push((pos, sentence));
                cursor = pos + sentence.len();
            }
            v
        };

        if sentences.is_empty() {
            return Err(ChunkError::NoChunksProduced);
        }

        // ── Step 3: sliding window accumulation ──
        let mut raw_chunks: Vec<(String, usize)> = Vec::new(); // (text, byte_offset_of_first_sentence)
        let mut window: Vec<&str> = Vec::new();
        let mut window_chars: usize = 0;
        let mut window_offset: usize = sentences[0].0; // byte offset of first sentence in window

        let emit = |window: &Vec<&str>, offset: usize| -> (String, usize) {
            (window.concat(), offset)
        };

        for (sent_idx, &(byte_off, sentence)) in sentences.iter().enumerate() {
            // Never split inside a fenced block: if this sentence starts
            // inside a fence, always accumulate regardless of size.
            let locked = in_fence(byte_off);

            window.push(sentence);
            window_chars += sentence.chars().count();

            let at_target  = window_chars >= self.target_chars;
            let at_max     = window_chars >= 3_200;
            let last_sent  = sent_idx == sentences.len() - 1;

            // Emit when we hit the target (and we're not locked in a fence),
            // when we hit the hard max regardless, or on the last sentence.
            if (at_target && !locked) || at_max || last_sent {
                if window_chars >= min_chars || last_sent {
                    raw_chunks.push(emit(&window, window_offset));
                }

                // ── Overlap: retain last `overlap_chars` worth of sentences ──
                if !last_sent {
                    let mut carry_chars = 0_usize;
                    let mut carry_start = window.len();
                    for i in (0..window.len()).rev() {
                        carry_chars += window[i].chars().count();
                        if carry_chars >= overlap_chars {
                            carry_start = i;
                            break;
                        }
                    }
                    let carry: Vec<&str> = window[carry_start..].to_vec();
                    window_chars = carry.iter().map(|s| s.chars().count()).sum();

                    // Correctly compute the absolute sentence index of the
                    // first carried sentence.
                    //
                    // At emit time, `window` holds exactly `window.len()`
                    // sentences ending at `sent_idx` (inclusive), so the first
                    // sentence in the window has absolute index:
                    //   first_in_window = (sent_idx + 1) - window.len()
                    //
                    // The carry begins at `carry_start` within that window, so:
                    //   carry_absolute_idx = first_in_window + carry_start
                    //
                    // saturating_sub guards against the pathological case where
                    // window.len() > sent_idx + 1 (shouldn't happen in practice
                    // but avoids usize underflow / wrap in debug builds).
                    let first_in_window = (sent_idx + 1).saturating_sub(window.len());
                    let carry_absolute_idx = first_in_window + carry_start;

                    window_offset = sentences
                        .get(carry_absolute_idx)
                        .map(|(off, _)| *off)
                        .unwrap_or(byte_off);
                    window = carry;
                }
            }
        }

        if raw_chunks.is_empty() {
            return Err(ChunkError::NoChunksProduced);
        }

        // ── Step 4: stamp metadata and build DocumentChunks ──
        let total = raw_chunks.len() as u32;
        let mut chunks: Vec<DocumentChunk> = Vec::with_capacity(raw_chunks.len());

        for (idx, (chunk_text, byte_offset)) in raw_chunks.into_iter().enumerate() {
            // Resolve heading path using the END of the raw chunk text in the
            // source document as the probe point.
            //
            // Rationale: `unicode_sentences()` skips heading lines (no terminal
            // punctuation), so `byte_offset` — the start of the first *sentence*
            // — may be 0 for the very first chunk even when an H1 sits above it.
            // Probing at chunk-end guarantees we always scan past any ATX
            // heading that precedes the chunk's content, regardless of where
            // sentences start within that chunk.
            let probe_end = if self.inject_heading_prefix {
                // Find the byte position just after the last character of
                // chunk_text in the source document.  text.find() gives the
                // *first* occurrence; since chunk_text is derived from
                // sentences in document order this will be the correct window.
                text.find(chunk_text.as_str())
                    .map(|start| start + chunk_text.len())
                    .unwrap_or(byte_offset + chunk_text.len())
            } else {
                0
            };

            let heading = if self.inject_heading_prefix {
                Self::heading_path_at(text, probe_end)
            } else {
                String::new()
            };

            // Prepend heading path to chunk text when present
            let final_text = if heading.is_empty() {
                chunk_text
            } else {
                format!("{heading}\n\n{chunk_text}")
            };

            let char_count = final_text.chars().count();

            // Inherit all template fields; overwrite the five chunk-specific ones
            let mut attributes = template.attributes.clone();
            if !heading.is_empty() {
                attributes.insert("heading_path".to_string(), heading);
            }

            chunks.push(DocumentChunk {
                id:           Uuid::new_v4().to_string(),
                text:         final_text,
                char_count,
                chunk_index:  idx as u32,
                total_chunks: total,
                attributes,
                // ── lexicon fields: Bridge default; classify step below overwrites ──
                lexicon_plane: LexiconPlane::Bridge,
                lexicon_voice: None,
                // ── all other fields inherited verbatim from template ──
                document_title:  template.document_title.clone(),
                document_uri:    template.document_uri.clone(),
                kind:            template.kind,
                domain:          template.domain.clone(),
                language:        template.language.clone(),
                authored_at_unix: template.authored_at_unix,
                ttl_seconds:     template.ttl_seconds,
                confidence:      template.confidence,
                access_tier:     template.access_tier,
                access_control:  template.access_control.clone(),
                provenance:      template.provenance.clone(),
                artifact:        template.artifact.clone(),
            });
        }

        // ── Step 5: final validity pass (safety net) ──
        for chunk in &chunks {
            debug_assert!(
                chunk.is_valid(),
                "SlidingWindowChunker produced an invalid chunk at index {}: \
                 text_empty={} char_count_mismatch={} index_oob={} uri_empty={} domain_empty={}",
                chunk.chunk_index,
                chunk.text.is_empty(),
                chunk.char_count != chunk.text.chars().count(),
                chunk.chunk_index >= chunk.total_chunks,
                chunk.document_uri.is_empty(),
                chunk.domain.is_empty(),
            );
        }

        // ── Step 6: lexicon classification ──
        // classify_document_chunk is idempotent: chunks pre-classified by the
        // template (lexicon_plane != Bridge) are left untouched.
        for chunk in &mut chunks {
            classify_document_chunk(chunk);
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
        lexicon::{LexiconPlane, LexiconVoice},
        provenance::ProvenanceReceipt,
        schema::DataSource,
    };
    use std::collections::BTreeMap;

    /// Build a valid template chunk (text / char_count / chunk_index /
    /// total_chunks / id are intentionally placeholder — the chunker overwrites them).
    fn template() -> DocumentChunk {
        DocumentChunk {
            id:              "00000000-0000-0000-0000-000000000000".into(),
            document_title:  "Test Document".into(),
            document_uri:    "gaia://test/doc".into(),
            kind:            DocumentKind::CanonTablet,
            text:            "placeholder".into(),
            char_count:      11,
            chunk_index:     0,
            total_chunks:    1,
            domain:          "test".into(),
            language:        "en".into(),
            authored_at_unix: Some(1_700_000_000),
            ttl_seconds:     None,
            confidence:      ConfidenceTier::Canon,
            access_tier:     AccessTier::Public,
            access_control:  vec![],
            provenance: ProvenanceReceipt {
                source:           DataSource::CanonTablet,
                source_url:       "gaia://test/doc".into(),
                external_id:      "test-doc-v1".into(),
                fetched_at_unix:  1_700_000_001,
                observed_at_unix: 1_700_000_000,
                sha256:           "a".repeat(64),
                license:          "CC-BY-4.0".into(),
            },
            artifact:        None,
            attributes:      BTreeMap::new(),
            // lexicon defaults — classify step overwrites after ingestion
            lexicon_plane:   LexiconPlane::Bridge,
            lexicon_voice:   None,
        }
    }

    /// Generate a plain-prose document long enough to produce multiple chunks.
    fn long_prose(sentences: usize) -> String {
        // ~120 chars per sentence ≈ 30 tokens; 20 sentences ≈ 2 400 chars ≈ 600 tokens
        let sentence = "GAIA ingests Earth-observation data from satellites, \
                        IoT sensors, and biodiversity networks to build a \
                        living, auditable model of the planet. ";
        sentence.repeat(sentences)
    }

    // ── Happy path ──

    #[test]
    fn chunk_plain_prose_all_valid() {
        let chunker = SlidingWindowChunker::default();
        let text = long_prose(30); // ~3 600 chars → expect 2+ chunks
        let chunks = chunker.chunk(&text, template()).unwrap();
        assert!(!chunks.is_empty(), "expected at least one chunk");
        for c in &chunks {
            assert!(c.is_valid(), "chunk {} failed is_valid()", c.chunk_index);
        }
    }

    #[test]
    fn chunk_index_contiguous_and_total_consistent() {
        let chunker = SlidingWindowChunker::default();
        let text = long_prose(30);
        let chunks = chunker.chunk(&text, template()).unwrap();
        let total = chunks[0].total_chunks;
        assert_eq!(total as usize, chunks.len());
        for (i, c) in chunks.iter().enumerate() {
            assert_eq!(c.chunk_index, i as u32);
            assert_eq!(c.total_chunks, total);
        }
    }

    #[test]
    fn chunk_no_chunk_below_minimum_chars() {
        let chunker = SlidingWindowChunker::default();
        // Even the last (potentially short) chunk must be ≥ 800 chars or merged
        // For a document that produces a single chunk, it may be < 800 chars
        // only if it's the sole chunk (the whole document is short).
        // Here we test a long document: every chunk except possibly the last
        // must be >= min.
        let text = long_prose(40);
        let chunks = chunker.chunk(&text, template()).unwrap();
        // All chunks except the last must meet minimum
        for c in chunks.iter().take(chunks.len().saturating_sub(1)) {
            assert!(
                c.char_count >= 800,
                "non-final chunk {} has char_count {} < 800",
                c.chunk_index,
                c.char_count
            );
        }
    }

    #[test]
    fn char_count_matches_text_chars_count() {
        let chunker = SlidingWindowChunker::default();
        let text = long_prose(30);
        let chunks = chunker.chunk(&text, template()).unwrap();
        for c in &chunks {
            assert_eq!(
                c.char_count,
                c.text.chars().count(),
                "char_count mismatch on chunk {}",
                c.chunk_index
            );
        }
    }

    // ── Lexicon classify step ──

    #[test]
    fn chunk_classify_step_runs_after_chunk() {
        // CanonTablet kind + CanonTablet source → Order / Institutional.
        // Verifies classify_document_chunk() fired inside chunk().
        let chunker = SlidingWindowChunker::default();
        let text = long_prose(20);
        let chunks = chunker.chunk(&text, template()).unwrap();
        for c in &chunks {
            assert_eq!(
                c.lexicon_plane,
                LexiconPlane::Order,
                "chunk {} should be Order after classify step, got {:?}",
                c.chunk_index,
                c.lexicon_plane
            );
            assert_eq!(
                c.lexicon_voice,
                Some(LexiconVoice::Institutional),
                "chunk {} should have Institutional voice",
                c.chunk_index
            );
        }
    }

    #[test]
    fn chunk_classify_respects_preexisting_plane() {
        // If template already has a non-Bridge plane, classify step must not
        // overwrite it (idempotency contract).
        let mut tmpl = template();
        tmpl.lexicon_plane = LexiconPlane::Chaos;
        tmpl.lexicon_voice = Some(LexiconVoice::HumanVoice);
        let chunker = SlidingWindowChunker::default();
        let text = long_prose(20);
        let chunks = chunker.chunk(&text, tmpl).unwrap();
        for c in &chunks {
            assert_eq!(
                c.lexicon_plane,
                LexiconPlane::Chaos,
                "pre-classified Chaos plane must survive chunk() + classify"
            );
            assert_eq!(c.lexicon_voice, Some(LexiconVoice::HumanVoice));
        }
    }

    // ── Heading prefix ──

    #[test]
    fn chunk_markdown_heading_prefix_injected() {
        let chunker = SlidingWindowChunker::default();
        // Build a Markdown document with a clear H1 and enough body text
        let sentence = "This section describes the ingestion pipeline in detail. ";
        let body = sentence.repeat(25);
        let text = format!("# Ingestion Design\n\n{body}");
        let chunks = chunker.chunk(&text, template()).unwrap();
        // At least one chunk should carry the heading
        let with_heading = chunks
            .iter()
            .filter(|c| c.attributes.contains_key("heading_path"))
            .count();
        assert!(with_heading > 0, "no chunks carried a heading_path attribute");
        // The heading text should appear somewhere in those chunks
        let heading_in_text = chunks
            .iter()
            .any(|c| c.text.contains("Ingestion Design"));
        assert!(heading_in_text, "heading text not found in any chunk text");
    }

    #[test]
    fn chunk_no_heading_prefix_when_disabled() {
        let chunker = SlidingWindowChunker {
            inject_heading_prefix: false,
            ..Default::default()
        };
        let sentence = "Plain prose without heading injection for this test case. ";
        let body = sentence.repeat(25);
        let text = format!("# Should Not Appear\n\n{body}");
        let chunks = chunker.chunk(&text, template()).unwrap();
        for c in &chunks {
            assert!(
                !c.attributes.contains_key("heading_path"),
                "heading_path should be absent when inject_heading_prefix=false"
            );
        }
    }

    // ── Error cases ──

    #[test]
    fn chunk_rejects_empty_source() {
        let chunker = SlidingWindowChunker::default();
        assert!(matches!(
            chunker.chunk("", template()),
            Err(ChunkError::EmptySource)
        ));
        assert!(matches!(
            chunker.chunk("   \n  \t  ", template()),
            Err(ChunkError::EmptySource)
        ));
    }

    #[test]
    fn chunk_rejects_invalid_config() {
        let bad_target = SlidingWindowChunker {
            target_chars: 100, // below 800 minimum
            ..Default::default()
        };
        assert!(matches!(
            bad_target.chunk("some text", template()),
            Err(ChunkError::InvalidTemplate(_))
        ));

        let bad_overlap = SlidingWindowChunker {
            overlap_fraction: 0.50, // above 0.20 maximum
            ..Default::default()
        };
        assert!(matches!(
            bad_overlap.chunk("some text", template()),
            Err(ChunkError::InvalidTemplate(_))
        ));
    }

    // ── Code block preservation ──

    #[test]
    fn code_block_not_split_mid_fence() {
        let chunker = SlidingWindowChunker::default();
        // Build a document where a fenced code block sits inside a larger body.
        // The code block itself is short; what we verify is that the fence
        // markers end up in the same chunk (the chunker does not split inside
        // a fenced region).
        let preamble = "This document describes how provenance is sealed. \
                        Every ingested record carries a cryptographic receipt. \
                        The receipt is computed over the raw source bytes. \
                        This ensures full auditability across the pipeline. ";
        let code = "```rust\nfn seal(data: &[u8]) -> String {\n    hex::encode(Sha256::digest(data))\n}\n```";
        let postamble = "After sealing, the receipt is stored alongside the record. \
                         Any downstream consumer can verify the hash independently. ";
        // Repeat preamble to push past target so the chunker must emit before code
        let text = format!("{}{}{}", preamble.repeat(10), code, postamble.repeat(10));
        let chunks = chunker.chunk(&text, template()).unwrap();
        // Verify no chunk contains only an opening ``` without a closing ```
        for c in &chunks {
            let opens  = c.text.matches("```").count();
            // Either 0 fence markers, or an even number (open+close pairs)
            // A lone opening fence would give 1 — that is the bug we're guarding.
            assert!(
                opens % 2 == 0,
                "chunk {} contains an odd number of fence markers ({}): possible mid-block split",
                c.chunk_index,
                opens
            );
        }
    }
}
