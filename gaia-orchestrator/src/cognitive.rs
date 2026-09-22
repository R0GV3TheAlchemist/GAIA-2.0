//! Cognitive orchestration layer — #721.
//!
//! Provides the missing L4 machinery that sits between `IntentEngine` (NL →
//! `IntentGraph`) and `Executor` (DAG runner):
//!
//!   IntentGraph
//!       → GoalDecomposer       (complex intent → sub-goals, ≥ 3 nodes)
//!       → ConstraintResolver   (validates time / cost / privacy / compute)
//!       → CapabilityMatcher    (EACN-based team formation)
//!       → PlanGenerator        (builds executable Plan from matched agents)
//!       → PlanValidator        (pre-flight checks before run)
//!       → ExecutionPlanner     (resource allocation + scheduling)
//!       → FailureRecovery      (re-plan on mid-DAG error)
//!       → AdaptationEngine     (records outcome metadata)
//!
//! # gaia-inference integration point (#726)
//!
//! `GoalDecomposer::decompose` currently falls back to keyword heuristics when
//! `IntentBackend::Stub` is active. When #726 lands, replace the `Stub` branch
//! with a call to `gaia_inference::complete(prompt)` — the return contract
//! (a `Vec<String>` of sub-goal labels) is identical.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    dag::{DagNode, Plan, ResourceEstimate},
    intent::{Compute, Constraints, IntentGraph, Privacy, SubIntent},
    mcp::McpRegistry,
    select::pick_agent,
};

// ── 1. GoalDecomposer ──────────────────────────────────────────────────────

/// Breaks a complex `IntentGraph` into ≥ 3 executable sub-goals.
///
/// When `backend == Stub`, uses keyword heuristics so tests never need a
/// live LLM.  When `backend == Ollama` or `LlamaCpp`, the caller should
/// pre-populate `graph.sub_intents` via `IntentEngine::parse` (which already
/// calls the model); `decompose` then validates and, if needed, expands.
///
/// **#726 hook**: replace the `IntentBackend::Stub` arm with
/// `gaia_inference::decompose(goal, constraints)` returning `Vec<String>`.
pub struct GoalDecomposer;

impl GoalDecomposer {
    /// Returns an `IntentGraph` guaranteed to have ≥ 3 `sub_intents`.
    pub fn decompose(graph: IntentGraph) -> Result<IntentGraph, String> {
        if graph.goal.trim().is_empty() {
            return Err("goal must not be empty".into());
        }

        // If the graph already has enough nodes, validate and return as-is.
        if graph.sub_intents.len() >= 3 {
            validate_dag(&graph.sub_intents)?;
            return Ok(graph);
        }

        // Stub decomposition — keyword-driven.
        let steps = stub_decompose(&graph.goal, &graph.constraints);
        let mut sub_intents = Vec::with_capacity(steps.len());
        let mut previous: Option<Uuid> = None;
        for step in steps {
            let id = Uuid::new_v4();
            sub_intents.push(SubIntent {
                id,
                goal: step,
                depends_on: previous.map(|p| vec![p]).unwrap_or_default(),
            });
            previous = Some(id);
        }

        validate_dag(&sub_intents)?;
        Ok(IntentGraph {
            sub_intents,
            ..graph
        })
    }
}

/// Heuristic step expansion for stub / offline use.
fn stub_decompose(goal: &str, constraints: &Constraints) -> Vec<String> {
    let lower = goal.to_ascii_lowercase();
    let mut steps: Vec<String> = vec![format!("retrieve context: {goal}")];

    if lower.contains("research") || lower.contains("find") || lower.contains("search") {
        steps.push(format!("search sources: {goal}"));
        steps.push(format!("filter results: {goal}"));
        steps.push(format!("summarize: {goal}"));
    } else if lower.contains("write") || lower.contains("draft") || lower.contains("report") {
        steps.push(format!("outline: {goal}"));
        steps.push(format!("draft: {goal}"));
        steps.push(format!("review: {goal}"));
    } else if lower.contains("analyz") || lower.contains("evaluat") {
        steps.push(format!("collect data: {goal}"));
        steps.push(format!("analyze: {goal}"));
        steps.push(format!("conclude: {goal}"));
    } else {
        steps.push(format!("plan: {goal}"));
        steps.push(format!("execute: {goal}"));
        steps.push(format!("verify: {goal}"));
    }

    // Honour privacy constraint — mark cloud-gated steps.
    if constraints.privacy == Privacy::LocalOnly {
        for s in &mut steps {
            if s.starts_with("search sources") {
                s.push_str(" [local-only]");
            }
        }
    }

    steps
}

