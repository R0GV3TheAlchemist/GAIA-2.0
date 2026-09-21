use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
    pub manifest: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct AgentResponse {
    pub id: String,
    pub name: String,
    pub status: String,
}

pub async fn create_agent(
    State(_state): State<AppState>,
    Json(req): Json<CreateAgentRequest>,
) -> impl IntoResponse {
    let id = uuid::Uuid::new_v4().to_string();
    tracing::info!(agent = %req.name, %id, "agent created");
    // TODO: register with orchestrator
    (StatusCode::CREATED, Json(AgentResponse { id, name: req.name, status: "created".into() }))
}

pub async fn deploy_agent(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    tracing::info!(%id, "agent deploy requested");
    // TODO: instruct orchestrator to start agent
    (StatusCode::ACCEPTED, Json(AgentResponse { id: id.clone(), name: id, status: "deploying".into() }))
}

/// DELETE /agents/{id}/revoke — immediately cancels the agent.
pub async fn revoke_agent(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut inner = state._inner.write().await;
    if let Some(handle) = inner.active_agents.remove(&id) {
        handle.abort();
        tracing::info!(%id, "agent revoked");
        StatusCode::NO_CONTENT
    } else {
        tracing::warn!(%id, "revoke: agent not found");
        StatusCode::NOT_FOUND
    }
}
