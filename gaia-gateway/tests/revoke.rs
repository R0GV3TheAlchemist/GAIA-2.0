// Revoke an unknown agent returns 404.

#[tokio::test]
async fn revoke_unknown_agent_returns_404() {
    use axum::http::StatusCode;
    use gaia_gateway::{router, state::AppState};
    use axum_test::TestServer;

    let app = router(AppState::default());
    let server = TestServer::new(app).unwrap();
    let resp = server.delete("/agents/nonexistent/revoke").await;
    assert_eq!(resp.status_code(), StatusCode::NOT_FOUND);
}