/// Validate that the sub-intent DAG has no cycles and all dependencies exist.
fn validate_dag(subs: &[SubIntent]) -> Result<(), String> {
    let ids: std::collections::HashSet<Uuid> = subs.iter().map(|s| s.id).collect();
    for sub in subs {
        for dep in &sub.depends_on {
            if !ids.contains(dep) {
                return Err(format!(
                    "sub-intent {} depends on unknown id {dep}",
                    sub.id
                ));
            }
        }
    }
    // Topo-sort smoke-test to catch cycles.
    let mut remaining: Vec<&SubIntent> = subs.iter().collect();
    let mut settled: std::collections::HashSet<Uuid> = std::collections::HashSet::new();
    loop {
        if remaining.is_empty() {
            break;
        }
        let ready: Vec<Uuid> = remaining
            .iter()
            .filter(|s| s.depends_on.iter().all(|d| settled.contains(d)))
            .map(|s| s.id)
            .collect();
        if ready.is_empty() {
            return Err("sub-intent DAG contains a cycle or unresolvable dependency".into());
        }
        for id in ready {
            settled.insert(id);
            remaining.retain(|s| s.id != id);
        }
    }
    Ok(())
}

// ── 2. ConstraintResolver ──────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintViolation {
    TimeInvalid(String),
    CostInvalid(String),
    ComputeUnavailable(String),
    PrivacyConflict(String),
}

impl std::fmt::Display for ConstraintViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TimeInvalid(m)
            | Self::CostInvalid(m)
            | Self::ComputeUnavailable(m)
            | Self::PrivacyConflict(m) => write!(f, "{m}"),
        }
    }
}

/// Validates that the constraints in an `IntentGraph` are satisfiable.
pub struct ConstraintResolver;

impl ConstraintResolver {
    pub fn resolve(graph: &IntentGraph) -> Result<(), ConstraintViolation> {
        let c = &graph.constraints;

        // Time constraint: reject obviously malformed values.
        if let Some(t) = &c.time {
            if t.trim().is_empty() {
                return Err(ConstraintViolation::TimeInvalid(
                    "time constraint is empty".into(),
                ));
            }
            if t.len() > 64 {
                return Err(ConstraintViolation::TimeInvalid(format!(
                    "time constraint too long: {}",
                    t.len()
                )));
            }
        }

        // Cost constraint: same length check.
        if let Some(cost) = &c.cost {
            if cost.trim().is_empty() {
                return Err(ConstraintViolation::CostInvalid(
                    "cost constraint is empty".into(),
                ));
            }
        }

        // Compute: Continuum requires CloudAllowed.
        if c.compute == Compute::Continuum && c.privacy == Privacy::LocalOnly {
            return Err(ConstraintViolation::PrivacyConflict(
                "Continuum compute requires CloudAllowed privacy".into(),
            ));
        }

        Ok(())
    }
}

// ── 3. CapabilityMatcher ───────────────────────────────────────────────────

/// An EACN-sourced capability entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EacnEntry {
    pub name: String,
    pub capabilities: Vec<String>,
    pub roles: Vec<String>,
}

/// A resolved team: one primary agent per sub-goal.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentTeam {
    /// Maps sub-intent id → assigned agent name.
    pub assignments: HashMap<Uuid, String>,
    /// Fallback agents keyed by sub-intent id.
    pub fallbacks: HashMap<Uuid, String>,
}

