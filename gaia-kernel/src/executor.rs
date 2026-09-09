use std::sync::Arc;

use crate::broker::{Broker, Capabilities, ExecutorInfo, Task};
use crate::identity::Principal;

pub struct Executor {
    pub node_id: String,
    pub caps: Capabilities,
    broker: Arc<Broker>,
}

impl Executor {
    pub fn new(principal: &Principal, caps: Capabilities, broker: Arc<Broker>) -> Self {
        let node_id = principal.did();
        broker.register(ExecutorInfo {
            node_id: node_id.clone(),
            caps: caps.clone(),
        });
        Self {
            node_id,
            caps,
            broker,
        }
    }

    pub fn pull_one(&self) -> Option<Task> {
        self.broker.pull(&self.node_id)
    }

    pub fn run_task(&self, task: &Task) -> String {
        match task.kind.as_str() {
            "noop" => format!("noop-ok:{}", task.id),
            other => format!("unsupported:{other}"),
        }
    }
}
