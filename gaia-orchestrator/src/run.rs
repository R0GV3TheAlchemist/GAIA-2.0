use crate::{Broker, Plan, TrustAudit};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalRun {
    pub plan_id: Uuid,
    pub completed_jobs: Vec<String>,
    pub failed_over_jobs: Vec<String>,
}

/// Local in-process runner. It has no network, model, or cryptographic capability.
pub struct LocalRunner;

impl LocalRunner {
    /// Runs only an explicitly accepted plan. `kill_after_first_pull` simulates executor death.
    pub fn run(
        plan: &Plan,
        broker: &mut Broker,
        audit: &mut TrustAudit,
        kill_after_first_pull: Option<&str>,
    ) -> Result<LocalRun, String> {
        if !plan.accepted {
            return Err("plan must be inspected and accepted before local run".into());
        }
        broker.enqueue_plan(plan);
        let mut completed_jobs = Vec::new();
        let mut failed_over_jobs = Vec::new();
        let mut killed = false;

        while !broker.queue.is_empty() || broker.workers.iter().any(|w| w.inflight.is_some()) {
            let worker_id = broker
                .live_specialists()
                .first()
                .map(|w| w.id.clone())
                .ok_or_else(|| "no live specialist for queued work".to_string())?;
            let Some(job) = broker.pull(&worker_id)? else { break };
            audit.append(plan.intent_id, Some(plan.id), Some(&worker_id), format!("node-started:{job}"));

            if !killed && kill_after_first_pull == Some(worker_id.as_str()) {
                broker.kill(&worker_id);
                audit.append(plan.intent_id, Some(plan.id), Some(&worker_id), format!("node-failed-over:{job}"));
                failed_over_jobs.push(job);
                killed = true;
                continue;
            }

            broker.complete(&worker_id);
            audit.append(plan.intent_id, Some(plan.id), Some(&worker_id), format!("node-completed:{job}"));
            completed_jobs.push(job);
        }

        if completed_jobs.len() != plan.nodes.len() {
            return Err("not all DAG nodes completed".into());
        }
        Ok(LocalRun {
            plan_id: plan.id,
            completed_jobs,
            failed_over_jobs,
        })
    }
}