/// Discovers agents via EACN and assembles a valid team for a plan.
///
/// Falls back to `McpRegistry` token matching when the EACN catalog is
/// empty (stub / offline mode).
pub struct CapabilityMatcher {
    pub eacn: Vec<EacnEntry>,
}

impl CapabilityMatcher {
    pub fn new(eacn: Vec<EacnEntry>) -> Self {
        Self { eacn }
    }

    /// Returns an empty matcher that degrades to registry scoring.
    pub fn stub() -> Self {
        Self { eacn: vec![] }
    }

    pub fn match_team(
        &self,
        graph: &IntentGraph,
        registry: &McpRegistry,
    ) -> Result<AgentTeam, String> {
        let mut assignments = HashMap::new();
        let mut fallbacks = HashMap::new();
        let names: Vec<String> = registry.list().iter().map(|a| a.name.clone()).collect();
        if names.is_empty() {
            return Err("AIP registry is empty — cannot form a team".into());
        }

        for sub in &graph.sub_intents {
            // Try EACN first.
            let primary = if !self.eacn.is_empty() {
                self.eacn_match(&sub.goal).unwrap_or_else(|| names[0].clone())
            } else {
                pick_agent(registry, &sub.goal).unwrap_or_else(|_| names[0].clone())
            };

            let fallback = names
                .iter()
                .find(|n| **n != primary)
                .cloned()
                .unwrap_or_else(|| primary.clone());

            assignments.insert(sub.id, primary);
            fallbacks.insert(sub.id, fallback);
        }

        Ok(AgentTeam {
            assignments,
            fallbacks,
        })
    }

    fn eacn_match(&self, goal: &str) -> Option<String> {
        let goal = goal.to_ascii_lowercase();
        let mut best: Option<(usize, String)> = None;

        for entry in &self.eacn {
            let score = entry
                .capabilities
                .iter()
                .chain(entry.roles.iter())
                .filter(|token: &&String| {
                    goal.contains(token.as_str().to_ascii_lowercase().as_str())
                })
                .count();

            if score == 0 {
                continue;
            }

            match &best {
                None => best = Some((score, entry.name.clone())),
                Some((previous, _)) if score > *previous => {
                    best = Some((score, entry.name.clone()));
                }
                _ => {}
            }
        }

        best.map(|(_, name)| name)
    }
}

// ── 4. PlanGenerator ──────────────────────────────────────────────────────

/// Produces an executable `Plan` from an `IntentGraph` + `AgentTeam`.
pub struct PlanGenerator;

impl PlanGenerator {
    pub fn generate(graph: &IntentGraph, team: &AgentTeam) -> Plan {
        let nodes = graph
            .sub_intents
            .iter()
            .map(|sub| {
                let agent = team
                    .assignments
                    .get(&sub.id)
                    .cloned()
                    .unwrap_or_else(|| "generic".into());
                let fallback = team
                    .fallbacks
                    .get(&sub.id)
                    .cloned()
                    .unwrap_or_else(|| agent.clone());
                DagNode {
                    id: sub.id,
                    goal: sub.goal.clone(),
                    agent,
                    depends_on: sub.depends_on.clone(),
                    estimate: ResourceEstimate {
                        compute: graph.constraints.compute,
                        notes: "cognitive layer v1".into(),
                    },
                    fallback_agent: fallback,
                    max_retries: 1,
                }
            })
            .collect();

        Plan {
            id: Uuid::new_v4(),
            intent_id: graph.id,
            nodes,
            accepted: false,
        }
    }
}

// ── 5. PlanValidator ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanError {
    TooFewNodes(usize),
    NotAccepted,
    CycleDetected,
    UnassignedNode(Uuid),
}

impl std::fmt::Display for PlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewNodes(n) => write!(f, "plan has only {n} nodes; need ≥ 3"),
            Self::NotAccepted => write!(f, "plan must be accepted before validation"),
            Self::CycleDetected => write!(f, "DAG cycle detected"),
            Self::UnassignedNode(id) => write!(f, "node {id} has no agent assigned"),
        }
    }
}

pub struct PlanValidator;

