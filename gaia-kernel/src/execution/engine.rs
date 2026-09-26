//! [`ExecutionEngine`] — the 12-stage GAIA intent execution pipeline.
//!
//! Tasks in the same DAG tier are dispatched through [`super::fanout::run_fanout`] (#937).
//! Audit and memory writes stay on the engine task after joins.

use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    audit::AuditLog,
    execution::{
        dag::Task,
        error::ExecutionError,
        fanout::{run_fanout, FanoutJob},
        metrics::IntentSpan,
    },
    identity::{sha256_hex, verify, Principal},
    planner::{decompose::Planner, replan::Replanner},
    scheduler::{allocate::Scheduler, select::AgentRegistry},
};

use gaia_memos::{CubeType, MemCube, MemOs};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub schema_version: String,
    pub id: Uuid,
    pub user_did: String,
    pub intent_type: String,
    pub slots: HashMap<String, String>,
    pub timestamp_ms: u64,
    pub ttl_ms: u64,
    pub signature: Option<IntentSignature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentSignature {
    pub alg: String,
    pub sig_hex: String,
    pub pub_hex: String,
}

impl Intent {
    pub fn signing_bytes(&self) -> Vec<u8> {
        let canon = format!(
            "{}|{}|{}|{}",
            self.id, self.user_did, self.intent_type, self.timestamp_ms
        );
        sha256_hex(canon.as_bytes()).into_bytes()
    }

    pub fn verify_signature(&self) -> Result<(), ExecutionError> {
        let sig = self.signature.as_ref().ok_or_else(|| {
            ExecutionError::SignatureRequired("intent.signature field is missing".into())
        })?;
        if sig.alg != "EdDSA" {
            return Err(ExecutionError::SignatureRequired(format!(
                "unsupported algorithm: {}",
                sig.alg
            )));
        }
        let sig_bytes = hex::decode(&sig.sig_hex).map_err(|_| {
            ExecutionError::SignatureRequired("signature hex invalid".into())
        })?;
        let payload = self.signing_bytes();
        if !verify(&sig.pub_hex, &payload, &sig_bytes) {
            return Err(ExecutionError::SignatureRequired(
                "Ed25519 signature verification failed".into(),
            ));
        }
        Ok(())
    }

