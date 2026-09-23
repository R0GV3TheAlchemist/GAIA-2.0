//! #30 HPC batch adapter. Pull-based. Not a live Slurm or MPI fabric.

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchTask {
    pub job_id: String,
    pub kind: String,
    pub payload: String,
}

/// Local stand-in for a Slurm/MPI pull executor.
#[derive(Default)]
pub struct HpcAdapter {
    queue: VecDeque<BatchTask>,
    pulled: Vec<BatchTask>,
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
