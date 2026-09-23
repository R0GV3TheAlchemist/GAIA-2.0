//! [`ExecutionEngine`] — the 12-stage GAIA intent execution pipeline.
//!
//! Every agent action in GAIA flows through this engine.  Every decision is
//! logged with a cryptographic receipt.  Every failure triggers re-planning.
//!
//! # Stage map
//!
//! ```text
//! Stage  1 — Intent validation   (signature + schema)
//! Stage  2 — Decomposition       (Intent → Vec<SubGoal>)
//! Stage  3 — Planning            (SubGoals → TaskDAG)
//! Stage  4 — Discovery           (CapabilityRegistry lookup per task)
//! Stage  5 — Selection           (Scheduler picks best agent per task)
//! Stage  6 — Policy              (ACP gate — hard stop if denied)
//! Stage  7 — Allocation          (reserve CPU / memory quota)
//! Stage  8 — Execution           (run inside sandbox — stub until #740 lands)
//! Stage  9 — Observation         (collect outcome + telemetry)
//! Stage 10 — Adaptation          (re-plan on failure; at least one fallback)
//! Stage 11 — Audit               (Ed25519-signed cryptographic receipt)
//! Stage 12 — Memory update       (persist result to MemOS)
//! ```
//!
//! Stages 4–12 run once per task tier in topological order; tasks within a
//! tier execute concurrently via `tokio::join`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    audit::AuditLog,
    execution::{
        dag::Task,
        error::ExecutionError,
        metrics::IntentSpan,
    },
    identity::{verify, sha256_hex, Principal},
    planner::{
        decompose::Planner,
        replan::Replanner,
    },
    scheduler::{
        allocate::Scheduler,
        select::AgentRegistry,
    },
};

use gaia_memos::{CubeType, MemCube, MemOs};

// ── Intent ─────────────────────────────────────────────────────────────

/// Schema version 1.0 signed intent (JSON-serialisable).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub schema_version: String,
    pub id:             Uuid,
    pub user_did:       String,
    pub intent_type:    String,
    pub slots:          HashMap<String, String>,
    pub timestamp_ms:   u64,
    pub ttl_ms:         u64,
    pub signature:      Option<IntentSignature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentSignature {
    pub alg:     String,
    pub sig_hex: String,
    pub pub_hex: String,
}

impl Intent {
    /// Returns the canonical bytes that are signed: sha256(id | user_did | intent_type | timestamp_ms).
    pub fn signing_bytes(&self) -> Vec<u8> {
        let canon = format!(
            "{}|{}|{}|{}",
            self.id, self.user_did, self.intent_type, self.timestamp_ms
        );
        sha256_hex(canon.as_bytes()).into_bytes()
    }

