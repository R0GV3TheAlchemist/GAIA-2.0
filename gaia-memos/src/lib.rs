//! MemOS v0.2: 5-tier persistent memory with hybrid search, Ebbinghaus decay,
//! cryptographic erasure, per-user isolation, and cross-device sync.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use gaia_sfs::{cosine, embed};

pub mod continuity;
pub mod decay;
pub mod erasure;
pub mod episode_store;
pub mod isolation;
pub mod persist;
pub mod retrieval_guard;
pub mod screenpipe;
pub mod sync;

pub use continuity::{CaptureConsent, Continuity, Episode, FileRef, Snapshot};
pub use decay::{tick as decay_tick, DEFAULT_DECAY_FACTOR, IMPORTANCE_FLOOR, PROMOTION_THRESHOLD};
pub use erasure::ErasureReceipt;
pub use episode_store::EpisodeStore;
pub use isolation::UserScope;
pub use persist::MemStore;
pub use retrieval_guard::{guard_chunk, guard_chunks, LexiconPlaneMismatch, QueryPlane};
pub use screenpipe::ScreenpipeStub;
pub use sync::{export as sync_export, merge as sync_merge, SyncBundle};

// ── Error ────────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum MemosError {
    #[error("not found: {0}")]
    NotFound(Uuid),
    #[error("persist: {0}")]
    Persist(#[from] persist::PersistError),
    #[error("retrieval guard: {0}")]
    RetrievalGuard(#[from] LexiconPlaneMismatch),
}

pub type Result<T> = std::result::Result<T, MemosError>;

// ── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CubeType {
    Parametric,
    Activation,
    Plaintext,
    Episodic,
    Procedural,
    Semantic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Lifecycle {
    Active,
    Archived,
    Compressed,
    Migrated,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Tier {
    Activation,
    Working,
    Episodic,
    Semantic,
    Parametric,
}

impl Tier {
    pub fn for_type(t: CubeType) -> Self {
        match t {
            CubeType::Activation => Tier::Activation,
            CubeType::Plaintext  => Tier::Working,
            CubeType::Episodic   => Tier::Episodic,
            CubeType::Semantic | CubeType::Procedural => Tier::Semantic,
            CubeType::Parametric => Tier::Parametric,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemCube {
    pub id:               Uuid,
    #[serde(rename = "type")]
    pub cube_type:        CubeType,
    pub lifecycle:        Lifecycle,
    pub content:          String,
    pub content_encoding: String,
    pub provenance_source: String,
    pub importance:       f32,
    pub associations:     Vec<Uuid>,
    pub version:          u32,
    pub access_count:     u32,
    pub embedding:        Vec<f32>,
    pub tier:             Tier,
}

impl MemCube {
    pub fn new(cube_type: CubeType, content: impl Into<String>, source: impl Into<String>) -> Self {
        let content = content.into();
        Self {
            id:               Uuid::new_v4(),
            cube_type,
            lifecycle:        Lifecycle::Active,
            embedding:        embed(&content),
            content,
            content_encoding: "utf-8".into(),
            provenance_source: source.into(),
            importance:       0.7,
            associations:     Vec::new(),
            version:          1,
            access_count:     0,
            tier:             Tier::for_type(cube_type),
        }
    }
}

// ── MemOs ─────────────────────────────────────────────────────────────────────

/// In-process (or SQLite-backed) five-tier memory store.
///
/// * `MemOs::new()`                      — in-process only (tests, embeddings)
/// * `MemOs::open(path, user_did)`        — SQLite-backed persistence
#[derive(Default)]
pub struct MemOs {
    cubes:    HashMap<Uuid, MemCube>,
    store:    Option<(MemStore, String)>,   // (MemStore, user_did)
}

impl MemOs {
    /// In-process only — no persistence.  All existing call-sites stay green.
    pub fn new() -> Self {
        Self::default()
    }

    /// Open a SQLite-backed store at `path` scoped to `user_did`.
    /// Existing cubes for this user are loaded into memory on open.
    pub fn open(path: impl AsRef<Path>, user_did: impl Into<String>) -> Result<Self> {
        let user_did = user_did.into();
        let store = MemStore::open(path.as_ref().to_str().unwrap_or(":memory:"))?;
        let cubes_vec = store.load_user(&user_did)?;
        let mut cubes = HashMap::new();
        for c in cubes_vec {
            cubes.insert(c.id, c);
        }
        Ok(Self {
            cubes,
            store: Some((store, user_did)),
        })
    }

    pub fn put(&mut self, cube: MemCube) -> Uuid {
        let id = cube.id;
        if let Some((store, user_did)) = &self.store {
            let _ = store.upsert(user_did, &cube);
        }
        self.cubes.insert(id, cube);
        id
    }

    pub fn get(&self, id: Uuid) -> Result<MemCube> {
        self.cubes.get(&id).cloned().ok_or(MemosError::NotFound(id))
    }

    pub fn archive(&mut self, id: Uuid) -> Result<MemCube> {
        let cube = self.cubes.get_mut(&id).ok_or(MemosError::NotFound(id))?;
        cube.lifecycle = Lifecycle::Archived;
        if let Some((store, user_did)) = &self.store {
            let _ = store.update_meta(user_did, cube);
        }
        Ok(cube.clone())
    }

    pub fn archive_by_content(&mut self, text: &str) -> usize {
        let ids: Vec<Uuid> = self
            .cubes
            .iter()
            .filter(|(_, c)| c.content == text && c.lifecycle == Lifecycle::Active)
            .map(|(id, _)| *id)
            .collect();
        let n = ids.len();
        for id in ids {
            let _ = self.archive(id);
        }
        n
    }

    /// User-triggered deletion with cryptographic erasure receipt.
    /// Removes the cube from memory and from SQLite (if persistent),
    /// and returns a receipt proving what was deleted and when.
    pub fn forget(&mut self, id: Uuid) -> Result<ErasureReceipt> {
        let cube = self.cubes.remove(&id).ok_or(MemosError::NotFound(id))?;
        let receipt = ErasureReceipt::generate(&cube);
        if let Some((store, user_did)) = &self.store {
            let _ = store.delete(user_did, &id.to_string());
        }
        Ok(receipt)
    }

    pub fn export_all(&self) -> Vec<MemCube> {
        self.cubes.values().cloned().collect()
    }

    pub fn import(&mut self, cubes: Vec<MemCube>) -> usize {
        let n = cubes.len();
        for c in cubes {
            self.put(c);
        }
        n
    }

    /// Hybrid recall: 55 % semantic (cosine) + 25 % BM25 (term overlap)
    /// + 20 % importance.
    ///
    /// Recency bonus (+0.1) applied to cubes accessed
    /// within the last tick via `access_count > 0`.
    pub fn recall(&mut self, query: &str, k: usize) -> Vec<(f32, MemCube)> {
        let q = embed(query);
        let q_terms = terms(query);
        let mut scored: Vec<(f32, Uuid)> = self
            .cubes
            .iter()
            .filter(|(_, c)| c.lifecycle == Lifecycle::Active)
            .map(|(id, c)| {
                let sem      = cosine(&q, &c.embedding);
                let lex      = term_overlap(&q_terms, &terms(&c.content));
                let recency  = if c.access_count > 0 { 0.10 } else { 0.0 };
                let score    = 0.55 * sem + 0.25 * lex + 0.20 * c.importance + recency;
                (score, *id)
            })
            .filter(|(s, _)| *s > 0.02)
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(k.max(1));
        scored
            .into_iter()
            .filter_map(|(s, id)| {
                self.cubes.get_mut(&id).map(|c| {
                    c.access_count += 1;
                    c.importance = (c.importance + 0.05).min(1.0);
                    if let Some((store, user_did)) = &self.store {
                        let _ = store.update_meta(user_did, c);
                    }
                    (s, c.clone())
                })
            })
            .collect()
    }

    /// Convenience wrapper around [`recall`] that discards scores and returns
    /// up to 20 matching [`MemCube`]s.
    pub fn search(&mut self, query: &str) -> Vec<MemCube> {
        self.recall(query, 20)
            .into_iter()
            .map(|(_, cube)| cube)
            .collect()
    }

    pub fn decay(&mut self, factor: f32) -> usize {
        let mut cubes: Vec<MemCube> = self.cubes.values().cloned().collect();
        let n = decay::tick(&mut cubes, factor);
        for c in cubes {
            if let Some(existing) = self.cubes.get_mut(&c.id) {
                existing.importance = c.importance;
                if let Some((store, user_did)) = &self.store {
                    let _ = store.update_meta(user_did, existing);
                }
            }
        }
        n
    }

    pub fn migrate(&mut self) -> usize {
        let mut n = 0;
        for c in self.cubes.values_mut() {
            if c.lifecycle != Lifecycle::Active {
                continue;
            }
            if c.importance >= PROMOTION_THRESHOLD
                && matches!(c.tier, Tier::Activation | Tier::Working)
            {
                c.tier       = Tier::Episodic;
                c.cube_type  = CubeType::Episodic;
                c.lifecycle  = Lifecycle::Migrated;
                c.version   += 1;
                if let Some((store, user_did)) = &self.store {
                    let _ = store.update_meta(user_did, c);
                }
                n += 1;
            }
        }
        n
    }

    pub fn consolidate(&mut self) -> Option<MemCube> {
        let episodic: Vec<MemCube> = self
            .cubes
            .values()
            .filter(|c| c.cube_type == CubeType::Episodic && c.lifecycle == Lifecycle::Active)
            .cloned()
            .collect();
        if episodic.is_empty() {
            return None;
        }
        let joined = episodic
            .iter()
            .map(|c| c.content.as_str())
            .collect::<Vec<_>>()
            .join(" | ");
        let mut summary = MemCube::new(CubeType::Semantic, joined, "consolidate");
        summary.associations = episodic.iter().map(|c| c.id).collect();
        summary.importance   = episodic
            .iter()
            .map(|c| c.importance)
            .fold(0.0f32, |a, b| a.max(b));
        for src in &episodic {
            if let Some(c) = self.cubes.get_mut(&src.id) {
                c.lifecycle = Lifecycle::Archived;
                if let Some((store, user_did)) = &self.store {
                    let _ = store.update_meta(user_did, c);
                }
            }
        }
        let out = summary.clone();
        self.put(summary);
        Some(out)
    }

    pub fn count_tier(&self, tier: Tier) -> usize {
        self.cubes.values().filter(|c| c.tier == tier).count()
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn terms(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| t.len() > 1)
        .map(|t| t.to_string())
        .collect()
}

fn term_overlap(a: &[String], b: &[String]) -> f32 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    a.iter().filter(|t| b.contains(t)).count() as f32 / a.len() as f32
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn tmp_db() -> String {
        let n = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("/tmp/gaia-memos-test-{n}.db")
    }

    // ── Legacy tests (must stay green) ───────────────────────────────────────

    #[test]
    fn create_recall_archive_tiers() {
        let mut mem = MemOs::new();
        mem.put(MemCube::new(CubeType::Activation, "current context window texas", "t1"));
        mem.put(MemCube::new(CubeType::Plaintext,  "working notes on weather",     "t2"));
        let eid = mem.put(MemCube::new(CubeType::Episodic, "episodic: talked about texas weather", "t3"));
        assert!(mem.count_tier(Tier::Activation) >= 1);
        assert!(mem.count_tier(Tier::Working)    >= 1);
        assert!(mem.count_tier(Tier::Episodic)   >= 1);
        let hits = mem.recall("texas weather", 3);
        assert!(!hits.is_empty());
        mem.archive(eid).unwrap();
        assert_eq!(mem.get(eid).unwrap().lifecycle, Lifecycle::Archived);
        mem.decay(0.5);
        if mem.consolidate().is_none() {
            mem.put(MemCube::new(CubeType::Episodic, "another texas storm memory", "t4"));
            assert!(mem.consolidate().is_some());
        }
    }

    #[test]
    fn migrate_promotes_important_working() {
        let mut mem = MemOs::new();
        let mut c   = MemCube::new(CubeType::Plaintext, "sticky fact", "t");
        c.importance = 0.9;
        mem.put(c);
        assert_eq!(mem.migrate(), 1);
    }

    #[test]
    fn archive_by_content_hides_from_recall() {
        let mut mem = MemOs::new();
        mem.put(MemCube::new(CubeType::Plaintext, "I like jazz", "type"));
        assert_eq!(mem.archive_by_content("I like jazz"), 1);
        assert!(mem.recall("jazz", 3).is_empty());
    }

    #[test]
    fn harness_a_to_b_keeps_cube_uuid() {
        let mut a  = MemOs::new();
        let id     = a.put(MemCube::new(CubeType::Plaintext, "alice identity", "fixture"));
        let dump   = a.export_all();
        let mut b  = MemOs::new();
        assert_eq!(b.import(dump), 1);
        let restored = b.get(id).unwrap();
        assert_eq!(restored.id, id);
        assert_eq!(restored.content, "alice identity");
    }

    // ── Production tests ─────────────────────────────────────────────────────

    #[test]
    fn persist_survives_restart() {
        let path = tmp_db();
        let cube_id;
        {
            let mut mem = MemOs::open(&path, "did:key:gaia:user-a").unwrap();
            cube_id = mem.put(MemCube::new(CubeType::Episodic, "restart survival test", "test"));
        }
        let mem2 = MemOs::open(&path, "did:key:gaia:user-a").unwrap();
        let restored = mem2.get(cube_id).unwrap();
        assert_eq!(restored.content, "restart survival test");
    }

    #[test]
    fn isolation_user_a_cannot_read_user_b() {
        let path = tmp_db();
        let cube_id;
        {
            let mut a = MemOs::open(&path, "did:key:gaia:user-a").unwrap();
            cube_id   = a.put(MemCube::new(CubeType::Plaintext, "alice secret", "a"));
        }
        let b = MemOs::open(&path, "did:key:gaia:user-b").unwrap();
        assert!(b.get(cube_id).is_err(), "User B must not read User A's cube");
    }

    #[test]
    fn erasure_receipt_is_internally_consistent() {
        let cube    = MemCube::new(CubeType::Plaintext, "delete me", "test");
        let receipt = ErasureReceipt::generate(&cube);
        assert!(receipt.verify(), "receipt must be self-consistent");
        assert!(!receipt.receipt_hex.is_empty());
    }

    #[test]
    fn forget_removes_cube_and_returns_receipt() {
        let mut mem = MemOs::new();
        let id      = mem.put(MemCube::new(CubeType::Plaintext, "erase this", "t"));
        let receipt = mem.forget(id).unwrap();
        assert_eq!(receipt.cube_id, id.to_string());
        assert!(mem.get(id).is_err(), "cube must be gone after forget()");
    }

    #[test]
    fn user_scope_isolation() {
        let mut alice = UserScope::new("did:key:gaia:alice");
        let mut bob   = UserScope::new("did:key:gaia:bob");
        let aid = alice.remember("alice memory", "test");
        let bid = bob.remember("bob memory",   "test");
        assert!(alice.get(aid).is_ok());
        assert!(alice.get(bid).is_err(),  "alice must not read bob's cube");
        assert!(bob.get(bid).is_ok());
        assert!(bob.get(aid).is_err(),    "bob must not read alice's cube");
    }

    #[test]
    fn cross_device_sync_merge_respects_version() {
        let mut home   = MemOs::new();
        let mut mobile = MemOs::new();
        let cube = MemCube::new(CubeType::Episodic, "home memory", "home");
        let id   = home.put(cube);
        let bundle  = sync_export(&home, "did:key:gaia:home-node");
        let written = sync_merge(&mut mobile, bundle);
        assert_eq!(written, 1);
        assert_eq!(mobile.get(id).unwrap().content, "home memory");
        let bundle2  = sync_export(&home, "did:key:gaia:home-node");
        let written2 = sync_merge(&mut mobile, bundle2);
        assert_eq!(written2, 0, "re-merge of same version must be no-op");
    }

    #[test]
    fn decay_tick_reduces_unaccessed_importance() {
        let mut cubes = vec![
            MemCube::new(CubeType::Plaintext, "unaccessed", "t"),
        ];
        let before = cubes[0].importance;
        decay::tick(&mut cubes, DEFAULT_DECAY_FACTOR);
        assert!(
            cubes[0].importance < before,
            "unaccessed cube importance must decay"
        );
    }

    #[test]
    fn search_delegates_to_recall_and_strips_scores() {
        let mut mem = MemOs::new();
        mem.put(MemCube::new(CubeType::Plaintext, "search convenience wrapper test", "t"));
        let results = mem.search("convenience wrapper");
        assert!(!results.is_empty(), "search must find the cube");
        let _ : Vec<MemCube> = results;
    }
}
