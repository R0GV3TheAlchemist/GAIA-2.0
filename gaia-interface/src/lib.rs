//! L6 sovereign interface (#6 / #27 / #28).
//! Local session, permission console, orchestrator gateway.
//! Not Ratatui, Axum, React, Vite, gRPC, voice, vision, or mobile.

mod console;
mod gateway;
mod orch;
mod session;

pub use console::{AgentRow, PermissionConsole, Studio, StudioRecipe, TraceEvent};
pub use gateway::{HttpGateway, HttpRequest, HttpResponse};
pub use orch::{ForwardedIntent, OrchestratorGateway};
pub use session::{
    AgentState, AuditLine, IntentRecord, MemoryNote, Permission, Profile, Session, SessionError,
};
