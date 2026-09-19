use gaia_gaian::{gaian_release_checklist, Agent, AgentAct, GaianError};

#[test]
fn agent_cannot_email_pay_or_publish_without_grant() {
    let mut agent = Agent::new();
    assert_eq!(
        agent.act(AgentAct::Email).unwrap_err(),
        GaianError::GrantRequired
    );
    assert_eq!(
        agent.act(AgentAct::Pay).unwrap_err(),
        GaianError::GrantRequired
    );
    assert_eq!(
        agent.act(AgentAct::PublishLikeness).unwrap_err(),
        GaianError::GrantRequired
    );
    agent.act(AgentAct::Ask).unwrap();
    agent.grant.email = true;
    agent.act(AgentAct::Email).unwrap();
    agent.revoke();
    assert_eq!(agent.act(AgentAct::Email).unwrap_err(), GaianError::Revoked);
    assert_eq!(agent.act(AgentAct::Ask).unwrap_err(), GaianError::Revoked);
}

#[test]
fn checklist_refuses_v1_and_names_child_safety() {
    let list = gaian_release_checklist();
    assert!(list.contains(&"child-safety tests"));
    assert!(list.contains(&"non-impersonation tests"));
    assert!(list.contains(&"no GAIAN v1.0 tag"));
}