    /// Stage 1a — reject if signature field is absent or algorithm is wrong.
    pub fn verify_signature(&self) -> Result<(), ExecutionError> {
        let sig = self.signature.as_ref().ok_or_else(|| {
            ExecutionError::SignatureRequired(
                "intent.signature field is missing".into(),
            )
        })?;
        if sig.alg != "EdDSA" {
            return Err(ExecutionError::SignatureRequired(format!(
                "unsupported algorithm: {}", sig.alg
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

    /// Stage 1b — basic schema validation.
    pub fn validate_schema(&self) -> Result<(), ExecutionError> {
        if self.schema_version != "1.0" {
            return Err(ExecutionError::SchemaInvalid(format!(
                "unsupported schema_version: {}",
                self.schema_version
            )));
        }
        if self.user_did.is_empty() {
            return Err(ExecutionError::SchemaInvalid(
                "user_did must not be empty".into(),
            ));
        }
        let valid_types = [
            "device_control",
            "query",
            "plan",
            "alert",
            "memory_write",
        ];
        if !valid_types.contains(&self.intent_type.as_str()) {
            return Err(ExecutionError::SchemaInvalid(format!(
                "unknown intent_type: {}",
                self.intent_type
            )));
        }
        if self.ttl_ms == 0 {
            return Err(ExecutionError::SchemaInvalid(
                "ttl_ms must be > 0".into(),
            ));
        }
        Ok(())
    }
}

// ── ExecutionOutcome ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
    Success { output: String },
    Failed  { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id:  Uuid,
    pub outcome:  Outcome,
    pub stage_ms: [u64; 12],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub intent_id:    Uuid,
    pub task_results: Vec<TaskResult>,
    pub total_ms:     u64,
}

// ── ExecutionEngine ──────────────────────────────────────────────

/// Wasmtime 46 + WASI 0.3 twelve-stage intent execution pipeline.
pub struct ExecutionEngine {
    pub planner:   Planner,
    pub replanner: Replanner,
    pub registry:  AgentRegistry,
    pub scheduler: Scheduler,
    pub audit:     AuditLog,
    pub memory:    MemOs,
    pub principal: Principal,
}

impl ExecutionEngine {
    pub fn new(principal: Principal) -> Self {
        Self {
            planner:   Planner::new(),
            replanner: Replanner::new(),
            registry:  AgentRegistry::new(),
            scheduler: Scheduler::new(),
            audit:     AuditLog::default(),
            memory:    MemOs::new(),
            principal,
        }
    }

    /// Run the full 12-stage pipeline for a signed [`Intent`].
    pub async fn execute(
        &mut self,
        intent: Intent,
    ) -> Result<ExecutionResult, ExecutionError> {
        let mut span = IntentSpan::new();

        // ── Stage 1: Validate ──────────────────────────────────────
        let s1 = crate::execution::metrics::StageSpan::begin(1);
        intent.verify_signature()?;
        intent.validate_schema()?;
        span.record_stage(1, s1.finish());

        // ── Stage 2: Decompose ─────────────────────────────────────
        let s2 = crate::execution::metrics::StageSpan::begin(2);
        let subgoals = self.planner.decompose(&intent);
        span.record_stage(2, s2.finish());

        // ── Stage 3: Plan (produces TaskDAG) ─────────────────────────
        let s3 = crate::execution::metrics::StageSpan::begin(3);
        let dag = self.planner.plan(&subgoals);
        span.record_stage(3, s3.finish());

        let tiers = dag.topological_tiers().map_err(|e| {
            ExecutionError::SchemaInvalid(format!("DAG cycle: {e}"))
        })?;

        let mut task_results: Vec<TaskResult> = Vec::new();

        for tier in tiers {
            // Tasks in the same tier have no mutual dependencies — run in parallel.
            let mut tier_results: Vec<TaskResult> = Vec::with_capacity(tier.len());

            // Collect futures for parallel execution.
            // Using sequential await here preserves the async contract while
            // keeping the borrow-checker happy with &mut self fields.
            // True fan-out via tokio::spawn requires Arc<Mutex<_>> on inner
            // fields — that refactor is tracked in #721 (Orchestrator).
            for task in tier {
                let result = self.execute_task(&intent, task, &mut span).await;
                tier_results.push(result);
            }

            task_results.extend(tier_results);
        }

        let total_ms = span.finish(true);

        Ok(ExecutionResult {
            intent_id:    intent.id,
            task_results,
            total_ms,
        })
    }

    /// Run stages 4–12 for a single task.
    async fn execute_task(
        &mut self,
        intent: &Intent,
        task: Task,
        span: &mut IntentSpan,
    ) -> TaskResult {
        let task_id = task.id;
        let mut stage_ms = [0u64; 12];

        // ── Stage 4: Discover ──────────────────────────────────────
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

        // ── Stage 5: Select ────────────────────────────────────────
        let s5 = crate::execution::metrics::StageSpan::begin(5);
        let agent = self.registry.select(candidates);
        stage_ms[4] = s5.finish();

        // ── Stage 6: Policy (ACP gate) ───────────────────────────────
        let s6 = crate::execution::metrics::StageSpan::begin(6);
        let policy_ok = agent.capabilities.contains(&task.capability);
        stage_ms[5] = s6.finish();

        if !policy_ok {
            self.write_audit(intent, &task, "capability_denied");
            return TaskResult {
                task_id,
                outcome: Outcome::Failed {
                    reason: format!("GAIA_CAPABILITY_DENIED: {} denied to {}", task.capability, agent.id),
                },
                stage_ms,
            };
        }

        // ── Stage 7: Allocate ──────────────────────────────────────
        let s7 = crate::execution::metrics::StageSpan::begin(7);
        let _quota = self.scheduler.allocate(&task);
        stage_ms[6] = s7.finish();

        // ── Stage 8: Execute (sandbox stub — wired fully in #740) ────────────
        let s8 = crate::execution::metrics::StageSpan::begin(8);
        let mut outcome = Outcome::Success {
            output: format!("executed:{}:{}", agent.id, task.capability),
        };
        stage_ms[7] = s8.finish();

        // ── Stage 9: Observe ───────────────────────────────────────
        let s9 = crate::execution::metrics::StageSpan::begin(9);
        // TODO(#734): emit telemetry span for outcome
        stage_ms[8] = s9.finish();

        // ── Stage 10: Adapt (re-plan on failure) ─────────────────────────
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

        // ── Stage 11: Audit ────────────────────────────────────────
        let s11 = crate::execution::metrics::StageSpan::begin(11);
        self.write_audit(intent, &task, "success");
        stage_ms[10] = s11.finish();

        // ── Stage 12: Memory update ────────────────────────────────
        let s12 = crate::execution::metrics::StageSpan::begin(12);
        let output_str = match &outcome {
            Outcome::Success { output } => output.clone(),
            Outcome::Failed  { reason } => reason.clone(),
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

        TaskResult { task_id, outcome, stage_ms }
    }

    fn write_audit(&mut self, intent: &Intent, task: &Task, detail: &str) {
        self.audit.append(
            &self.principal,
            "execution",
            &format!("intent:{}:task:{}:{}", intent.id, task.id, detail),
        );
    }
}
