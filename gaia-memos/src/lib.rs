//! MemOS v0.1: MemCube store and five-tier hierarchy + Continuity (Blueprint 59).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use gaia_sfs::{cosine, embed};

pub mod continuity;
pub mod episode_store;

pub use continuity::{CaptureConsent, Continuity, Episode, Snapshot};
pub use episode_store::EpisodeStore;

#[derive(Debug, Error)]
pub enum MemosError {
    #[error("not found: {0}")]
    NotFound(Uuid),
}

pub type Result<T> = std::result::Result<T, MemosError>;

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
            CubeType::Plaintext => Tier::Working,
            CubeType::Episodic => Tier::Episodic,
            CubeType::Semantic | CubeType::Procedural => Tier::Semantic,
            CubeType::Parametric => Tier::Parametric,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemCube {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub cube_type: CubeType,
    pub lifecycle: Lifecycle,
    pub content: String,
    pub content_encoding: String,
    pub provenance_source: String,
    pub importance: f32,
    pub associations: Vec<Uuid>,
    pub version: u32,
    pub access_count: u32,
    pub embedding: Vec<f32>,
    pub tier: Tier,
}

impl MemCube {
    pub fn new(cube_type: CubeType, content: impl Into<String>, source: impl Into<String>) -> Self {
        let content = content.into();
        Self {
            id: Uuid::new_v4(),
            cube_type,
            lifecycle: Lifecycle::Active,
            embedding: embed(&content),
            content,
            content_encoding: "utf-8".into(),
            provenance_source: source.into(),
            importance: 0.7,
            associations: Vec::new(),
            version: 1,
            access_count: 0,
            tier: Tier::for_type(cube_type),
        }
    }
}

#[derive(Default)]
pub struct MemOs {
    cubes: HashMap<Uuid, MemCube>,
}

impl MemOs {
    pub fn new() -> Self { Self::default() }

    pub fn put(&mut self, cube: MemCube) -> Uuid {
        let id = cube.id;
        self.cubes.insert(id, cube);
        id
    }

    pub fn get(&self, id: Uuid) -> Result<MemCube> {
        self.cubes.get(&id).cloned().ok_or(MemosError::NotFound(id))
    }

    pub fn archive(&mut self, id: Uuid) -> Result<MemCube> {
        let cube = self.cubes.get_mut(&id).ok_or(MemosError::NotFound(id))?;
        cube.lifecycle = Lifecycle::Archived;
        Ok(cube.clone())
    }

    pub fn archive_by_content(&mut self, text: &str) -> usize {
        let ids: Vec<Uuid> = self.cubes.iter()
            .filter(|(_, c)| c.content == text && c.lifecycle == Lifecycle::Active)
            .map(|(id, _)| *id)
            .collect();
        let n = ids.len();
        for id in ids {
            let _ = self.archive(id);
        }
        n
    }

    /// Harness dump for A→B migration. IDs are preserved on import.
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

    pub fn recall(&mut self, query: &str, k: usize) -> Vec<(f32, MemCube)> {
        let q = embed(query);
        let q_terms = terms(query);
        let mut scored: Vec<(f32, Uuid)> = self
            .cubes
            .iter()
            .filter(|(_, c)| c.lifecycle == Lifecycle::Active)
            .map(|(id, c)| {
                let sem = cosine(&q, &c.embedding);
                let lex = term_overlap(&q_terms, &terms(&c.content));
                let score = 0.55 * sem + 0.25 * lex + 0.20 * c.importance;
                (score, *id)
            })
            .filter(|(s, _)| *s > 0.02)
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(k.max(1));
        scored.into_iter().filter_map(|(s, id)| {
            self.cubes.get_mut(&id).map(|c| {
                c.access_count += 1;
                c.importance = (c.importance + 0.05).min(1.0);
                (s, c.clone())
            })
        }).collect()
    }

    pub fn decay(&mut self, factor: f32) -> usize {
        let mut n = 0;
        for c in self.cubes.values_mut() {
            if c.access_count == 0 && c.lifecycle == Lifecycle::Active {
                c.importance = (c.importance * factor).max(0.01);
                n += 1;
            }
        }
        n
    }

    pub fn migrate(&mut self) -> usize {
        let mut n = 0;
        for c in self.cubes.values_mut() {
            if c.lifecycle != Lifecycle::Active { continue; }
            if c.importance >= 0.85 && matches!(c.tier, Tier::Activation | Tier::Working) {
                c.tier = Tier::Episodic;
                c.cube_type = CubeType::Episodic;
                c.lifecycle = Lifecycle::Migrated;
                c.version += 1;
                n += 1;
            }
        }
        n
    }

    pub fn consolidate(&mut self) -> Option<MemCube> {
        let episodic: Vec<MemCube> = self.cubes.values()
            .filter(|c| c.cube_type == CubeType::Episodic && c.lifecycle == Lifecycle::Active)
            .cloned().collect();
        if episodic.is_empty() { return None; }
        let joined = episodic.iter().map(|c| c.content.as_str()).collect::<Vec<_>>().join(" | ");
        let mut summary = MemCube::new(CubeType::Semantic, joined, "consolidate");
        summary.associations = episodic.iter().map(|c| c.id).collect();
        summary.importance = episodic.iter().map(|c| c.importance).fold(0.0f32, |a, b| a.max(b));
        for src in &episodic {
            if let Some(c) = self.cubes.get_mut(&src.id) {
                c.lifecycle = Lifecycle::Archived;
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

fn terms(text: &str) -> Vec<String> {
    text.to_lowercase().split(|c: char| !c.is_ascii_alphanumeric()).filter(|t| t.len() > 1).map(|t| t.to_string()).collect()
}

fn term_overlap(a: &[String], b: &[String]) -> f32 {
    if a.is_empty() || b.is_empty() { return 0.0; }
    a.iter().filter(|t| b.contains(t)).count() as f32 / a.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_recall_archive_tiers() {
        let mut mem = MemOs::new();
        mem.put(MemCube::new(CubeType::Activation, "current context window texas", "t1"));
        mem.put(MemCube::new(CubeType::Plaintext, "working notes on weather", "t2"));
        let eid = mem.put(MemCube::new(CubeType::Episodic, "episodic: talked about texas weather", "t3"));
        assert!(mem.count_tier(Tier::Activation) >= 1);
        assert!(mem.count_tier(Tier::Working) >= 1);
        assert!(mem.count_tier(Tier::Episodic) >= 1);
        let hits = mem.recall("texas weather", 3);
        assert!(!hits.is_empty());
        assert!(!hits[0].1.content.is_empty());
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
        let mut c = MemCube::new(CubeType::Plaintext, "sticky fact", "t");
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
        let mut a = MemOs::new();
        let id = a.put(MemCube::new(CubeType::Plaintext, "alice identity", "fixture"));
        let dump = a.export_all();
        let mut b = MemOs::new();
        assert_eq!(b.import(dump), 1);
        let restored = b.get(id).unwrap();
        assert_eq!(restored.id, id);
        assert_eq!(restored.content, "alice identity");
    }
}
