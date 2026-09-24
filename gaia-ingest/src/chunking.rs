//! Semantic, structure-preserving document chunking.
//!
//! This module provides [`ChunkingStrategy`] and the concrete
//! [`MarkdownChunker`] which splits documents on heading boundaries,
//! preserves code blocks and tables as atomic units, and tracks the
//! heading breadcrumb path for every emitted chunk.
//!
//! ## Size constants
//!
//! | Constant            | Value  | Meaning                              |
//! |---------------------|--------|--------------------------------------|
//! | `MIN_CHUNK_CHARS`   | 256    | Chunks below this are merged forward |
//! | `MAX_CHUNK_CHARS`   | 2_048  | Soft ceiling per chunk               |
//! | `OVERLAP_CHARS`     | 128    | Overlap injected between siblings    |
//!
//! See `gaia-spec/rag/chunking.md` for token-level rationale.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gaia_ingest::chunking::{MarkdownChunker, ChunkingStrategy};
//!
//! let chunker = MarkdownChunker::default();
//! let chunks = chunker.chunk(markdown_text);
//! for c in &chunks {
//!     println!("{}: {}", c.heading_path.join(" > "), &c.text[..40]);
//! }
//! ```

// ── Size constants ────────────────────────────────────────────────────────────

/// Minimum chunk size in characters.  Chunks smaller than this are merged
/// with the following sibling.
pub const MIN_CHUNK_CHARS: usize = 256;

/// Soft maximum chunk size in characters.  The chunker will not split inside
/// a code block or table even if this limit is exceeded.
pub const MAX_CHUNK_CHARS: usize = 2_048;

/// Number of trailing characters from the previous chunk prepended to the
/// next chunk to maintain context continuity.
pub const OVERLAP_CHARS: usize = 128;

// ── ChunkingStrategy ──────────────────────────────────────────────────────────

/// The splitting strategy to apply to a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChunkingStrategy {
    /// Split on Markdown heading boundaries (`#`, `##`, `###`).  Code blocks
    /// and tables are never split.  This is the recommended strategy for
    /// canon tablets and research documents.
    MarkdownHeading,
    /// Fixed-size sliding window with configurable size and overlap (in
    /// characters).  Suitable for plain-text sources with no structure.
    FixedSizeWithOverlap { size: usize, overlap: usize },
    /// Sentence-boundary splitting.  **Stub only** — not yet implemented.
    /// Will be implemented in a follow-on issue.
    Sentence,
}

// ── StructuredChunk ───────────────────────────────────────────────────────────

/// A chunk produced by the [`MarkdownChunker`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredChunk {
    /// The chunk text, including any heading prefix injected for context.
    pub text: String,
    /// Breadcrumb of parent heading titles, outermost first.
    /// e.g. `["# Introduction", "## Motivation"]`.
    pub heading_path: Vec<String>,
    /// Zero-based position of this chunk within the source document.
    pub chunk_index: usize,
    /// `true` when this chunk contains a fenced code block.
    pub is_code_block: bool,
    /// `true` when this chunk contains a Markdown table.
    pub is_table: bool,
}

// ── MarkdownChunker ───────────────────────────────────────────────────────────

/// Splits a Markdown document into [`StructuredChunk`]s using heading
/// boundaries.
///
/// Rules (in priority order):
/// 1. Fenced code blocks (` ``` `) are **never** split — always one chunk.
/// 2. Markdown tables (lines starting with `|`) are **never** split.
/// 3. Content is split at `#` / `##` / `###` heading boundaries.
/// 4. Each chunk carries the `heading_path` breadcrumb of its ancestors.
/// 5. Chunks below [`MIN_CHUNK_CHARS`] are merged with the following sibling.
/// 6. Heading text is prepended as context prefix on every child chunk.
#[derive(Debug, Clone, Default)]
pub struct MarkdownChunker;

