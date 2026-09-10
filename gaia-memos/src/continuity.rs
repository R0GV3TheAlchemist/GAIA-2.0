//! Continuity substrate (Blueprint 59).
//! Capture flags default **false**. Snapshots persist when a store is attached.
//! License: Apache-2.0

use std::collections::VecDeque;

use crate::episode_store::EpisodeStore;

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

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub id: String,
    pub label: String,
    pub current_step: String,
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

    pub fn pause_world(&mut self, label: &str, current_step: &str) -> String {
        let id = format!("snap_{}", self.snapshots.len());
        if let Some(store) = &self.store {
            let _ = store.save_snapshot(&id, label, current_step);
        }
        self.snapshots.push(Snapshot {
            id: id.clone(),
            label: label.to_string(),
            current_step: current_step.to_string(),
        });
        id
    }

    /// Hydrate by id/label/prompt. Prefers SQLite so this survives process restart.
    pub fn restore_world(&self, prompt: &str) -> Option<Snapshot> {
        if let Some(store) = &self.store {
            if let Ok(Some((id, label, current_step))) = store.find_snapshot(prompt) {
                return Some(Snapshot {
                    id,
                    label,
                    current_step,
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
    fn restore_after_new_process() {
        let path = std::env::temp_dir().join("gaia_part6_hydrate.sqlite");
        let _ = std::fs::remove_file(&path);
        {
            let mut c = Continuity::new();
            c.attach_store(EpisodeStore::open(path.to_str().unwrap()).unwrap());
            c.pause_world("destinE memo", "DRAFTING_CARE");
        }
        let mut c = Continuity::new();
        c.attach_store(EpisodeStore::open(path.to_str().unwrap()).unwrap());
        let snap = c.restore_world("destinE memo").expect("hydrated");
        assert_eq!(snap.current_step, "DRAFTING_CARE");
        let _ = std::fs::remove_file(&path);
    }
}
