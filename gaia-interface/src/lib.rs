//! L6 sovereign interface (#6 / #27 / #28 / #29).
//! Local session, permission console, orchestrator gateway, local sense surface.
//! Not Ratatui, Axum, React, Vite, gRPC, Whisper, LLaVA, or Flutter.

mod console;
mod gateway;
mod orch;
mod sense;
mod session;

pub use console::{AgentRow, PermissionConsole, Studio, StudioRecipe, TraceEvent};
pub use gateway::{HttpGateway, HttpRequest, HttpResponse};
pub use orch::{ForwardedIntent, OrchestratorGateway};
pub use sense::{
    LocalAsr, LocalCaption, LocalVision, MobileClient, SenseError, SensePolicy, SenseSurface,
};
pub use session::{
    AgentState, AuditLine, IntentRecord, MemoryNote, Permission, Profile, Session, SessionError,
};
