// Integration smoke-test: gateway starts and /health returns 204.
// Run with: cargo test -p gaia-gateway

#[tokio::test]
async fn health_returns_no_content() {
    use axum::http::StatusCode;
    use gaia_gateway::{router, state::AppState};
    use axum_test::TestServer;

    let app = router(AppState::default());
    let server = TestServer::new(app).unwrap();
    let resp = server.get("/health").await;
    assert_eq!(resp.status_code(), StatusCode::NO_CONTENT);
}
