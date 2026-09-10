//! Continuity substrate (Blueprint 59).
//! Capture flags default **false**. SQLite insert only after consent.
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

    /// Vague query. Uses FTS5 when a store is attached; else the ring.
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
        self.snapshots.push(Snapshot {
            id: id.clone(),
            label: label.to_string(),
            current_step: current_step.to_string(),
        });
        id
    }

    pub fn restore_world(&self, prompt: &str) -> Option<&Snapshot> {
        self.snapshots.iter().rev().find(|s| {
            s.id == prompt || s.label == prompt || prompt.contains(&s.label)
        })
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
    fn capture_defaults_off_skips_sqlite() {
        let mut c = Continuity::new();
        let path = std::env::temp_dir().join("gaia_part5_denied.sqlite");
        let _ = std::fs::remove_file(&path);
        c.attach_store(EpisodeStore::open(path.to_str().unwrap()).unwrap());
        let err = c.remember_life(Episode {
            t_unix_ms: 0,
            text: "secret".into(),
            modality: "screen".into(),
            snapshot_id: None,
        });
        assert!(err.is_err());
        assert!(c.ask_history("anything").is_empty());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn ask_history_uses_fts() {
        let mut c = Continuity::new();
        c.consent.files = true;
        let path = std::env::temp_dir().join("gaia_part5_fts.sqlite");
        let _ = std::fs::remove_file(&path);
        c.attach_store(EpisodeStore::open(path.to_str().unwrap()).unwrap());
        c.remember_life(Episode {
            t_unix_ms: 42,
            text: "open CARE.md DestinE draft".into(),
            modality: "files".into(),
            snapshot_id: None,
        })
        .unwrap();
        let hits = c.ask_history("CARE");
        assert!(hits[0].text.contains("CARE"));
        let _ = std::fs::remove_file(&path);
    }
}
