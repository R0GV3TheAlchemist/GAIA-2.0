//! Semantic File System v0.1.
//!
//! Directory-backed store with SHA-256 content IDs, hashed-token embeddings,
//! and a cryptographic parent-cid lineage. `posix_root()` is the tree that
//! unmodified POSIX tools can read and write; a FUSE mount is an optional
//! export of the same tree (not required for unprivileged CI).

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

pub const EMBED_DIM: usize = 64;

#[derive(Debug, Error)]
pub enum SfsError {
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("invalid path: {0}")]
    InvalidPath(String),
}

pub type Result<T> = std::result::Result<T, SfsError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticMeta {
    pub who: String,
    pub when_unix: u64,
    pub context: String,
    pub intent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SfsObject {
    pub path: String,
    pub cid: String,
    pub parent_cid: Option<String>,
    pub meta: SemanticMeta,
    pub size: usize,
    pub embedding: Vec<f32>,
}

pub struct Sfs {
    root: PathBuf,
    index: HashMap<String, SfsObject>,
}

impl Sfs {
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(root.join("posix"))?;
        fs::create_dir_all(root.join("objects"))?;
        fs::create_dir_all(root.join("meta"))?;
        let mut sfs = Self { root, index: HashMap::new() };
        sfs.reload()?;
        Ok(sfs)
    }

    pub fn posix_root(&self) -> PathBuf {
        self.root.join("posix")
    }

    pub fn put(&mut self, path: &str, data: &[u8], meta: SemanticMeta) -> Result<SfsObject> {
        let rel = normalize_path(path)?;
        let cid = sha256_hex(data);
        let parent_cid = self.index.get(&rel).map(|o| o.cid.clone());
        let embedding = embed(&String::from_utf8_lossy(data));
        let obj = SfsObject {
            path: rel.clone(),
            cid: cid.clone(),
            parent_cid,
            meta,
            size: data.len(),
            embedding,
        };
        let posix = self.posix_root().join(&rel);
        if let Some(parent) = posix.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&posix, data)?;
        fs::write(self.root.join("objects").join(&cid), data)?;
        fs::write(self.root.join("meta").join(format!("{cid}.json")), serde_json::to_vec_pretty(&obj)?)?;
        self.write_lineage(&rel, &obj)?;
        self.index.insert(rel, obj.clone());
        Ok(obj)
    }

    pub fn get(&self, path: &str) -> Result<SfsObject> {
        let rel = normalize_path(path)?;
        self.index.get(&rel).cloned().ok_or_else(|| SfsError::NotFound(rel))
    }

    pub fn read(&self, path: &str) -> Result<Vec<u8>> {
        let obj = self.get(path)?;
        Ok(fs::read(self.root.join("objects").join(obj.cid))?)
    }

    pub fn lineage(&self, path: &str) -> Result<Vec<SfsObject>> {
        let mut out = Vec::new();
        let mut cur = Some(self.get(path)?);
        let mut guard = 0;
        while let Some(obj) = cur {
            let parent = obj.parent_cid.clone();
            out.push(obj);
            cur = match parent {
                Some(cid) => self.by_cid(&cid),
                None => None,
            };
            guard += 1;
            if guard > 10_000 { break; }
        }
        Ok(out)
    }

    pub fn search(&self, query: &str, k: usize) -> Vec<(f32, SfsObject)> {
        let q = embed(query);
        let mut scored: Vec<(f32, SfsObject)> = self
            .index
            .values()
            .map(|o| (cosine(&q, &o.embedding), o.clone()))
            .filter(|(s, _)| *s > 0.0)
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(k.max(1));
        scored
    }

    fn by_cid(&self, cid: &str) -> Option<SfsObject> {
        self.index.values().find(|o| o.cid == cid).cloned().or_else(|| {
            let p = self.root.join("meta").join(format!("{cid}.json"));
            fs::read(&p).ok().and_then(|b| serde_json::from_slice(&b).ok())
        })
    }

    fn write_lineage(&self, rel: &str, obj: &SfsObject) -> Result<()> {
        let mut f = fs::OpenOptions::new().create(true).append(true).open(self.root.join("lineage.log"))?;
        writeln!(f, "{} {} {} parent={}", obj.cid, rel, obj.meta.who, obj.parent_cid.as_deref().unwrap_or("genesis"))?;
        Ok(())
    }

    fn reload(&mut self) -> Result<()> {
        let meta_dir = self.root.join("meta");
        if !meta_dir.exists() { return Ok(()); }
        for entry in fs::read_dir(meta_dir)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) != Some("json") { continue; }
            let obj: SfsObject = serde_json::from_slice(&fs::read(entry.path())?)?;
            self.index.insert(obj.path.clone(), obj);
        }
        Ok(())
    }
}

fn normalize_path(path: &str) -> Result<String> {
    let trimmed = path.trim().trim_start_matches('/');
    if trimmed.is_empty() || trimmed.contains('\0') || trimmed.split('/').any(|p| p == "..") {
        return Err(SfsError::InvalidPath(path.into()));
    }
    Ok(trimmed.replace('\\', "/"))
}

pub fn sha256_hex(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    hex::encode(h.finalize())
}

pub fn embed(text: &str) -> Vec<f32> {
    let mut v = vec![0.0f32; EMBED_DIM];
    for tok in tokenize(text) {
        let h = sha256_hex(tok.as_bytes());
        let idx = u32::from_str_radix(&h[..8], 16).unwrap_or(0) as usize % EMBED_DIM;
        v[idx] += 1.0;
    }
    let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in &mut v { *x /= norm; }
    }
    v
}

pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| t.len() > 1)
        .map(|t| t.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn tmp() -> PathBuf {
        let n = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("gaia-sfs-{n}"))
    }

    fn meta() -> SemanticMeta {
        SemanticMeta { who: "did:key:gaia:test".into(), when_unix: 1, context: "test".into(), intent: "store".into() }
    }

    #[test]
    fn put_get_search_lineage_posix() {
        let mut sfs = Sfs::open(tmp()).unwrap();
        let a = sfs.put("docs/note.txt", b"gaia semantic file about texas weather", meta()).unwrap();
        assert_eq!(a.cid.len(), 64);
        let b = sfs.put("docs/note.txt", b"updated notes on texas weather and rain", meta()).unwrap();
        assert_eq!(b.parent_cid.as_deref(), Some(a.cid.as_str()));
        assert!(sfs.lineage("docs/note.txt").unwrap().len() >= 2);
        assert!(!sfs.search("texas weather", 3).is_empty());
        let body = std::fs::read_to_string(sfs.posix_root().join("docs/note.txt")).unwrap();
        assert!(body.contains("rain"));
    }
}
