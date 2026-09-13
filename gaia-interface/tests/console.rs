//! #28: watch an intent without a terminal, list/revoke agents, document a11y.

use gaia_interface::{AgentState, PermissionConsole, Session, Studio};
use std::fs;

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
fn every_running_agent_is_visible_and_revocable() {
    let mut session = Session::new();
    session.init("developer").unwrap();
    session.start().unwrap();
    let studio = Studio::default();
    let rows = PermissionConsole::new(&session, &studio).agents();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].id, "local-researcher");
    assert!(rows[0].revocable);
    session.revoke("local-researcher").unwrap();
    let after = PermissionConsole::new(&session, &studio).agents();
    assert_eq!(after[0].state, "Revoked");
    assert!(!after[0].revocable);
    assert_eq!(session.agents()[0].state, AgentState::Revoked);
    let html = PermissionConsole::new(&session, &studio).render_html();
    assert!(html.contains("aria-label=\"Revoke") || html.contains("not revocable"));
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
    let notes = fs::read_to_string("ACCESSIBILITY.md").unwrap();
    assert!(notes.contains("keyboard"));
    assert!(notes.contains("contrast"));
    assert!(notes.contains("screen reader"));
}
