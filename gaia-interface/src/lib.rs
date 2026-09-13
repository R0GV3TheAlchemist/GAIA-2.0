//! L6 sovereign interface (#6 / #27 / #28).
//! Local-only session, in-process HTTP adapter, permission console HTML.
//! Not Ratatui, Axum, React, Vite, gRPC, voice, vision, mobile, or Studio-as-IDE.

mod console;
mod gateway;
mod session;

pub use console::{AgentRow, PermissionConsole, Studio, StudioRecipe, TraceEvent};
pub use gateway::{HttpGateway, HttpRequest, HttpResponse};
pub use session::{AgentState, IntentRecord, Profile, Session, SessionError};
