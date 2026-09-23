//! `ChunkId` — a BLAKE3 content fingerprint for a [`DocumentChunk`].
//!
//! A `ChunkId` is a 32-byte BLAKE3 hash of the chunk's *normalised* text
//! (lowercased, interior whitespace collapsed to a single space, leading /
//! trailing whitespace stripped).  This makes the fingerprint:
//!
//! - **Stable across minor formatting changes** — extra blank lines, mixed
//!   case, or stray spaces do not produce a new id.
//! - **Sensitive to actual content changes** — any word-level edit produces
//!   a completely different hash.
//! - **Collision-resistant** — BLAKE3 provides 256-bit security.
//!
//! ## Usage
//!
//! ```rust
//! use gaia_ingest::{ChunkId, DocumentChunk};
//!
//! // Derive from an existing chunk:
//! let id = ChunkId::from_chunk(&chunk);
//!
//! // Or use the From impl:
//! let id = ChunkId::from(&chunk);
//!
//! // Use as a HashMap key:
//! use std::collections::HashMap;
//! let mut seen: HashMap<ChunkId, usize> = HashMap::new();
//! seen.insert(id, 42);
//! ```

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::document::DocumentChunk;

// ── ChunkId ──────────────────────────────────────────────────────────────────

/// A 32-byte BLAKE3 fingerprint of a [`DocumentChunk`]'s normalised text.
///
/// Two chunks with identical content (ignoring case and whitespace) will
/// produce the same `ChunkId`.  Use this for deduplication, cache keying,
/// and provenance cross-referencing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChunkId([u8; 32]);

impl ChunkId {
    /// Compute a `ChunkId` from the given text.
    ///
    /// Normalisation steps applied before hashing:
    /// 1. Lowercase the entire string.
    /// 2. Collapse every run of ASCII whitespace (`[ \t\r\n]+`) to a single space.
    /// 3. Strip leading and trailing spaces.
    pub fn from_text(text: &str) -> Self {
        let normalised = normalise(text);
        let hash = blake3::hash(normalised.as_bytes());
        Self(*hash.as_bytes())
    }

    /// Compute a `ChunkId` directly from a [`DocumentChunk`].
    pub fn from_chunk(chunk: &DocumentChunk) -> Self {
        Self::from_text(&chunk.text)
    }

    /// Return the raw 32-byte hash.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Encode as a 64-character lowercase hex string.
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl fmt::Display for ChunkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl From<&DocumentChunk> for ChunkId {
    fn from(chunk: &DocumentChunk) -> Self {
        Self::from_chunk(chunk)
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Lowercase + collapse whitespace.
fn normalise(s: &str) -> String {
    let lower = s.to_lowercase();
    let mut out = String::with_capacity(lower.len());
    let mut prev_space = true; // skip leading spaces
    for ch in lower.chars() {
        if ch.is_ascii_whitespace() {
            if !prev_space {
                out.push(' ');
                prev_space = true;
            }
        } else {
            out.push(ch);
            prev_space = false;
        }
    }
    // strip trailing space that `prev_space` logic may have appended
    if out.ends_with(' ') {
        out.pop();
    }
    out
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        document::{AccessTier, ConfidenceTier, DocumentChunk, DocumentKind},
        lexicon::{LexiconPlane, LexiconVoice},
        provenance::ProvenanceReceipt,
        schema::DataSource,
    };
    use std::collections::{BTreeMap, HashMap};

    fn minimal_chunk(text: &str) -> DocumentChunk {
        DocumentChunk {
            id:              "00000000-0000-0000-0000-000000000001".into(),
            text:            text.into(),
            char_count:      text.chars().count(),
            chunk_index:     0,
            total_chunks:    1,
            document_title:  "test".into(),
            document_uri:    "file:///test.md".into(),
            kind:            DocumentKind::SpecDocument,
            domain:          "test".into(),
            language:        "en".into(),
            authored_at_unix: 0,
            ttl_seconds:     None,
            confidence:      ConfidenceTier::High,
            access_tier:     AccessTier::Internal,
            access_control:  Vec::new(),
            attributes:      BTreeMap::new(),
            lexicon_plane:   LexiconPlane::Bridge,
            lexicon_voice:   LexiconVoice::Logos,
            provenance:      ProvenanceReceipt {
                source:           DataSource::InternalDocument,
                source_url:       "file:///test.md".into(),
                external_id:      "test".into(),
                fetched_at_unix:  1,
                observed_at_unix: 1,
                sha256:           "a".repeat(64),
                license:          "proprietary".into(),
            },
            artifact: None,
        }
    }

    #[test]
    fn same_text_same_id() {
        let a = ChunkId::from_text("Hello world");
        let b = ChunkId::from_text("Hello world");
        assert_eq!(a, b);
    }

    #[test]
    fn case_insensitive() {
        let a = ChunkId::from_text("Hello World");
        let b = ChunkId::from_text("hello world");
        assert_eq!(a, b);
    }

    #[test]
    fn whitespace_collapsed() {
        let a = ChunkId::from_text("hello   world");
        let b = ChunkId::from_text("hello world");
        assert_eq!(a, b);
    }

    #[test]
    fn leading_trailing_whitespace_stripped() {
        let a = ChunkId::from_text("  hello world  ");
        let b = ChunkId::from_text("hello world");
        assert_eq!(a, b);
    }

    #[test]
    fn different_content_different_id() {
        let a = ChunkId::from_text("hello world");
        let b = ChunkId::from_text("hello earth");
        assert_ne!(a, b);
    }

    #[test]
    fn from_chunk_matches_from_text() {
        let chunk = minimal_chunk("The quick brown fox.");
        let a = ChunkId::from_chunk(&chunk);
        let b = ChunkId::from_text("The quick brown fox.");
        assert_eq!(a, b);
    }

    #[test]
    fn from_trait_impl() {
        let chunk = minimal_chunk("trait test");
        let a = ChunkId::from(&chunk);
        let b = ChunkId::from_chunk(&chunk);
        assert_eq!(a, b);
    }

    #[test]
    fn display_is_64_char_hex() {
        let id = ChunkId::from_text("display test");
        let s = id.to_string();
        assert_eq!(s.len(), 64);
        assert!(s.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn can_be_used_as_hashmap_key() {
        let id = ChunkId::from_text("key test");
        let mut map: HashMap<ChunkId, &str> = HashMap::new();
        map.insert(id, "value");
        assert_eq!(map[&id], "value");
    }

    #[test]
    fn serde_roundtrip() {
        let id = ChunkId::from_text("serde test");
        let json = serde_json::to_string(&id).unwrap();
        let back: ChunkId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, back);
    }

    #[test]
    fn as_bytes_length() {
        let id = ChunkId::from_text("bytes test");
        assert_eq!(id.as_bytes().len(), 32);
    }
}
