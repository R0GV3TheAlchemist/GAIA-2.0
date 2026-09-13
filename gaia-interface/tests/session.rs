use gaia_interface::{AgentState, HttpGateway, HttpRequest, Session, SessionError};

fn booted() -> Session {
    let mut session = Session::new();
    session.init("developer").unwrap();
    session.start().unwrap();
    session
}

#[test]
fn developer_profile_is_local_only() {
    let mut session = Session::new();
    let profile = session.init("developer").unwrap();
    assert!(profile.local_only);
    assert!(!profile.cloud_opt_in);
    session.start().unwrap();
    assert!(session.started());
}

#[test]
fn start_before_init_is_refused() {
    let err = Session::new().start().unwrap_err();
    assert_eq!(err, SessionError::NotInitialized);
}

#[test]
fn cli_and_http_share_the_same_intent() {
    let mut session = booted();
    let via_cli = session.exec(&["intent", "research and summarize CARE"]).unwrap();
    assert!(via_cli.contains("intent_id=1"));
    let via_http = HttpGateway::new(&mut session).handle(HttpRequest {
        method: "POST".into(),
        path: "/intent".into(),
        body: r#"{"text":"research and summarize CARE"}"#.into(),
    });
    assert_eq!(via_http.status, 200);
    assert!(via_http.body.contains("\"id\":2"));
    assert_eq!(session.intents().len(), 2);
    assert_eq!(session.intents()[0].events[0].kind, "admitted");
    assert_eq!(session.intents()[1].events[0].kind, "admitted");
}

#[test]
fn revoke_stops_agent_immediately() {
    let mut session = booted();
    session.exec(&["intent", "research"]).unwrap();
    let out = session.exec(&["revoke", "local-researcher"]).unwrap();
    assert_eq!(out, "revoked local-researcher");
    assert_eq!(session.agents()[0].state, AgentState::Revoked);
    assert!(session.intents()[0].cancelled);
    let err = session.exec(&["intent", "research again"]).unwrap_err();
    assert_eq!(err, SessionError::Usage("no running agent".into()));
}

#[test]
fn http_revoke_matches_cli() {
    let mut session = booted();
    let response = HttpGateway::new(&mut session).handle(HttpRequest {
        method: "POST".into(),
        path: "/agents/local-researcher/revoke".into(),
        body: "{}".into(),
    });
    assert_eq!(response.status, 200);
    assert_eq!(session.agents()[0].state, AgentState::Revoked);
}

#[test]
fn cloud_intent_is_denied_without_opt_in() {
    let mut session = booted();
    let err = session.declare_intent("run this in cloud").unwrap_err();
    assert_eq!(err, SessionError::CloudDenied);
}

#[test]
fn agent_create_then_deploy_works_on_cli_and_http() {
    let mut session = booted();
    let created = session.exec(&["agent", "create", "writer"]).unwrap();
    assert_eq!(created, "created agent writer state=Created");
    let deployed = session.exec(&["agent", "deploy", "writer"]).unwrap();
    assert_eq!(deployed, "deployed agent writer state=Running");
    let via_http = HttpGateway::new(&mut session).handle(HttpRequest {
        method: "POST".into(),
        path: "/agents".into(),
        body: r#"{"name":"analyst"}"#.into(),
    });
    assert_eq!(via_http.status, 200);
    assert!(via_http.body.contains("Created"));
    let deploy_http = HttpGateway::new(&mut session).handle(HttpRequest {
        method: "POST".into(),
        path: "/agents/analyst/deploy".into(),
        body: "{}".into(),
    });
    assert_eq!(deploy_http.status, 200);
    assert!(deploy_http.body.contains("Running"));
}

#[test]
fn memory_and_audit_are_session_local() {
    let mut session = booted();
    session.exec(&["memory", "CARE note"]).unwrap();
    assert_eq!(session.memory().len(), 1);
    assert_eq!(session.memory()[0].text, "CARE note");
    let listed = session.exec(&["audit"]).unwrap();
    assert!(listed.starts_with("audit_events="));
    assert!(session.audit().iter().any(|l| l.event.contains("memory")));
    let http = HttpGateway::new(&mut session).handle(HttpRequest {
        method: "GET".into(),
        path: "/audit".into(),
        body: String::new(),
    });
    assert_eq!(http.status, 200);
    assert!(http.body.contains("init profile=developer"));
}
