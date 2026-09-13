//! Bind DAG nodes to registered AIP manifests. Not a marketplace.

use crate::dag::Plan;
use crate::mcp::McpRegistry;

pub fn pick_agent(registry: &McpRegistry, goal: &str) -> Result<String, String> {
    let g = goal.to_ascii_lowercase();
    let mut scored: Vec<(usize, String)> = Vec::new();
    for agent in registry.list() {
        let mut blob = agent.name.to_ascii_lowercase();
        blob.push(' ');
        for tool in &agent.tools {
            blob.push_str(&tool.name.to_ascii_lowercase());
            blob.push(' ');
            blob.push_str(&tool.description.to_ascii_lowercase());
            blob.push(' ');
        }
        let mut score = 0;
        for token in g.split(|c: char| !c.is_ascii_alphanumeric()) {
            if token.len() < 4 {
                continue;
            }
            if blob.contains(token) {
                score += 1;
            }
        }
        if score > 0 {
            scored.push((score, agent.name.clone()));
        }
    }
    scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    scored
        .into_iter()
        .next()
        .map(|(_, name)| name)
        .ok_or_else(|| format!("no AIP agent registered for goal: {goal}"))
}

pub fn bind_plan_to_registry(plan: &mut Plan, registry: &McpRegistry) -> Result<(), String> {
    if registry.list().is_empty() {
        return Err("AIP registry is empty".into());
    }
    let names: Vec<String> = registry.list().iter().map(|a| a.name.clone()).collect();
    for node in &mut plan.nodes {
        node.agent = pick_agent(registry, &node.goal)?;
        node.fallback_agent = names
            .iter()
            .find(|n| *n != &node.agent)
            .cloned()
            .unwrap_or_else(|| node.agent.clone());
    }
    Ok(())
}