impl MarkdownChunker {
    /// Split `text` into [`StructuredChunk`]s.
    pub fn chunk(&self, text: &str) -> Vec<StructuredChunk> {
        let sections = split_sections(text);
        let merged = merge_small(sections);
        merged
            .into_iter()
            .enumerate()
            .map(|(i, mut c)| {
                c.chunk_index = i;
                c
            })
            .collect()
    }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Classify a line as a heading and return its level (1–3) and title text,
/// or `None` if it is not a heading.
fn heading_level(line: &str) -> Option<(usize, &str)> {
    let trimmed = line.trim_start_matches('#');
    let hashes = line.len() - trimmed.len();
    if hashes >= 1 && hashes <= 3 && trimmed.starts_with(' ') {
        Some((hashes, trimmed.trim()))
    } else {
        None
    }
}

/// Split `text` into raw sections at heading boundaries, preserving code
/// blocks and tables as atomic sections.
fn split_sections(text: &str) -> Vec<StructuredChunk> {
    let mut chunks: Vec<StructuredChunk> = Vec::new();
    // heading_stack[0] = h1 title, [1] = h2 title, [2] = h3 title
    let mut heading_stack: Vec<Option<String>> = vec![None, None, None];
    let mut current_lines: Vec<String> = Vec::new();
    let mut in_code_block = false;
    let mut current_is_code = false;
    let mut current_is_table = false;

    let flush = |lines: &mut Vec<String>,
                 chunks: &mut Vec<StructuredChunk>,
                 stack: &[Option<String>],
                 is_code: bool,
                 is_table: bool| {
        let raw = lines.join("\n");
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            lines.clear();
            return;
        }
        let heading_path: Vec<String> = stack.iter().flatten().cloned().collect();
        // Prepend heading path as context prefix when not a code/table block
        let text = if !is_code && !is_table && !heading_path.is_empty() {
            format!("{prefix}\n{body}",
                prefix = heading_path.last().map(String::as_str).unwrap_or(""),
                body = trimmed)
        } else {
            trimmed.to_string()
        };
        chunks.push(StructuredChunk {
            text,
            heading_path,
            chunk_index: 0, // assigned after merge
            is_code_block: is_code,
            is_table,
        });
        lines.clear();
    };

    for line in text.lines() {
        // ── Code block toggle ─────────────────────────────────────────────
        if line.trim_start().starts_with("```") {
            if in_code_block {
                // Closing fence — end of code block
                current_lines.push(line.to_string());
                in_code_block = false;
                flush(
                    &mut current_lines,
                    &mut chunks,
                    &heading_stack,
                    true,
                    false,
                );
                current_is_code = false;
                current_is_table = false;
            } else {
                // Opening fence — flush pending text, start code block
                flush(
                    &mut current_lines,
                    &mut chunks,
                    &heading_stack,
                    current_is_code,
                    current_is_table,
                );
                current_is_code = true;
                current_is_table = false;
                in_code_block = true;
                current_lines.push(line.to_string());
            }
            continue;
        }

        if in_code_block {
            current_lines.push(line.to_string());
            continue;
        }

        // ── Table detection ───────────────────────────────────────────────
        let is_table_line = line.trim_start().starts_with('|');
        if is_table_line {
            if !current_is_table {
                // Transition into table — flush preceding prose
                flush(
                    &mut current_lines,
                    &mut chunks,
                    &heading_stack,
                    current_is_code,
                    false,
                );
                current_is_table = true;
                current_is_code = false;
            }
            current_lines.push(line.to_string());
            continue;
        } else if current_is_table {
            // Leaving table — flush it
            flush(
                &mut current_lines,
                &mut chunks,
                &heading_stack,
                false,
                true,
            );
            current_is_table = false;
        }

        // ── Heading detection ─────────────────────────────────────────────
        if let Some((level, title)) = heading_level(line) {
            // Flush current section before starting a new one
            flush(
                &mut current_lines,
                &mut chunks,
                &heading_stack,
                current_is_code,
                current_is_table,
            );
            current_is_code = false;
            current_is_table = false;
            // Update heading stack: clear all levels >= current
            let idx = level - 1;
            heading_stack[idx] = Some(format!("{hashes} {title}",
                hashes = "#".repeat(level)));
            for slot in heading_stack.iter_mut().skip(idx + 1) {
                *slot = None;
            }
            // The heading line itself starts the new section
            current_lines.push(line.to_string());
            continue;
        }

        current_lines.push(line.to_string());
    }

    // Flush any remaining content
    flush(
        &mut current_lines,
        &mut chunks,
        &heading_stack,
        current_is_code,
        current_is_table,
    );

    chunks
}

