//! L6 sovereign interface (#6 / #27 / #28 / #29).
//! Local session, permission console, orchestrator gateway, local sense surface.
//! Not Ratatui, Axum, React, Vite, gRPC, Whisper, LLaVA, or Flutter.

mod agentic;
mod console;
mod gateway;
mod orch;
mod packs;
mod plane;
mod sense;
mod session;

pub use agentic::{
    guard_embodiment_off, render_tool_event, render_tool_event_html, render_tool_result,
    render_world_model_claim, render_world_model_claim_html, sandbox_status,
    try_computer_use, try_computer_use_host_escape,
};
pub use console::{AgentRow, PermissionConsole, Studio, StudioRecipe, TraceEvent};
pub use gateway::{HttpGateway, HttpRequest, HttpResponse};
pub use orch::{ForwardedIntent, OrchestratorGateway};
pub use packs::{
    guard_no_exam_cert, render_professional_pack, render_professional_pack_html,
    render_science_card, render_science_card_html, try_insurer_automation,
};
pub use plane::{
    Developer, COMMISSION_BPS, UPGRADE_PRICE_UNITS, UPGRADE_THRESHOLD_UNITS,
};
pub use sense::{
    LocalAsr, LocalCaption, LocalVision, MobileClient, SenseError, SensePolicy, SenseSurface,
};
pub use session::{
    AgentState, AuditLine, IntentRecord, MemoryNote, Permission, Profile, Session, SessionError,
};
