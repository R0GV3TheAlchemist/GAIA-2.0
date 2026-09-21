use axum::{
    Json,
    extract::State,
    response::{
        IntoResponse,
        sse::{Event, Sse},
    },
};
use futures::stream::{self, StreamExt};
use crate::state::AppState;

pub async fn get_audit(
    State(_state): State<AppState>,
) -> impl IntoResponse {
    // TODO: query audit store
    Json(serde_json::json!({ "entries": [] }))
}

pub async fn stream_audit(
    State(_state): State<AppState>,
) -> Sse<impl futures::Stream<Item = Result<Event, std::convert::Infallible>>> {
    // TODO: tail live audit log
    let s = stream::once(async {
        Ok(Event::default().data("[stub] audit stream not yet connected"))
    });
    Sse::new(s)
}