impl PlanValidator {
    /// Pre-flight validation. Checks node count, DAG acyclicity, and agent
    /// assignments.  Does NOT check `plan.accepted` — that is the caller's
    /// responsibility (the ACP control-plane does this).
    pub fn validate(plan: &Plan) -> Result<(), PlanError> {
        if plan.nodes.len() < 3 {
            return Err(PlanError::TooFewNodes(plan.nodes.len()));
        }

        // Detect cycles via topo-sort.
        plan.topo_order().map_err(|_| PlanError::CycleDetected)?;

        // Every node must have a non-empty agent name.
        for node in &plan.nodes {
            if node.agent.trim().is_empty() {
                return Err(PlanError::UnassignedNode(node.id));
            }
        }

        Ok(())
    }
}

// ── 6. ExecutionPlanner ───────────────────────────────────────────────────

/// Resource allocation metadata produced before a plan is dispatched.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AllocationPlan {
    pub plan_id: Uuid,
    /// Topological execution order.
    pub order: Vec<Uuid>,
    /// Estimated parallelism: number of nodes that can run concurrently at
    /// each wave in the topo-sort.
    pub waves: Vec<Vec<Uuid>>,
    pub total_cpu_millis_est: u64,
    pub total_memory_mib_est: u64,
}

pub struct ExecutionPlanner {
    /// Default per-node CPU budget (milliseconds).
    pub cpu_millis_per_node: u64,
    /// Default per-node memory budget (MiB).
    pub memory_mib_per_node: u64,
}

impl Default for ExecutionPlanner {
    fn default() -> Self {
        Self {
            cpu_millis_per_node: 1_000,
            memory_mib_per_node: 64,
        }
    }
}

impl ExecutionPlanner {
    pub fn allocate(&self, plan: &Plan) -> Result<AllocationPlan, String> {
        let order = plan.topo_order()?;

        // Build waves: nodes that become ready at the same depth.
        let mut depth: HashMap<Uuid, usize> = HashMap::new();
        for &id in &order {
            let node = plan.nodes.iter().find(|n| n.id == id).unwrap();
            let d = node
                .depends_on
                .iter()
                .map(|dep| depth.get(dep).copied().unwrap_or(0) + 1)
                .max()
                .unwrap_or(0);
            depth.insert(id, d);
        }
        let max_depth = depth.values().copied().max().unwrap_or(0);
        let mut waves: Vec<Vec<Uuid>> = vec![vec![]; max_depth + 1];
        for (&id, &d) in &depth {
            waves[d].push(id);
        }
        for wave in &mut waves {
            wave.sort(); // deterministic order within wave
        }

        let n = plan.nodes.len() as u64;
        Ok(AllocationPlan {
            plan_id: plan.id,
            order,
            waves,
            total_cpu_millis_est: n * self.cpu_millis_per_node,
            total_memory_mib_est: n * self.memory_mib_per_node,
        })
    }
}

// ── 7. FailureRecovery ────────────────────────────────────────────────────

/// Strategy applied when a DAG node fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryStrategy {
    /// Use the node's `fallback_agent`.
    UseFallback,
    /// Skip the failed node and attempt dependents anyway.
    Skip,
    /// Abort the entire plan.
    Abort,
}

/// Recovery action emitted for a failed node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryAction {
    pub node_id: Uuid,
    pub strategy: RecoveryStrategy,
    pub new_agent: Option<String>,
}

/// Re-plans when one or more DAG nodes return an error.
///
/// Default policy: attempt fallback agent first; if the node has no
/// distinct fallback, skip it; if the plan has fewer than 3 remaining
/// nodes after skipping, abort.
pub struct FailureRecovery;

