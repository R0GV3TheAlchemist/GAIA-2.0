use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{
        IntoResponse, Response,
        sse::{Event, Sse},
    },
};
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct IntentRequest {
    pub text: String,
    pub profile: Option<String>,
}

#[derive(Serialize)]
pub struct IntentResponse {
    pub id: String,
    pub status: String,
}

/// POST /intent — fire-and-forget, returns an intent ID.
pub async fn submit_intent(
    State(_state): State<AppState>,
    Json(req): Json<IntentRequest>,
) -> impl IntoResponse {
    let id = uuid::Uuid::new_v4().to_string();
    tracing::info!(intent = %req.text, %id, "intent received");
    // TODO: forward to gaia-orchestrator
    (StatusCode::ACCEPTED, Json(IntentResponse { id, status: "queued".into() }))
}

/// GET /intent/stream?text=… — streams result chunks as SSE.
pub async fn stream_intent(
    State(_state): State<AppState>,
) -> Sse<impl futures::Stream<Item = Result<Event, std::convert::Infallible>>> {
    // TODO: connect to orchestrator stream; placeholder emits one stub event.
    let s = stream::once(async {
        Ok(Event::default().data("[stub] intent stream not yet wired to orchestrator"))
    });
    Sse::new(s)
}
