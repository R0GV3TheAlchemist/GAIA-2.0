//! L6 sovereign interface (#6 / #27).
//! Local-only session, in-process HTTP adapter, orchestrator gateway.
//! Not Ratatui, Axum, gRPC, voice, vision, mobile, or Studio.

mod gateway;
mod orch;
mod session;

pub use gateway::{HttpGateway, HttpRequest, HttpResponse};
pub use orch::{ForwardedIntent, OrchestratorGateway};
pub use session::{AgentState, IntentRecord, Profile, Session, SessionError};
