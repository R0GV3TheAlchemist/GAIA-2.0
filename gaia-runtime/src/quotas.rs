//! Sliding-window call quotas (#934). Memory bytes are declared, not measured RSS.

use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RateLimitExceeded {
    pub agent_id: String,
    pub retry_after: Duration,
}

#[derive(Debug, Clone)]
pub struct AgentQuota {
    pub agent_id: String,
    pub max_calls_per_window: u32,
    pub window_duration: Duration,
    pub max_memory_bytes: usize,
    calls: Vec<Instant>,
    baseline_calls: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeAnomaly {
    pub agent_id: String,
    pub observed_calls: u32,
    pub baseline_calls: u32,
}

impl AgentQuota {
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self {
            agent_id: agent_id.into(),
            max_calls_per_window: 100,
            window_duration: Duration::from_secs(60),
            max_memory_bytes: 64 * 1024 * 1024,
            calls: Vec::new(),
            baseline_calls: 10,
        }
    }

    pub fn allow(&mut self, now: Instant) -> Result<Option<RuntimeAnomaly>, RateLimitExceeded> {
        self.calls
            .retain(|t| now.duration_since(*t) < self.window_duration);
        if self.calls.len() as u32 >= self.max_calls_per_window {
            return Err(RateLimitExceeded {
                agent_id: self.agent_id.clone(),
                retry_after: self.window_duration,
            });
        }
        self.calls.push(now);
        let observed = self.calls.len() as u32;
        let anomaly = if self.baseline_calls > 0 && observed > self.baseline_calls.saturating_mul(10) {
            Some(RuntimeAnomaly {
                agent_id: self.agent_id.clone(),
                observed_calls: observed,
                baseline_calls: self.baseline_calls,
            })
        } else {
            None
        };
        Ok(anomaly)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_hundred_first_call_is_limited() {
        let mut q = AgentQuota::new("agent-a");
        let now = Instant::now();
        for _ in 0..100 {
            q.allow(now).expect("first 100 permitted");
        }
        let err = q.allow(now).expect_err("101st must rate-limit");
        assert_eq!(err.agent_id, "agent-a");
    }
}
