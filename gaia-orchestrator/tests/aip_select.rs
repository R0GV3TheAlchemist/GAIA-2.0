use gaia_memos::MemOs;
use gaia_orchestrator::{
    bind_plan_to_registry, pick_agent, IntentEngine, McpRegistry, TaskPlanner,
};

#[test]
fn research_goal_selects_registered_researcher() {
    let reg = McpRegistry::local();
    let name = pick_agent(&reg, "research and summarize CARE").unwrap();
    assert_eq!(name, "gaia-local-researcher");
}

#[test]
fn unknown_goal_fails_closed() {
    let reg = McpRegistry::local();
    let err = pick_agent(&reg, "xyzzy-unmapped-task").unwrap_err();
    assert!(err.contains("no AIP agent"));
}

#[test]
fn bind_replaces_stub_role_labels() {
    let mut mem = MemOs::new();
    let g = IntentEngine::local_stub()
        .parse("research and summarize CARE", &mut mem)
        .unwrap();
    let mut plan = TaskPlanner::from_intent(&g);
    let reg = McpRegistry::local();
    bind_plan_to_registry(&mut plan, &reg).unwrap();
    assert!(plan
        .nodes
        .iter()
        .all(|n| n.agent == "gaia-local-researcher"));
    assert!(plan.inspect().contains("gaia-local-researcher"));
}
