//! #27 gateway in front of the orchestrator. No Axum socket.

use gaia_interface::{HttpGateway, HttpRequest, OrchestratorGateway};

#[test]
fn developer_boot_then_cli_and_http_share_orchestrator_intent() {
    let mut gw = OrchestratorGateway::local();
    gw.exec(&["init", "--profile=developer"]).unwrap();
    gw.exec(&["start"]).unwrap();
    let via_cli = gw.exec(&["intent", "research and summarize CARE"]).unwrap();
    assert!(via_cli.contains("intent_id=1"));
    assert!(via_cli.contains("stored=true"));
    assert!(via_cli.contains("graph="));

    let via_http = HttpGateway::new(gw.session_mut()).handle(HttpRequest {
        method: "POST".into(),
        path: "/intent".into(),
        body: r#"{"text":"research and summarize CARE"}"#.into(),
    });
    assert_eq!(via_http.status, 200);
    assert!(via_http.body.contains("\"id\":2"));
}

#[test]
fn intent_stream_is_the_recorded_events() {
    let mut gw = OrchestratorGateway::local();
    gw.boot_developer().unwrap();
    let forwarded = gw.forward_intent("research CARE").unwrap();
    let events = gw.stream(forwarded.session.id).unwrap();
    assert_eq!(events[0].kind, "admitted");
    assert_eq!(events[1].kind, "streamed");
    assert_eq!(events[2].kind, "completed");
    assert!(forwarded.stored);
}

#[test]
fn revoke_still_stops_the_agent() {
    let mut gw = OrchestratorGateway::local();
    gw.boot_developer().unwrap();
    gw.forward_intent("research").unwrap();
    let out = gw.exec(&["revoke", "local-researcher"]).unwrap();
    assert_eq!(out, "revoked local-researcher");
    assert!(gw.forward_intent("again").is_err());
}
