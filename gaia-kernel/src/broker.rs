use std::collections::VecDeque;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    pub cpu_cores: u32,
    pub memory_mb: u32,
    pub gpu: bool,
    pub npu: bool,
    pub vram_mb: u32,
    pub bandwidth_mbps: u32,
    pub location: String,
    pub cuda: bool,
    pub rocm: bool,
    pub opencl: bool,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            cpu_cores: 2,
            memory_mb: 1024,
            gpu: false,
            npu: false,
            vram_mb: 0,
            bandwidth_mbps: 100,
            location: "local".into(),
            cuda: false,
            rocm: false,
            opencl: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub kind: String,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutorInfo {
    pub node_id: String,
    pub caps: Capabilities,
}

/// In-process pull broker. Executors never open inbound ports.
#[derive(Default)]
pub struct Broker {
    inner: Mutex<Inner>,
}

#[derive(Default)]
struct Inner {
    nodes: Vec<ExecutorInfo>,
    queue: VecDeque<Task>,
}

impl Broker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, info: ExecutorInfo) {
        self.inner.lock().expect("broker").nodes.push(info);
    }

    pub fn enqueue(&self, kind: impl Into<String>, payload: impl Into<String>) -> Uuid {
        let task = Task {
            id: Uuid::new_v4(),
            kind: kind.into(),
            payload: payload.into(),
        };
        let id = task.id;
        self.inner.lock().expect("broker").queue.push_back(task);
        id
    }

    /// Pull the next task. Outbound-only from the executor's point of view.
    pub fn pull(&self, _node_id: &str) -> Option<Task> {
        self.inner.lock().expect("broker").queue.pop_front()
    }

    pub fn node_count(&self) -> usize {
        self.inner.lock().expect("broker").nodes.len()
    }
}
