//! #25 core pack: deploy system + cognitive, critic reviews locally.
//! Not a marketplace and not the #26 A2A protocol.

use gaia_agents::{catalog, AgentHost, AgentKind};

#[test]
fn pack_lists_system_and_cognitive_agents() {
    let names: Vec<&str> = catalog().iter().map(|e| e.name).collect();
    for required in [
        "memory-manager",
        "resource-optimizer",
        "security-monitor",
        "update-manager",
        "researcher",
        "writer",
        "coder",
        "analyst",
        "planner",
        "critic",
        "mcp-bridge",
    ] {
        assert!(names.contains(&required), "missing {required}");
    }
    assert!(catalog().iter().any(|e| e.kind == AgentKind::System));
    assert!(catalog().iter().any(|e| e.kind == AgentKind::Cognitive));
}

#[test]
fn gaia_agent_deploy_works_for_system_and_cognitive() {
    let mut host = AgentHost::new();
    let system = host.exec(&["agent", "deploy", "memory-manager"]).unwrap();
    assert_eq!(system, "deployed system agent memory-manager intent=memcube.consolidate");
    let cognitive = host.exec(&["agent", "deploy", "researcher"]).unwrap();
    assert_eq!(cognitive, "deployed cognitive agent researcher intent=research.question");
    assert_eq!(host.deployed().len(), 2);
}

#[test]
fn unknown_agent_is_refused() {
    let mut host = AgentHost::new();
    assert!(host.exec(&["agent", "deploy", "not-a-pack-agent"]).is_err());
}

#[test]
fn critic_reviews_another_agents_output() {
    let mut host = AgentHost::new();
    host.deploy("researcher").unwrap();
    host.deploy("critic").unwrap();
    let output = host.invoke("researcher", "CARE").unwrap();
    let review = host.review("critic", "researcher", &output).unwrap();
    assert_eq!(review.critic, "critic");
    assert_eq!(review.subject_agent, "researcher");
    assert!(review.verdict.contains("researcher"));
    let cli = host
        .exec(&["agent", "review", "researcher", "draft notes"])
        .unwrap();
    assert!(cli.contains("critic=critic"));
    assert!(cli.contains("subject=researcher"));
}

#[test]
fn critic_cannot_review_before_subject_is_deployed() {
    let mut host = AgentHost::new();
    host.deploy("critic").unwrap();
    assert!(host.review("critic", "researcher", "draft").is_err());
}

#[test]
fn bridge_deploy_does_not_call_an_external_api() {
    let mut host = AgentHost::new();
    host.deploy("mcp-bridge").unwrap();
    let err = host.invoke("mcp-bridge", "search").unwrap_err();
    assert!(err.to_string().contains("not wired"));
}
