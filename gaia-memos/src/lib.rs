//! GAIA 2.0 Memory OS crate (Blueprint 58 MemOS + Blueprint 59 Continuity).
//! License: Apache-2.0

pub mod continuity;
pub mod episode_store;

pub use continuity::{CaptureConsent, Continuity, Episode, Snapshot};
pub use episode_store::EpisodeStore;
