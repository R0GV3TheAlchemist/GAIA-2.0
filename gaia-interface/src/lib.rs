//! L6 sovereign interface (#6 / #27).
//! Local-only session with CLI and in-process HTTP adapters.
//! Not Ratatui, Axum, gRPC, voice, vision, mobile, or Studio.

mod gateway;
mod session;

pub use gateway::{HttpGateway, HttpRequest, HttpResponse};
pub use session::{
    AgentState, AuditLine, IntentRecord, MemoryNote, Profile, Session, SessionError,
};
