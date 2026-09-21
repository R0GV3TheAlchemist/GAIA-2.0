use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
}

pub async fn list_memory(
    State(_state): State<AppState>,
) -> impl IntoResponse {
    // TODO: proxy to gaia-memos
    Json(serde_json::json!({ "cubes": [] }))
}

pub async fn search_memory(
    State(_state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> impl IntoResponse {
    tracing::info!(query = ?params.q, "memory search");
    // TODO: proxy to gaia-memos
    Json(serde_json::json!({ "results": [] }))
}
