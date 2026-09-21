//! DELETE /agents/:id/revoke for an unknown agent → 404

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use gaia_gateway::{router, state::AppState};
use tower::ServiceExt;

#[tokio::test]
async fn revoke_unknown_agent_returns_404() {
    let app = router(AppState::default());
    let req = Request::builder()
        .method("DELETE")
        .uri("/agents/nonexistent/revoke")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
