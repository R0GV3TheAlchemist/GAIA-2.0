use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared application state threaded through all Axum handlers.
/// Extend this as the orchestrator client and agent registry are wired in.
#[derive(Clone, Default)]
pub struct AppState {
    /// TODO: replace with a real OrchestratorClient once gaia-orchestrator
    ///       exposes a stable async API.
    pub _inner: Arc<RwLock<Inner>>,
}

pub struct Inner {
    /// Running agent IDs — used by the revoke endpoint.
    pub active_agents: std::collections::HashMap<String, tokio::task::JoinHandle<()>>,
}

// JoinHandle<T> does not implement Default, so we cannot #[derive(Default)]
// on Inner.  Provide an explicit impl that initialises the map to empty.
impl Default for Inner {
    fn default() -> Self {
        Self {
            active_agents: std::collections::HashMap::new(),
        }
    }
}
