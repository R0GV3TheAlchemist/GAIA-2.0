pub mod routes;
pub mod state;

use axum::{
    Router,
    routing::{delete, get, post},
};
use state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        // Intent
        .route("/intent", post(routes::intent::submit_intent))
        .route("/intent/stream", get(routes::intent::stream_intent))
        // Agents — Axum 0.8 captures use `{id}`, not `:id`.
        .route("/agents", post(routes::agent::create_agent))
        .route("/agents/{id}/deploy", post(routes::agent::deploy_agent))
        .route("/agents/{id}/revoke", delete(routes::agent::revoke_agent))
        // Memory
        .route("/memory", get(routes::memory::list_memory))
        .route("/memory/search", get(routes::memory::search_memory))
        // Audit
        .route("/audit", get(routes::audit::get_audit))
        .route("/audit/stream", get(routes::audit::stream_audit))
        // Health
        .route("/health", get(routes::health::health))
        .with_state(state)
}
