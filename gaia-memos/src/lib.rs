//! gaia-memos — episodic and working memory for GAIA agents.
//!
//! ## Batch B additions
//!
//! `retrieval_guard` now exposes [`MemosQuery`], [`MemoCandidate`], and
//! [`AuthorizedMemosResult`] for authorization-filtered memo retrieval
//! using `gaia-ingest::auth::RetrievalFilter`.

pub mod continuity;
pub mod decay;
pub mod episode_store;
pub mod erasure;
pub mod isolation;
pub mod persist;
pub mod retrieval_guard;
pub mod screenpipe;
pub mod sync;

pub use continuity::{
    apply_continuity_patch, continuity_delta, ContinuityPatch, EpisodeSummary,
};
pub use decay::{
    apply_decay, decay_score, salience_after_sleep, AccessRecord, DecayConfig,
};
pub use episode_store::{
    append_episode, recent_episodes, EpisodeRecord, EpisodeStore,
};
pub use erasure::{erase_episode, redact_field, ErasureReceipt};
pub use isolation::{agent_namespace, IsolationKey, MemoryPartition};
pub use persist::{load_snapshot, save_snapshot, MemorySnapshot};
pub use retrieval_guard::{AuthorizedMemosResult, MemoCandidate, MemosQuery};
pub use screenpipe::{
    ingest_screenpipe_frame, ScreenpipeFrame, ScreenpipeIngestResult,
};
pub use sync::{merge_snapshots, sync_needed, SyncConflict};
