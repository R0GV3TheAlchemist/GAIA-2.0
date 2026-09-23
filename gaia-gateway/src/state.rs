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

// `JoinHandle<T>` implements Default since Tokio 1.x (returns a handle that
// immediately resolves to Err(Cancelled)), so `#[derive(Default)]` is valid
// and equivalent to the previously manual impl.
#[derive(Default)]
pub struct Inner {
    /// Running agent IDs — used by the revoke endpoint.
    pub active_agents: std::collections::HashMap<String, tokio::task::JoinHandle<()>>,
}