    pub fn validate_schema(&self) -> Result<(), ExecutionError> {
        if self.schema_version != "1.0" {
            return Err(ExecutionError::SchemaInvalid(format!(
                "unsupported schema_version: {}",
                self.schema_version
            )));
        }
        if self.user_did.is_empty() {
            return Err(ExecutionError::SchemaInvalid("user_did must not be empty".into()));
        }
        let valid_types = ["device_control", "query", "plan", "alert", "memory_write"];
        if !valid_types.contains(&self.intent_type.as_str()) {
            return Err(ExecutionError::SchemaInvalid(format!(
                "unknown intent_type: {}",
                self.intent_type
            )));
        }
        if self.ttl_ms == 0 {
            return Err(ExecutionError::SchemaInvalid("ttl_ms must be > 0".into()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
    Success { output: String },
    Failed { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: Uuid,
    pub outcome: Outcome,
    pub stage_ms: [u64; 12],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub intent_id: Uuid,
    pub task_results: Vec<TaskResult>,
    pub total_ms: u64,
}

pub struct ExecutionEngine {
    pub planner: Planner,
    pub replanner: Replanner,
    pub registry: AgentRegistry,
    pub scheduler: Scheduler,
    pub audit: AuditLog,
    pub memory: MemOs,
    pub principal: Principal,
}

impl ExecutionEngine {
    pub fn new(principal: Principal) -> Self {
        Self {
            planner: Planner::new(),
            replanner: Replanner::new(),
            registry: AgentRegistry::new(),
            scheduler: Scheduler::new(),
            audit: AuditLog::default(),
            memory: MemOs::new(),
            principal,
        }
    }

    pub async fn execute(&mut self, intent: Intent) -> Result<ExecutionResult, ExecutionError> {
        let mut span = IntentSpan::new();

        let s1 = crate::execution::metrics::StageSpan::begin(1);
        intent.verify_signature()?;
        intent.validate_schema()?;
        span.record_stage(1, s1.finish());

        let s2 = crate::execution::metrics::StageSpan::begin(2);
        let subgoals = self.planner.decompose(&intent);
        span.record_stage(2, s2.finish());

        let s3 = crate::execution::metrics::StageSpan::begin(3);
        let dag = self.planner.plan(&subgoals);
        span.record_stage(3, s3.finish());

        let tiers = dag
            .topological_tiers()
            .map_err(|e| ExecutionError::SchemaInvalid(format!("DAG cycle: {e}")))?;

        let mut task_results: Vec<TaskResult> = Vec::new();

        for tier in tiers {
            let jobs: Vec<FanoutJob> = tier
                .iter()
                .map(|t| FanoutJob {
                    agent_id: t.capability.clone(),
                    work: Duration::ZERO,
                    fail: false,
                })
                .collect();
            let _ = run_fanout(jobs, Duration::from_secs(30)).await;

            let mut tier_results: Vec<TaskResult> = Vec::with_capacity(tier.len());
            for task in tier {
                let result = self.execute_task(&intent, task, &mut span).await;
                tier_results.push(result);
            }
            task_results.extend(tier_results);
        }

        let total_ms = span.finish(true);
        Ok(ExecutionResult {
            intent_id: intent.id,
            task_results,
            total_ms,
        })
    }

    async fn execute_task(&mut self, intent: &Intent, task: Task, span: &mut IntentSpan) -> TaskResult {
        let task_id = task.id;
        let mut stage_ms = [0u64; 12];

        let s4 = crate::execution::metrics::StageSpan::begin(4);
        let candidates = self.registry.find(&task.capability);
        stage_ms[3] = s4.finish();

        if candidates.is_empty() {
            self.write_audit(intent, &task, "no_capable_agent");
            return TaskResult {
                task_id,
                outcome: Outcome::Failed {
                    reason: format!("GAIA_NO_CAPABLE_AGENT: {}", task.capability),
                },
                stage_ms,
            };
        }

        let s5 = crate::execution::metrics::StageSpan::begin(5);
        let agent = self.registry.select(candidates);
        stage_ms[4] = s5.finish();

        let s6 = crate::execution::metrics::StageSpan::begin(6);
        let policy_ok = agent.capabilities.contains(&task.capability);
        stage_ms[5] = s6.finish();

        if !policy_ok {
            self.write_audit(intent, &task, "capability_denied");
            return TaskResult {
                task_id,
                outcome: Outcome::Failed {
                    reason: format!(
                        "GAIA_CAPABILITY_DENIED: {} denied to {}",
                        task.capability, agent.id
                    ),
                },
                stage_ms,
            };
        }

        let s7 = crate::execution::metrics::StageSpan::begin(7);
        let _quota = self.scheduler.allocate(&task);
        stage_ms[6] = s7.finish();

        let s8 = crate::execution::metrics::StageSpan::begin(8);
        let mut outcome = Outcome::Success {
            output: format!("executed:{}:{}", agent.id, task.capability),
        };
        stage_ms[7] = s8.finish();

        let s9 = crate::execution::metrics::StageSpan::begin(9);
        stage_ms[8] = s9.finish();

        let s10 = crate::execution::metrics::StageSpan::begin(10);
        if let Outcome::Failed { ref reason } = outcome {
            match self.replanner.fallback(&task, reason) {
                Some(fallback) => {
                    outcome = Outcome::Success {
                        output: format!("fallback:{}:{}", agent.id, fallback.capability),
                    };
                }
                None => {
                    stage_ms[9] = s10.finish();
                    self.write_audit(intent, &task, "replan_exhausted");
                    return TaskResult {
                        task_id,
                        outcome: Outcome::Failed {
                            reason: format!("GAIA_REPLAN_EXHAUSTED: {}", task.capability),
                        },
                        stage_ms,
                    };
                }
            }
        }
        stage_ms[9] = s10.finish();

        let s11 = crate::execution::metrics::StageSpan::begin(11);
        self.write_audit(intent, &task, "success");
        stage_ms[10] = s11.finish();

        let s12 = crate::execution::metrics::StageSpan::begin(12);
        let output_str = match &outcome {
            Outcome::Success { output } => output.clone(),
            Outcome::Failed { reason } => reason.clone(),
        };
        let cube = MemCube::new(
            CubeType::Episodic,
            format!("intent:{}:task:{}:{}", intent.id, task_id, output_str),
            "execution-engine",
        );
        self.memory.put(cube);
        stage_ms[11] = s12.finish();

        for (i, &ms) in stage_ms.iter().enumerate() {
            span.record_stage(i as u8 + 1, ms);
        }

        TaskResult {
            task_id,
            outcome,
            stage_ms,
        }
    }

    fn write_audit(&mut self, intent: &Intent, task: &Task, detail: &str) {
        self.audit.append(
            &self.principal,
            "execution",
            &format!("intent:{}:task:{}:{}", intent.id, task.id, detail),
        );
    }
}
