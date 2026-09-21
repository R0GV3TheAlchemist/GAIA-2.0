//! Raw artifact references and the `ArtifactStore` trait.
//!
//! `gaia-ingest` does not own bytes — it owns *pointers* to bytes that
//! live in `gaia-sfs`.  `RawArtifactRef` is that pointer: an SFS path,
//! a SHA-256 content ID, a MIME type, and a byte count.
//!
//! The `ArtifactStore` trait is deliberately thin so that:
//! - unit tests can inject an in-memory store (`MemArtifactStore`)
//! - production code wires in the real `gaia-sfs::Sfs` instance
//! - future slices can add S3 / MinIO / Iceberg backends without
//!   touching the schema layer

use serde::{Deserialize, Serialize};

/// A pointer to a raw payload stored in SFS.
///
/// The bytes themselves live in `gaia-sfs`; this struct is carried
/// inside `NormalizedObservation` so every consumer knows where to
/// find the original artifact without re-fetching from the source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawArtifactRef {
    /// SFS-relative path (e.g. `"earth/copernicus/s2/2024-01-15/tile.tif"`).
    pub sfs_path: String,
    /// SHA-256 content ID assigned by SFS at write time (hex, 64 chars).
    pub cid: String,
    /// MIME type of the stored bytes (e.g. `"image/tiff"`, `"application/json"`).
    pub content_type: String,
    /// Size of the stored payload in bytes.
    pub bytes: usize,
}

impl RawArtifactRef {
    /// Returns `true` if the ref is internally consistent:
    /// - `sfs_path` is non-empty and does not traverse (`..`)
    /// - `cid` is a 64-character hex string
    /// - `bytes > 0`
    pub fn is_valid(&self) -> bool {
        !self.sfs_path.is_empty()
            && !self.sfs_path.split('/').any(|seg| seg == "..")
            && self.cid.len() == 64
            && self.cid.chars().all(|c| c.is_ascii_hexdigit())
            && !self.content_type.is_empty()
            && self.bytes > 0
    }
}

/// Errors produced by `ArtifactStore` implementations.
#[derive(Debug, thiserror::Error)]
pub enum IngestError {
    #[error("artifact store: {0}")]
    Store(String),
    #[error("empty payload — will not store zero bytes")]
    EmptyPayload,
    #[error("invalid path '{0}' — must be non-empty and path-traversal-free")]
    InvalidPath(String),
}

/// Trait implemented by any artifact storage backend.
///
/// The production implementation delegates to `gaia-sfs::Sfs`.
/// Tests use `MemArtifactStore` below.
pub trait ArtifactStore: Send + Sync {
    fn store(
        &mut self,
        data: &[u8],
        sfs_path: &str,
        content_type: &str,
    ) -> Result<RawArtifactRef, IngestError>;
}

// ── In-memory test implementation ────────────────────────────────────────────

/// A purely in-memory `ArtifactStore` for unit tests.
/// Computes a deterministic fake CID (SHA-256 of the data via gaia-sfs's
/// public `sha256_hex` function is not available here, so we use a
/// fixed-length hex of the data length for test purposes only).
pub struct MemArtifactStore {
    pub stored: Vec<(String, Vec<u8>)>,
}

impl MemArtifactStore {
    pub fn new() -> Self {
        Self { stored: Vec::new() }
    }
}

impl Default for MemArtifactStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactStore for MemArtifactStore {
    fn store(
        &mut self,
        data: &[u8],
        sfs_path: &str,
        content_type: &str,
    ) -> Result<RawArtifactRef, IngestError> {
        if data.is_empty() {
            return Err(IngestError::EmptyPayload);
        }
        let trimmed = sfs_path.trim().trim_start_matches('/');
        if trimmed.is_empty() || trimmed.split('/').any(|s| s == "..") {
            return Err(IngestError::InvalidPath(sfs_path.into()));
        }
        // Fake CID for tests: SHA-256 via sha2
        use sha2::{Digest, Sha256};
        let cid = hex::encode(Sha256::digest(data));
        let bytes = data.len();
        self.stored.push((trimmed.to_string(), data.to_vec()));
        Ok(RawArtifactRef {
            sfs_path: trimmed.to_string(),
            cid,
            content_type: content_type.into(),
            bytes,
        })
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_store_happy_path() {
        let mut store = MemArtifactStore::new();
        let r = store
            .store(b"sentinel-2 tiff bytes", "earth/copernicus/s2/tile.tif", "image/tiff")
            .unwrap();
        assert!(r.is_valid());
        assert_eq!(r.bytes, 21);
        assert_eq!(r.content_type, "image/tiff");
        assert_eq!(r.sfs_path, "earth/copernicus/s2/tile.tif");
    }

    #[test]
    fn mem_store_rejects_empty_payload() {
        let mut store = MemArtifactStore::new();
        assert!(matches!(
            store.store(b"", "some/path", "application/json"),
            Err(IngestError::EmptyPayload)
        ));
    }

    #[test]
    fn mem_store_rejects_path_traversal() {
        let mut store = MemArtifactStore::new();
        assert!(matches!(
            store.store(b"data", "../../etc/passwd", "text/plain"),
            Err(IngestError::InvalidPath(_))
        ));
    }

    #[test]
    fn raw_artifact_ref_validity() {
        let valid = RawArtifactRef {
            sfs_path: "earth/noaa/obs.json".into(),
            cid: "a".repeat(64),
            content_type: "application/json".into(),
            bytes: 512,
        };
        assert!(valid.is_valid());

        let bad_cid = RawArtifactRef { cid: "tooshort".into(), ..valid.clone() };
        assert!(!bad_cid.is_valid());

        let path_traversal = RawArtifactRef { sfs_path: "../secret".into(), ..valid.clone() };
        assert!(!path_traversal.is_valid());

        let zero_bytes = RawArtifactRef { bytes: 0, ..valid };
        assert!(!zero_bytes.is_valid());
    }
}
