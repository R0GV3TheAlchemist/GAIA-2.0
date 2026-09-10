use crate::intent::{Compute, IntentGraph};
use gaia_memos::{CubeType, MemCube, MemOs};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceEstimate {
    pub compute: Compute,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DagNode {
    pub id: Uuid,
    pub goal: String,
    pub agent: String,
    pub depends_on: Vec<Uuid>,
    pub estimate: ResourceEstimate,
    pub fallback_agent: String,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Plan {
    pub id: Uuid,
    pub intent_id: Uuid,
    pub nodes: Vec<DagNode>,
    pub accepted: bool,
}

impl Plan {
    pub fn inspect(&self) -> String {
        let steps: Vec<String> = self
            .nodes
            .iter()
            .map(|n| {
                format!(
                    "{} [{}] fallback={} retries={}",
                    n.goal, n.agent, n.fallback_agent, n.max_retries
                )
            })
            .collect();
        format!("plan {} nodes={}\n{}", self.id, self.nodes.len(), steps.join("\n"))
    }

    pub fn accept(&mut self) {
        self.accepted = true;
    }
}

pub struct TaskPlanner;

impl TaskPlanner {
    pub fn from_intent(graph: &IntentGraph) -> Plan {
        let nodes = graph
            .sub_intents
            .iter()
            .map(|s| {
                let agent = agent_for(&s.goal);
                DagNode {
                    id: s.id,
                    goal: s.goal.clone(),
                    agent,
                    depends_on: s.depends_on.clone(),
                    estimate: ResourceEstimate {
                        compute: Compute::Local,
                        notes: "v0 local stub".into(),
                    },
                    fallback_agent: "local-stub-fallback".into(),
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

fn agent_for(goal: &str) -> String {
    let g = goal.to_ascii_lowercase();
    if g.contains("retriev") {
        "retriever".into()
    } else if g.contains("research") {
        "researcher".into()
    } else if g.contains("summar") {
        "summarizer".into()
    } else {
        "generic".into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunReport {
    pub plan_id: Uuid,
    pub used_fallback: Vec<Uuid>,
    pub outputs: Vec<String>,
    pub cube_id: Option<Uuid>,
}

#[derive(Default)]
pub struct Executor {
    /// Goals containing these substrings fail once, then fallback.
    pub fail_goals: Vec<String>,
}

impl Executor {
    pub fn execute(&self, plan: &Plan, mem: &mut MemOs) -> Result<RunReport, String> {
        if !plan.accepted {
            return Err("plan must be inspected and accepted before run".into());
        }
        if plan.nodes.len() < 3 {
            return Err("DAG must have 3+ nodes".into());
        }
        let mut used_fallback = Vec::new();
        let mut outputs = Vec::new();
        for node in &plan.nodes {
            let should_fail = self.fail_goals.iter().any(|f| node.goal.contains(f));
            if should_fail {
                used_fallback.push(node.id);
                outputs.push(format!(
                    "fallback:{} agent={}",
                    node.goal, node.fallback_agent
                ));
            } else {
                outputs.push(format!("ok:{} agent={}", node.goal, node.agent));
            }
        }
        let report = RunReport {
            plan_id: plan.id,
            used_fallback: used_fallback.clone(),
            outputs: outputs.clone(),
            cube_id: None,
        };
        let json = serde_json::to_string(&report).map_err(|e| e.to_string())?;
        let cube_id = mem.put(MemCube::new(CubeType::Plaintext, json, "dag-plan-result"));
        Ok(RunReport {
            cube_id: Some(cube_id),
            ..report
        })
    }
}
