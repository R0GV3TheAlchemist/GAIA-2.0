//! Continuity substrate (Blueprint 59).
//! Capture flags default **false**. SQLite insert only after consent.
//! License: Apache-2.0

use std::collections::VecDeque;

use crate::episode_store::EpisodeStore;

/// Per-modality consent. All fields start false (Invariant 0.3).
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
    pub modality: &'static str,
    pub snapshot_id: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub id: String,
    pub label: String,
    pub current_step: String,
}

/// L2.5 Continuity daemon surface.
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

    /// Capture loop. Denied while every flag is false. Drops oldest if full.
    pub fn remember_life(&mut self, episode: Episode) -> Result<(), &'static str> {
        if !self.consent.any_on() {
            return Err("capture denied: all consent flags false");
        }
        if let Some(store) = &self.store {
            store
                .insert(episode.t_unix_ms as i64, &episode.text, episode.modality)
                .map_err(|_| "sqlite insert failed")?;
        }
        if self.episodes.len() >= self.max_ring {
            self.episodes.pop_front();
        }
        self.episodes.push_back(episode);
        Ok(())
    }

    pub fn ask_history(&self, _vague: &str) -> Vec<&Episode> {
        self.episodes.iter().rev().take(8).collect()
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
        let path = std::env::temp_dir().join("gaia_part4_denied.sqlite");
        let _ = std::fs::remove_file(&path);
        c.attach_store(EpisodeStore::open(path.to_str().unwrap()).unwrap());
        let err = c.remember_life(Episode {
            t_unix_ms: 0,
            text: "secret".into(),
            modality: "screen",
            snapshot_id: None,
        });
        assert!(err.is_err());
        assert!(c.ask_history("anything").is_empty());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn consented_remember_writes_sqlite() {
        let mut c = Continuity::new();
        c.consent.files = true;
        let path = std::env::temp_dir().join("gaia_part4_ok.sqlite");
        let _ = std::fs::remove_file(&path);
        c.attach_store(EpisodeStore::open(path.to_str().unwrap()).unwrap());
        c.remember_life(Episode {
            t_unix_ms: 42,
            text: "open CARE.md".into(),
            modality: "files",
            snapshot_id: None,
        })
        .unwrap();
        assert_eq!(c.ask_history("CARE")[0].text, "open CARE.md");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn pause_restore_uses_step_not_chat() {
        let mut c = Continuity::new();
        let id = c.pause_world("destinE memo", "DRAFTING_CARE");
        let snap = c.restore_world("destinE memo").expect("snapshot");
        assert_eq!(snap.id, id);
        assert_eq!(snap.current_step, "DRAFTING_CARE");
    }
}