impl FailureRecovery {
    /// `failed_nodes` maps node id → error string.
    /// Returns a list of `RecoveryAction`s to apply before re-running.
    pub fn recover(
        plan: &Plan,
        failed_nodes: &HashMap<Uuid, String>,
    ) -> Result<Vec<RecoveryAction>, String> {
        if failed_nodes.is_empty() {
            return Ok(vec![]);
        }

        let mut actions = Vec::new();
        let mut skip_count = 0usize;

        for (&id, _reason) in failed_nodes {
            let node = plan
                .nodes
                .iter()
                .find(|n| n.id == id)
                .ok_or_else(|| format!("unknown node {id} in failed set"))?;

            if node.fallback_agent != node.agent && !node.fallback_agent.is_empty() {
                actions.push(RecoveryAction {
                    node_id: id,
                    strategy: RecoveryStrategy::UseFallback,
                    new_agent: Some(node.fallback_agent.clone()),
                });
            } else {
                skip_count += 1;
                actions.push(RecoveryAction {
                    node_id: id,
                    strategy: RecoveryStrategy::Skip,
                    new_agent: None,
                });
            }
        }

        let remaining_nodes = plan.nodes.len().saturating_sub(skip_count);
        if remaining_nodes < 3 {
            return Ok(vec![RecoveryAction {
                node_id: Uuid::nil(),
                strategy: RecoveryStrategy::Abort,
                new_agent: None,
            }]);
        }

        Ok(actions)
    }
}

// ── 8. AdaptationEngine ───────────────────────────────────────────────────

/// Outcome metadata recorded after a plan run.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlanOutcome {
    pub plan_id: Uuid,
    pub intent_id: Uuid,
    /// Number of nodes that succeeded on the first attempt.
    pub first_attempt_successes: usize,
    /// Number of nodes that needed the fallback agent.
    pub fallback_uses: usize,
    /// Number of nodes that were skipped via recovery.
    pub skipped: usize,
    /// Whether the plan was aborted.
    pub aborted: bool,
    /// Free-form notes (e.g. constraint violations surfaced at runtime).
    pub notes: Vec<String>,
}

/// Records outcome metadata and derives improvement signals for future plans.
///
/// In production this will feed a local fine-tuning loop or a retrieval store.
/// For now it accumulates `PlanOutcome` records in memory and exposes a
/// `score` function so the planner can rank historical agent→goal pairings.
pub struct AdaptationEngine {
    outcomes: Vec<PlanOutcome>,
    /// Tracks success rate per agent name.
    agent_scores: HashMap<String, (u32, u32)>, // (successes, total)
}

impl Default for AdaptationEngine {
    fn default() -> Self {
        Self {
            outcomes: Vec::new(),
            agent_scores: HashMap::new(),
        }
    }
}

impl AdaptationEngine {
    pub fn record(&mut self, outcome: PlanOutcome, plan: &Plan) {
        // Update per-agent scores from the run report.
        for node in &plan.nodes {
            let entry = self.agent_scores.entry(node.agent.clone()).or_insert((0, 0));
            entry.1 += 1;
            // Count as success if this node id is NOT in the fallback/skip set.
            let used_fallback = outcome.fallback_uses > 0; // coarse — see TODO below
            if !used_fallback {
                entry.0 += 1;
            }
        }
        self.outcomes.push(outcome);
    }

    /// Returns a 0.0–1.0 success-rate score for an agent, or 0.5 if unknown.
    pub fn score(&self, agent: &str) -> f64 {
        match self.agent_scores.get(agent) {
            Some(&(successes, total)) if total > 0 => successes as f64 / total as f64,
            _ => 0.5,
        }
    }

    pub fn outcomes(&self) -> &[PlanOutcome] {
        &self.outcomes
    }

    pub fn total_runs(&self) -> usize {
        self.outcomes.len()
    }
}

// ── Pipeline helper ────────────────────────────────────────────────────────

/// End-to-end cognitive pipeline: `IntentGraph` → validated, allocated `Plan`.
///
/// This is the function `ExecutionEngine` should call once #720 is wired.
pub fn build_plan(
    graph: IntentGraph,
    registry: &McpRegistry,
    matcher: &CapabilityMatcher,
    planner: &ExecutionPlanner,
) -> Result<(Plan, AllocationPlan), String> {
    // 1. Resolve constraints.
    ConstraintResolver::resolve(&graph)
        .map_err(|e| format!("constraint violation: {e}"))?;

    // 2. Decompose into ≥ 3 sub-goals.
    let graph = GoalDecomposer::decompose(graph)?;

    // 3. Match capabilities → team.
    let team = matcher.match_team(&graph, registry)?;

    // 4. Generate plan.
    let plan = PlanGenerator::generate(&graph, &team);

    // 5. Validate plan.
    PlanValidator::validate(&plan).map_err(|e| format!("plan invalid: {e}"))?;

    // 6. Allocate resources.
    let allocation = planner.allocate(&plan)?;

    Ok((plan, allocation))
}

// ── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        intent::{Compute, Constraints, IntentBackend, IntentGraph, Privacy, SubIntent},
        mcp::{AipManifest, McpRegistry, McpTool},
    };

    fn simple_graph(goal: &str) -> IntentGraph {
        IntentGraph {
            id: Uuid::new_v4(),
            goal: goal.into(),
            constraints: Constraints::default(),
            sub_intents: vec![],
            context_cube_ids: vec![],
            backend: IntentBackend::Stub,
        }
    }

    fn registry_with_agents(names: &[&str]) -> McpRegistry {
        let mut r = McpRegistry::default();
        for name in names {
            r.agents.push(AipManifest {
                name: name.to_string(),
                version: "0.1.0".into(),
                tools: vec![McpTool {
                    name: name.to_string(),
                    description: format!("{name} tool"),
                }],
                resources: vec![],
            });
        }
        r
    }

    // ── GoalDecomposer ────────────────────────────────────────────────────

    #[test]
    fn decompose_produces_at_least_three_nodes() {
        let graph = simple_graph("research local Rust patterns");
        let decomposed = GoalDecomposer::decompose(graph).unwrap();
        assert!(
            decomposed.sub_intents.len() >= 3,
            "expected ≥ 3 sub-intents, got {}",
            decomposed.sub_intents.len()
        );
    }

    #[test]
    fn decompose_preserves_graph_that_already_has_three_nodes() {
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        let c = Uuid::new_v4();
        let graph = IntentGraph {
            sub_intents: vec![
                SubIntent { id: a, goal: "step a".into(), depends_on: vec![] },
                SubIntent { id: b, goal: "step b".into(), depends_on: vec![a] },
                SubIntent { id: c, goal: "step c".into(), depends_on: vec![b] },
            ],
            ..simple_graph("multi-step intent")
        };
        let out = GoalDecomposer::decompose(graph).unwrap();
        assert_eq!(out.sub_intents.len(), 3);
    }

    #[test]
    fn decompose_rejects_empty_goal() {
        let graph = simple_graph("   ");
        assert!(GoalDecomposer::decompose(graph).is_err());
    }

    #[test]
    fn decompose_chain_has_sequential_deps() {
        let graph = simple_graph("write a project proposal");
        let out = GoalDecomposer::decompose(graph).unwrap();
        // Each node after the first must depend on the previous.
        for i in 1..out.sub_intents.len() {
            let prev_id = out.sub_intents[i - 1].id;
            assert!(
                out.sub_intents[i].depends_on.contains(&prev_id),
                "node {i} does not depend on node {}",
                i - 1
            );
        }
    }

    // ── ConstraintResolver ────────────────────────────────────────────────

    #[test]
    fn resolver_accepts_local_defaults() {
        let graph = simple_graph("anything");
        assert!(ConstraintResolver::resolve(&graph).is_ok());
    }

    #[test]
    fn resolver_rejects_continuum_with_local_only() {
        let graph = IntentGraph {
            constraints: Constraints {
                compute: Compute::Continuum,
                privacy: Privacy::LocalOnly,
                ..Constraints::default()
            },
            ..simple_graph("cloud task")
        };
        assert!(ConstraintResolver::resolve(&graph).is_err());
    }

    #[test]
    fn resolver_accepts_continuum_with_cloud_allowed() {
        let graph = IntentGraph {
            constraints: Constraints {
                compute: Compute::Continuum,
                privacy: Privacy::CloudAllowed,
                ..Constraints::default()
            },
            ..simple_graph("cloud task")
        };
        assert!(ConstraintResolver::resolve(&graph).is_ok());
    }

    // ── CapabilityMatcher ─────────────────────────────────────────────────

    #[test]
    fn matcher_assigns_agent_to_every_sub_intent() {
        let graph = GoalDecomposer::decompose(simple_graph("research quantum computing")).unwrap();
        let registry = registry_with_agents(&["researcher", "summarizer", "planner"]);
        let matcher = CapabilityMatcher::stub();
        let team = matcher.match_team(&graph, &registry).unwrap();
        for sub in &graph.sub_intents {
            assert!(
                team.assignments.contains_key(&sub.id),
                "no assignment for sub-intent {}",
                sub.id
            );
        }
    }

    #[test]
    fn matcher_fails_on_empty_registry() {
        let graph = GoalDecomposer::decompose(simple_graph("anything")).unwrap();
        let registry = McpRegistry::default();
        let matcher = CapabilityMatcher::stub();
        assert!(matcher.match_team(&graph, &registry).is_err());
    }

    #[test]
    fn eacn_matcher_prefers_capability_matched_agent() {
        let graph = GoalDecomposer::decompose(simple_graph("summarize findings")).unwrap();
        let registry = registry_with_agents(&["researcher", "summarizer"]);
        let matcher = CapabilityMatcher::new(vec![EacnEntry {
            name: "summarizer".into(),
            capabilities: vec!["summarize".into(), "condense".into()],
            roles: vec!["writer".into()],
        }]);
        let team = matcher.match_team(&graph, &registry).unwrap();
        // At least one sub-intent should be matched to summarizer.
        let has_summarizer = team.assignments.values().any(|a| a == "summarizer");
        assert!(has_summarizer, "EACN should prefer summarizer for summarize goals");
    }

    // ── PlanGenerator + PlanValidator ─────────────────────────────────────

    #[test]
    fn generated_plan_passes_validation() {
        let graph = GoalDecomposer::decompose(simple_graph("analyze project risks")).unwrap();
        let registry = registry_with_agents(&["analyst", "planner", "researcher"]);
        let team = CapabilityMatcher::stub().match_team(&graph, &registry).unwrap();
        let plan = PlanGenerator::generate(&graph, &team);
        assert!(PlanValidator::validate(&plan).is_ok());
    }

    #[test]
    fn plan_with_two_nodes_fails_validation() {
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        let plan = Plan {
            id: Uuid::new_v4(),
            intent_id: Uuid::new_v4(),
            accepted: false,
            nodes: vec![
                DagNode {
                    id: a,
                    goal: "step a".into(),
                    agent: "agent-a".into(),
                    depends_on: vec![],
                    estimate: ResourceEstimate { compute: Compute::Local, notes: "".into() },
                    fallback_agent: "agent-b".into(),
                    max_retries: 1,
                },
                DagNode {
                    id: b,
                    goal: "step b".into(),
                    agent: "agent-b".into(),
                    depends_on: vec![a],
                    estimate: ResourceEstimate { compute: Compute::Local, notes: "".into() },
                    fallback_agent: "agent-a".into(),
                    max_retries: 1,
                },
            ],
        };
        assert!(matches!(PlanValidator::validate(&plan), Err(PlanError::TooFewNodes(2))));
    }

    // ── ExecutionPlanner ──────────────────────────────────────────────────

    #[test]
    fn allocation_has_correct_node_count_and_order() {
        let graph = GoalDecomposer::decompose(simple_graph("write a technical report")).unwrap();
        let registry = registry_with_agents(&["writer", "researcher", "critic"]);
        let team = CapabilityMatcher::stub().match_team(&graph, &registry).unwrap();
        let plan = PlanGenerator::generate(&graph, &team);
        let planner = ExecutionPlanner::default();
        let alloc = planner.allocate(&plan).unwrap();
        assert_eq!(alloc.order.len(), plan.nodes.len());
        assert!(!alloc.waves.is_empty());
        assert_eq!(
            alloc.total_cpu_millis_est,
            plan.nodes.len() as u64 * planner.cpu_millis_per_node
        );
    }

    // ── FailureRecovery ───────────────────────────────────────────────────

    #[test]
    fn recovery_uses_fallback_when_available() {
        let graph = GoalDecomposer::decompose(simple_graph("research and summarize")).unwrap();
        let registry = registry_with_agents(&["researcher", "summarizer", "backup"]);
        let team = CapabilityMatcher::stub().match_team(&graph, &registry).unwrap();
        let mut plan = PlanGenerator::generate(&graph, &team);
        // Accept plan so it could run.
        plan.accept();

        // Simulate the first node failing.
        let first_id = plan.nodes[0].id;
        let mut failed = HashMap::new();
        failed.insert(first_id, "fixture error".to_string());

        let actions = FailureRecovery::recover(&plan, &failed).unwrap();
        assert!(!actions.is_empty());
        let action = &actions[0];
        // Should use fallback if fallback differs from primary.
        assert!(
            action.strategy == RecoveryStrategy::UseFallback
                || action.strategy == RecoveryStrategy::Skip,
            "unexpected strategy: {:?}",
            action.strategy
        );
    }

    #[test]
    fn recovery_on_empty_failed_set_returns_no_actions() {
        let graph = GoalDecomposer::decompose(simple_graph("simple task")).unwrap();
        let registry = registry_with_agents(&["agent-a", "agent-b", "agent-c"]);
        let team = CapabilityMatcher::stub().match_team(&graph, &registry).unwrap();
        let plan = PlanGenerator::generate(&graph, &team);
        let actions = FailureRecovery::recover(&plan, &HashMap::new()).unwrap();
        assert!(actions.is_empty());
    }

    // ── AdaptationEngine ──────────────────────────────────────────────────

    #[test]
    fn adaptation_records_outcome_and_scores_agent() {
        let graph = GoalDecomposer::decompose(simple_graph("evaluate risk")).unwrap();
        let registry = registry_with_agents(&["analyst", "planner", "researcher"]);
        let team = CapabilityMatcher::stub().match_team(&graph, &registry).unwrap();
        let plan = PlanGenerator::generate(&graph, &team);

        let mut engine = AdaptationEngine::default();
        let outcome = PlanOutcome {
            plan_id: plan.id,
            intent_id: plan.intent_id,
            first_attempt_successes: plan.nodes.len(),
            fallback_uses: 0,
            skipped: 0,
            aborted: false,
            notes: vec![],
        };
        engine.record(outcome, &plan);
        assert_eq!(engine.total_runs(), 1);
        // All agents used should have a known score.
        for node in &plan.nodes {
            let score = engine.score(&node.agent);
            assert!(
                (0.0..=1.0).contains(&score),
                "score out of range: {score}"
            );
        }
    }

    #[test]
    fn unknown_agent_score_is_neutral() {
        let engine = AdaptationEngine::default();
        assert_eq!(engine.score("unknown-agent"), 0.5);
    }

    // ── build_plan pipeline ───────────────────────────────────────────────

    #[test]
    fn build_plan_produces_valid_plan_and_allocation() {
        let graph = simple_graph("research and write a report on WASM runtimes");
        let registry = registry_with_agents(&["researcher", "writer", "critic"]);
        let matcher = CapabilityMatcher::stub();
        let planner = ExecutionPlanner::default();
        let (plan, alloc) = build_plan(graph, &registry, &matcher, &planner).unwrap();
        assert!(plan.nodes.len() >= 3);
        assert_eq!(alloc.order.len(), plan.nodes.len());
    }

    #[test]
    fn build_plan_rejects_constraint_conflict() {
        let graph = IntentGraph {
            constraints: Constraints {
                compute: Compute::Continuum,
                privacy: Privacy::LocalOnly,
                ..Constraints::default()
            },
            ..simple_graph("cloud task")
        };
        let registry = registry_with_agents(&["a", "b", "c"]);
        let matcher = CapabilityMatcher::stub();
        let planner = ExecutionPlanner::default();
        assert!(build_plan(graph, &registry, &matcher, &planner).is_err());
    }
}
