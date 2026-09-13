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
    pub desired_specialists: usize,
    pub live_specialists: usize,
    pub hour_utc: u8,
    pub carbon_ok: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CarbonTimetable {
    pub closed_hours_utc: Vec<u8>,
}

impl CarbonTimetable {
    pub fn default_peak() -> Self {
        Self {
            closed_hours_utc: vec![16, 17, 18],
        }
    }

    pub fn is_open(&self, hour_utc: u8) -> bool {
        !self.closed_hours_utc.contains(&hour_utc)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReconcileReport {
    pub revived: Vec<String>,
    pub live_specialists: usize,
    pub desired_specialists: usize,
}

/// Pull-based resource broker. Nodes poll; no inbound ports.
pub struct Broker {
    pub workers: Vec<Worker>,
    pub queue: Vec<String>,
    pub desired_specialists: usize,
    pub timetable: CarbonTimetable,
    pub hour_utc: u8,
    pub carbon_ok: bool,
}

impl Broker {
    pub fn new() -> Self {
        let timetable = CarbonTimetable::default_peak();
        let hour_utc = 12;
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
            desired_specialists: 2,
            timetable,
            hour_utc,
            carbon_ok: true,
        }
    }

    pub fn requires_inbound_ports(&self) -> bool {
        false
    }

    /// There is no listen socket. ColonyOS pull only.
    pub fn listen(&self, _bind: &str) -> Result<(), String> {
        Err("broker does not open inbound ports".into())
    }

    pub fn set_hour_utc(&mut self, hour_utc: u8) {
        self.hour_utc = hour_utc;
        self.carbon_ok = self.timetable.is_open(hour_utc);
    }

    pub fn enqueue_plan(&mut self, plan: &Plan) {
        for n in &plan.nodes {
            self.queue.push(n.id.to_string());
        }
    }

    /// FIFO pull. Manager only delegates; specialists execute.
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
        if self.queue.is_empty() {
            return Ok(None);
        }
        let job = self.queue.remove(0);
        w.inflight = Some(job.clone());
        Ok(Some(job))
    }

    pub fn complete(&mut self, worker_id: &str) {
        if let Some(w) = self.workers.iter_mut().find(|w| w.id == worker_id) {
            w.inflight = None;
        }
    }

    pub fn kill(&mut self, worker_id: &str) {
        if let Some(w) = self.workers.iter_mut().find(|w| w.id == worker_id) {
            w.alive = false;
            if let Some(job) = w.inflight.take() {
                self.queue.insert(0, job);
            }
        }
    }

    pub fn reconcile(&mut self) -> ReconcileReport {
        let mut revived = Vec::new();
        while self.live_specialists().len() < self.desired_specialists {
            let Some(id) = self
                .workers
                .iter()
                .find(|w| w.role == "specialist" && !w.alive)
                .map(|w| w.id.clone())
            else {
                break;
            };
            if let Some(w) = self.workers.iter_mut().find(|w| w.id == id) {
                w.alive = true;
                w.inflight = None;
            }
            revived.push(id);
        }
        ReconcileReport {
            revived,
            live_specialists: self.live_specialists().len(),
            desired_specialists: self.desired_specialists,
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
            desired_specialists: self.desired_specialists,
            live_specialists: self.live_specialists().len(),
            hour_utc: self.hour_utc,
            carbon_ok: self.carbon_ok,
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