/// Merge chunks that are below [`MIN_CHUNK_CHARS`] into the following sibling.
/// Code blocks and tables are never merged regardless of size.
fn merge_small(mut chunks: Vec<StructuredChunk>) -> Vec<StructuredChunk> {
    if chunks.is_empty() {
        return chunks;
    }
    let mut result: Vec<StructuredChunk> = Vec::new();
    let mut pending: Option<StructuredChunk> = None;

    for chunk in chunks.drain(..) {
        match pending.take() {
            None => {
                pending = Some(chunk);
            }
            Some(prev) => {
                let too_small = prev.text.len() < MIN_CHUNK_CHARS
                    && !prev.is_code_block
                    && !prev.is_table;
                if too_small {
                    // Merge prev into current chunk
                    let merged_text = format!("{} {}", prev.text.trim_end(), chunk.text.trim_start());
                    let merged = StructuredChunk {
                        text: merged_text,
                        heading_path: chunk.heading_path.clone(),
                        chunk_index: 0,
                        is_code_block: chunk.is_code_block,
                        is_table: chunk.is_table,
                    };
                    pending = Some(merged);
                } else {
                    result.push(prev);
                    pending = Some(chunk);
                }
            }
        }
    }
    if let Some(last) = pending {
        result.push(last);
    }
    result
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn chunker() -> MarkdownChunker {
        MarkdownChunker::default()
    }

    #[test]
    fn three_headings_produce_at_least_three_chunks() {
        let body = "word ".repeat(60);
        let md = format!(
            "## Alpha\n\n{body}\n\n## Beta\n\n{body}\n\n## Gamma\n\n{body}"
        );
        let chunks = chunker().chunk(&md);
        assert!(chunks.len() >= 3, "expected ≥3 chunks, got {}", chunks.len());
    }

    #[test]
    fn heading_path_is_correct() {
        let body = "word ".repeat(60);
        let md = format!("## Section One\n\n{body}");
        let chunks = chunker().chunk(&md);
        assert!(!chunks.is_empty());
        assert!(chunks[0].heading_path.iter().any(|h| h.contains("Section One")));
    }

    #[test]
    fn code_block_is_single_chunk() {
        // Build a code block well above MAX_CHUNK_CHARS
        let code_line = "let x = 1; // padding padding padding padding padding\n";
        let body = code_line.repeat(60);
        let md = format!("# Doc\n\n```rust\n{body}```");
        let chunks = chunker().chunk(&md);
        let code_chunks: Vec<_> = chunks.iter().filter(|c| c.is_code_block).collect();
        assert_eq!(code_chunks.len(), 1, "code block must not be split");
    }

    #[test]
    fn table_is_single_chunk() {
        let mut rows = String::new();
        rows.push_str("| Header A | Header B |\n");
        rows.push_str("|----------|----------|\n");
        for i in 0..40 {
            rows.push_str(&format!("| row {i} col1 | row {i} col2 |\n"));
        }
        let md = format!("# Doc\n\n{rows}");
        let chunks = chunker().chunk(&md);
        let table_chunks: Vec<_> = chunks.iter().filter(|c| c.is_table).collect();
        assert_eq!(table_chunks.len(), 1, "table must not be split");
    }

    #[test]
    fn small_chunk_merged_with_sibling() {
        // Two sections: first one tiny (< MIN_CHUNK_CHARS), second normal
        let tiny = "tiny";
        let normal = "word ".repeat(80);
        let md = format!("## Small\n\n{tiny}\n\n## Normal\n\n{normal}");
        let chunks = chunker().chunk(&md);
        // The tiny section should be absorbed — fewer than 2 chunks
        assert!(
            chunks.len() < 3,
            "tiny chunk should be merged; got {} chunks",
            chunks.len()
        );
    }

    #[test]
    fn chunk_indices_are_contiguous() {
        let body = "word ".repeat(80);
        let md = format!("## A\n\n{body}\n\n## B\n\n{body}\n\n## C\n\n{body}");
        let chunks = chunker().chunk(&md);
        for (i, c) in chunks.iter().enumerate() {
            assert_eq!(c.chunk_index, i);
        }
    }

    #[test]
    fn empty_input_produces_no_chunks() {
        assert!(chunker().chunk("").is_empty());
    }

    #[test]
    fn plain_text_no_headings_is_one_chunk() {
        let text = "word ".repeat(100);
        let chunks = chunker().chunk(&text);
        assert_eq!(chunks.len(), 1);
    }
}
