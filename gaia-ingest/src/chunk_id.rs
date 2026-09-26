//! `ChunkId` — a BLAKE3 content fingerprint for a [`DocumentChunk`].

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::document::DocumentChunk;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChunkId([u8; 32]);

impl ChunkId {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn from_text(text: &str) -> Self {
        let normalised = normalise(text);
        let hash = blake3::hash(normalised.as_bytes());
        Self(*hash.as_bytes())
    }

    pub fn from_chunk(chunk: &DocumentChunk) -> Self {
        Self::from_text(&chunk.text)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

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

fn normalise(s: &str) -> String {
    let lower = s.to_lowercase();
    let mut out = String::with_capacity(lower.len());
    let mut prev_space = true;
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
    if out.ends_with(' ') {
        out.pop();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_text_same_id() {
        assert_eq!(ChunkId::from_text("Hello world"), ChunkId::from_text("Hello world"));
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(ChunkId::from_text("Hello World"), ChunkId::from_text("hello world"));
    }

    #[test]
    fn from_bytes_roundtrip() {
        let id = ChunkId::from_text("roundtrip");
        let back = ChunkId::from_bytes(*id.as_bytes());
        assert_eq!(id, back);
    }

    #[test]
    fn display_is_64_char_hex() {
        let s = ChunkId::from_text("display test").to_string();
        assert_eq!(s.len(), 64);
    }
}
