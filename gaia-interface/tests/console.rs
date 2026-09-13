//! #28: watch an intent without a terminal, pause/revoke, permission matrix.

use gaia_interface::{AgentState, HttpGateway, HttpRequest, PermissionConsole, Session, Studio};
use std::fs;
use std::path::PathBuf;

#[test]
fn html_trace_shows_intent_events_without_a_terminal() {
    let mut session = Session::new();
    session.init("developer").unwrap();
    session.start().unwrap();
    session.declare_intent("research CARE").unwrap();
    let studio = Studio::default();
    let html = PermissionConsole::new(&session, &studio).render_html();
    assert!(html.contains("<html lang=\"en\">"));
    assert!(html.contains("intent 1"));
    assert!(html.contains("admitted"));
    assert!(html.contains("streamed"));
    assert!(html.contains("aria-live"));
}

#[test]
fn every_running_agent_is_visible_pausable_and_revocable() {
    let mut session = Session::new();
    session.init("developer").unwrap();
    session.start().unwrap();
    let studio = Studio::default();
    let rows = PermissionConsole::new(&session, &studio).agents();
    assert_eq!(rows[0].id, "local-researcher");
    assert!(rows[0].pausable);
    assert!(rows[0].revocable);
    session.pause("local-researcher").unwrap();
    assert_eq!(session.agents()[0].state, AgentState::Paused);
    assert!(session.declare_intent("research").is_err());
    session.resume("local-researcher").unwrap();
    session.revoke("local-researcher").unwrap();
    let after = PermissionConsole::new(&session, &studio).agents();
    assert_eq!(after[0].state, "Revoked");
    assert!(!after[0].revocable);
}

#[test]
fn permission_matrix_is_inspectable() {
    let mut session = Session::new();
    session.init("developer").unwrap();
    session.start().unwrap();
    let studio = Studio::default();
    let console = PermissionConsole::new(&session, &studio);
    let matrix = console.matrix();
    assert!(matrix.iter().any(|p| p.capability == "MemoryRead" && p.granted));
    assert!(matrix.iter().any(|p| p.capability == "Network" && !p.granted));
    let html = console.render_html();
    assert!(html.contains("Permission matrix"));
    assert!(html.contains("denied"));
}

#[test]
fn http_pause_matches_cli() {
    let mut session = Session::new();
    session.init("developer").unwrap();
    session.start().unwrap();
    let response = HttpGateway::new(&mut session).handle(HttpRequest {
        method: "POST".into(),
        path: "/agents/local-researcher/pause".into(),
        body: "{}".into(),
    });
    assert_eq!(response.status, 200);
    assert_eq!(session.agents()[0].state, AgentState::Paused);
}

#[test]
fn studio_composes_a_manifest_recipe() {
    let mut studio = Studio::default();
    let recipe = studio.compose("writer", "write.draft").unwrap();
    assert_eq!(recipe.name, "writer");
    assert!(studio.compose("writer", "write.draft").is_err());
}

#[test]
fn wcag_notes_are_documented() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ACCESSIBILITY.md");
    let notes = fs::read_to_string(path).unwrap().to_ascii_lowercase();
    assert!(notes.contains("keyboard"));
    assert!(notes.contains("contrast"));
    assert!(notes.contains("screen reader"));
}
