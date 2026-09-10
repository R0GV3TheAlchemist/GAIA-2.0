//! Continuity substrate (Blueprint 59). Forget/correct honor Invariant 0.8.
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

    pub fn forget(&mut self, text: &str) -> Result<usize, &'static str> {
        self.episodes.retain(|e| e.text != text);
        if let Some(store) = &self.store {
            return store.forget(text).map_err(|_| "sqlite forget failed");
        }
        Ok(0)
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
    fn forget_hides_from_ask_history() {
        let path = std::env::temp_dir().join("gaia_forget_c.sqlite");
        let _ = std::fs::remove_file(&path);
        let mut c = Continuity::new();
        c.consent.files = true;
        c.attach_store(EpisodeStore::open(path.to_str().unwrap()).unwrap());
        c.remember_life(Episode {
            t_unix_ms: 1,
            text: "I like jazz".into(),
            modality: "type".into(),
            snapshot_id: None,
        })
        .unwrap();
        c.forget("I like jazz").unwrap();
        assert!(c.ask_history("jazz").is_empty());
        let _ = std::fs::remove_file(&path);
    }
}
