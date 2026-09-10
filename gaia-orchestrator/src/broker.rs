use crate::dag::{DagNode, Plan};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Worker {
    pub id: String,
    pub role: String,
    pub alive: bool,
    pub inflight: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Metrics {
    pub queue_depth: usize,
    pub executor_health: Vec<(String, bool)>,
}

/// Pull-based resource broker. Nodes poll; no inbound ports.
pub struct Broker {
    pub workers: Vec<Worker>,
    pub queue: Vec<String>,
    /// v0 ColonyOS hook: static timetable. false = carbon window closed.
    pub carbon_ok: bool,
}

impl Broker {
    pub fn new() -> Self {
        Self {
            workers: vec![
                Worker {
                    id: "manager".into(),
                    role: "manager".into(),
                    alive: true,
                    inflight: None,
                },
                Worker {
                    id: "specialist-a".into(),
                    role: "specialist".into(),
                    alive: true,
                    inflight: None,
                },
                Worker {
                    id: "specialist-b".into(),
                    role: "specialist".into(),
                    alive: true,
                    inflight: None,
                },
            ],
            queue: Vec::new(),
            carbon_ok: true,
        }
    }

    pub fn requires_inbound_ports(&self) -> bool {
        false
    }

    pub fn enqueue_plan(&mut self, plan: &Plan) {
        for n in &plan.nodes {
            self.queue.push(n.id.to_string());
        }
    }

    /// Worker pulls next node id. Manager only delegates; specialists execute.
    pub fn pull(&mut self, worker_id: &str) -> Result<Option<String>, String> {
        if !self.carbon_ok {
            return Err("carbon window closed".into());
        }
        let w = self
            .workers
            .iter_mut()
            .find(|w| w.id == worker_id)
            .ok_or_else(|| "unknown worker".to_string())?;
        if !w.alive {
            return Err("worker dead".into());
        }
        if w.role == "manager" {
            return Ok(None);
        }
        if let Some(job) = self.queue.pop() {
            w.inflight = Some(job.clone());
            Ok(Some(job))
        } else {
            Ok(None)
        }
    }

    pub fn complete(&mut self, worker_id: &str) {
        if let Some(w) = self.workers.iter_mut().find(|w| w.id == worker_id) {
            w.inflight = None;
        }
    }

    /// Killing an executor requeues in-flight work onto the shared pull queue.
    pub fn kill(&mut self, worker_id: &str) {
        if let Some(w) = self.workers.iter_mut().find(|w| w.id == worker_id) {
            w.alive = false;
            if let Some(job) = w.inflight.take() {
                self.queue.push(job);
            }
        }
    }

    pub fn live_specialists(&self) -> Vec<&Worker> {
        self.workers
            .iter()
            .filter(|w| w.alive && w.role == "specialist")
            .collect()
    }

    pub fn metrics(&self) -> Metrics {
        Metrics {
            queue_depth: self.queue.len(),
            executor_health: self
                .workers
                .iter()
                .map(|w| (w.id.clone(), w.alive))
                .collect(),
        }
    }

    pub fn node_for<'a>(&self, plan: &'a Plan, job: &str) -> Option<&'a DagNode> {
        plan.nodes.iter().find(|n| n.id.to_string() == job)
    }
}

impl Default for Broker {
    fn default() -> Self {
        Self::new()
    }
}
