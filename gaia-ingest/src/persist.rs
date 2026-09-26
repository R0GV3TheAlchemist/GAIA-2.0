//! File-backed citation index. JSONL, not Qdrant.

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::chunk_id::ChunkId;
use crate::dedup::ChunkStore;
use crate::embed::EmbeddingVector;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersistedChunk {
    pub hex: String,
    pub text: String,
    pub model_id: String,
    pub embedding: Option<Vec<f32>>,
}

#[derive(Debug)]
pub struct FileChunkStore {
    path: PathBuf,
    inner: ChunkStore,
    rows: Vec<PersistedChunk>,
}

impl FileChunkStore {
    pub fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let mut inner = ChunkStore::new();
        let mut rows = Vec::new();
        if path.exists() {
            let file = File::open(&path)?;
            for line in BufReader::new(file).lines() {
                let line = line?;
                if line.trim().is_empty() {
                    continue;
                }
                let row: PersistedChunk = serde_json::from_str(&line).map_err(|e| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, e)
                })?;
                if let Ok(bytes) = hex::decode(&row.hex) {
                    if bytes.len() == 32 {
                        let mut arr = [0u8; 32];
                        arr.copy_from_slice(&bytes);
                        inner.record(ChunkId::from_bytes(arr));
                    }
                }
                rows.push(row);
            }
        }
        Ok(Self { path, inner, rows })
    }

    pub fn record(
        &mut self,
        text: &str,
        model_id: &str,
        embedding: Option<&EmbeddingVector>,
    ) -> std::io::Result<ChunkId> {
        let id = ChunkId::from_text(text);
        if self.inner.contains(id) {
            return Ok(id);
        }
        self.inner.record(id);
        let row = PersistedChunk {
            hex: id.to_hex(),
            text: text.to_string(),
            model_id: model_id.to_string(),
            embedding: embedding.map(|e| e.as_slice().to_vec()),
        };
        self.rows.push(row.clone());
        if let Some(parent) = self.path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        writeln!(file, "{}", serde_json::to_string(&row).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })?)?;
        Ok(id)
    }

    pub fn contains_hex(&self, hex_id: &str) -> bool {
        self.inner.contains_hex(hex_id)
    }

    pub fn store(&self) -> &ChunkStore {
        &self.inner
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embed::EmbeddingModel;
    use crate::hash_embed::HashingEmbedder;

    #[test]
    fn persist_survives_reopen() {
        let dir = std::env::temp_dir().join(format!("gaia-persist-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("chunks.jsonl");
        let _ = std::fs::remove_file(&path);
        let text = "The treaty was signed in 1992.";
        {
            let mut store = FileChunkStore::open(&path).unwrap();
            store.record(text, "none", None).unwrap();
        }
        let store = FileChunkStore::open(&path).unwrap();
        let hex = ChunkId::from_text(text).to_hex();
        assert!(store.contains_hex(&hex));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn persist_keeps_hash_embedding() {
        let dir = std::env::temp_dir().join(format!("gaia-persist-emb-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("chunks.jsonl");
        let _ = std::fs::remove_file(&path);
        let embedder = HashingEmbedder::default();
        let vecs = embedder.embed(&["hello earth twin"]).unwrap();
        let mut store = FileChunkStore::open(&path).unwrap();
        store
            .record("hello earth twin", embedder.model_id(), Some(&vecs[0]))
            .unwrap();
        assert_eq!(vecs[0].dim(), 384);
        assert!(store.contains_hex(&ChunkId::from_text("hello earth twin").to_hex()));
    }
}
