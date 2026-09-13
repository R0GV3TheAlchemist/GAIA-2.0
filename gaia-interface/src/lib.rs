//! L6 sovereign interface (#6 / #27 / #28).
//! Local-only session, permission console HTML, pause/revoke matrix.
//! Not Ratatui, Axum, React, Vite, gRPC, voice, vision, or mobile.

mod console;
mod gateway;
mod session;

pub use console::{AgentRow, PermissionConsole, Studio, StudioRecipe, TraceEvent};
pub use gateway::{HttpGateway, HttpRequest, HttpResponse};
pub use session::{AgentState, IntentRecord, Permission, Profile, Session, SessionError};
