//! #30 HPC batch adapter. Pull-based. Not a live Slurm or MPI fabric.

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchTask {
    pub job_id: String,
    pub kind: String,
    pub payload: String,
}

/// Local stand-in for a Slurm/MPI pull executor.
pub struct HpcAdapter {
    queue: VecDeque<BatchTask>,
    pulled: Vec<BatchTask>,
}

impl Default for HpcAdapter {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
            pulled: Vec::new(),
        }
    }
}

impl HpcAdapter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn submit(&mut self, job_id: &str, kind: &str, payload: &str) {
        self.queue.push_back(BatchTask {
            job_id: job_id.into(),
            kind: kind.into(),
            payload: payload.into(),
        });
    }

    pub fn pull(&mut self) -> Option<BatchTask> {
        let task = self.queue.pop_front()?;
        self.pulled.push(task.clone());
        Some(task)
    }

    pub fn pulled(&self) -> &[BatchTask] {
        &self.pulled
    }
}
