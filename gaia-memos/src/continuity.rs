//! Continuity substrate (Blueprint 59) — Build Part 1.
//! Capture flags default **false**. No logger until consent is explicit.
//! License: Apache-2.0

use std::collections::VecDeque;

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
#[derive(Debug)]
pub struct Continuity {
    pub consent: CaptureConsent,
    episodes: VecDeque<Episode>,
    snapshots: Vec<Snapshot>,
    max_ring: usize,
}

impl Continuity {
    pub fn new() -> Self {
        Self {
            consent: CaptureConsent::all_off(),
            episodes: VecDeque::new(),
            snapshots: Vec::new(),
            max_ring: 256,
        }
    }

    /// Capture loop. No-ops while every flag is false. Drops oldest if full.
    pub fn remember_life(&mut self, episode: Episode) -> Result<(), &'static str> {
        if !self.consent.any_on() {
            return Err("capture denied: all consent flags false");
        }
        if self.episodes.len() >= self.max_ring {
            self.episodes.pop_front();
        }
        self.episodes.push_back(episode);
        Ok(())
    }

    /// Vague query over episodes. Empty until something was consented and stored.
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

    /// Hydrate by label/id. Does not replay chat. Returns stored current_step.
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
    fn capture_defaults_off() {
        let mut c = Continuity::new();
        assert!(!c.consent.any_on());
        let err = c.remember_life(Episode {
            t_unix_ms: 0,
            text: "secret".into(),
            modality: "screen",
            snapshot_id: None,
        });
        assert!(err.is_err());
        assert!(c.ask_history("anything").is_empty());
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
