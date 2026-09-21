use anyhow::Result;
use gaia_gateway::{router, state::AppState};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let state = AppState::default();
    let app = router(state);
    let addr = "0.0.0.0:7700";
    info!("gaia-gateway listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
