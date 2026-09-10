//! Continuity substrate (Blueprint 59). Snapshots carry open file path+hash.
//! License: Apache-2.0

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::episode_store::EpisodeStore;
use crate::{CubeType, MemCube, MemOs};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CaptureConsent {
    pub screen: bool,
    pub audio: bool,
    pub keys: bool,
    pub clipboard: bool,
    pub files: bool,
}

impl CaptureConsent {
    pub fn all_off() -> Self {
        Self::default()
    }

    pub fn any_on(&self) -> bool {
        self.screen || self.audio || self.keys || self.clipboard || self.files
    }
}

#[derive(Clone, Debug)]
pub struct Episode {
    pub t_unix_ms: u64,
    pub text: String,
    pub modality: String,
    pub snapshot_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileRef {
    pub path: String,
    pub hash: String,
}

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub id: String,
    pub label: String,
    pub current_step: String,
    pub open_files: Vec<FileRef>,
}

pub struct Continuity {
    pub consent: CaptureConsent,
    episodes: VecDeque<Episode>,
    snapshots: Vec<Snapshot>,
    max_ring: usize,
    store: Option<EpisodeStore>,
}

impl Continuity {
    pub fn new() -> Self {
        Self {
            consent: CaptureConsent::all_off(),
            episodes: VecDeque::new(),
            snapshots: Vec::new(),
            max_ring: 256,
            store: None,
        }
    }

    pub fn attach_store(&mut self, store: EpisodeStore) {
        self.store = Some(store);
    }

    pub fn remember_life(&mut self, episode: Episode) -> Result<(), &'static str> {
        if !self.consent.any_on() {
            return Err("capture denied: all consent flags false");
        }
        if let Some(store) = &self.store {
            store
                .insert(episode.t_unix_ms as i64, &episode.text, &episode.modality)
                .map_err(|_| "sqlite insert failed")?;
        }
        if self.episodes.len() >= self.max_ring {
            self.episodes.pop_front();
        }
        self.episodes.push_back(episode);
        Ok(())
    }

    pub fn ingest_episode(&mut self, episode: Episode, memos: &mut MemOs) -> Result<Uuid, &'static str> {
        let cube = MemCube::new(CubeType::Plaintext, episode.text.clone(), episode.modality.clone());
        self.remember_life(episode)?;
        Ok(memos.put(cube))
    }

    pub fn ask_history(&self, vague: &str) -> Vec<Episode> {
        if let Some(store) = &self.store {
            if let Ok(rows) = store.search(vague, 8) {
                if !rows.is_empty() {
                    return rows
                        .into_iter()
                        .map(|(t, text, modality)| Episode {
                            t_unix_ms: t as u64,
                            text,
                            modality,
                            snapshot_id: None,
                        })
                        .collect();
                }
            }
        }
        self.episodes.iter().rev().take(8).cloned().collect()
    }

    pub fn forget(&mut self, text: &str) -> Result<usize, &'static str> {
        self.episodes.retain(|e| e.text != text);
        if let Some(store) = &self.store {
            return store.forget(text).map_err(|_| "sqlite forget failed");
        }
        Ok(0)
    }

    pub fn forget_in(&mut self, text: &str, memos: &mut MemOs) -> Result<usize, &'static str> {
        let n = self.forget(text)?;
        memos.archive_by_content(text);
        Ok(n)
    }

    pub fn correct(&mut self, old_text: &str, new_text: &str) -> Result<usize, &'static str> {
        for e in &mut self.episodes {
            if e.text == old_text {
                e.text = new_text.to_string();
            }
        }
        if let Some(store) = &self.store {
            return store.correct(old_text, new_text).map_err(|_| "sqlite correct failed");
        }
        Ok(0)
    }

    pub fn pause_world(&mut self, label: &str, current_step: &str) -> String {
        self.pause_world_with_files(label, current_step, Vec::new())
    }

    pub fn pause_world_with_files(
        &mut self,
        label: &str,
        current_step: &str,
        open_files: Vec<FileRef>,
    ) -> String {
        let id = format!("snap_{}", self.snapshots.len());
        let json = serde_json::to_string(&open_files).unwrap_or_else(|_| "[]".into());
        if let Some(store) = &self.store {
            let _ = store.save_snapshot(&id, label, current_step, &json);
        }
        self.snapshots.push(Snapshot {
            id: id.clone(),
            label: label.to_string(),
            current_step: current_step.to_string(),
            open_files,
        });
        id
    }

    pub fn pause_into_memos(&mut self, label: &str, current_step: &str, memos: &mut MemOs) -> Uuid {
        let _id = self.pause_world(label, current_step);
        memos.put(MemCube::new(
            CubeType::Activation,
            format!("{label}::{current_step}"),
            "snapshot",
        ))
    }

    pub fn restore_world(&self, prompt: &str) -> Option<Snapshot> {
        if let Some(store) = &self.store {
            if let Ok(Some((id, label, current_step, files_json))) = store.find_snapshot(prompt) {
                let open_files = serde_json::from_str(&files_json).unwrap_or_default();
                return Some(Snapshot {
                    id,
                    label,
                    current_step,
                    open_files,
                });
            }
        }
        self.snapshots.iter().rev().find(|s| {
            s.id == prompt || s.label == prompt || prompt.contains(&s.label)
        }).cloned()
    }
}

impl Default for Continuity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restore_includes_open_files() {
        let path = std::env::temp_dir().join("gaia_filesnap.sqlite");
        let _ = std::fs::remove_file(&path);
        {
            let mut c = Continuity::new();
            c.attach_store(EpisodeStore::open(path.to_str().unwrap()).unwrap());
            c.pause_world_with_files(
                "destinE memo",
                "DRAFTING_CARE",
                vec![FileRef {
                    path: "Documents/CARE.md".into(),
                    hash: "abc123".into(),
                }],
            );
        }
        let mut c = Continuity::new();
        c.attach_store(EpisodeStore::open(path.to_str().unwrap()).unwrap());
        let snap = c.restore_world("destinE memo").unwrap();
        assert_eq!(snap.current_step, "DRAFTING_CARE");
        assert_eq!(snap.open_files[0].path, "Documents/CARE.md");
        assert_eq!(snap.open_files[0].hash, "abc123");
        let _ = std::fs::remove_file(&path);
    }
}
