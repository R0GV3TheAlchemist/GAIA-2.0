//! Per-agent circuit breaker (#934). In-memory scaffold.

use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitOpen;

#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    pub agent_id: String,
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
    state: CircuitState,
    consecutive_failures: u32,
    opened_at: Option<Instant>,
}

impl CircuitBreaker {
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self {
            agent_id: agent_id.into(),
            failure_threshold: 3,
            recovery_timeout: Duration::from_secs(30),
            state: CircuitState::Closed,
            consecutive_failures: 0,
            opened_at: None,
        }
    }

    pub fn state(&self) -> CircuitState {
        self.state
    }

    pub fn before_call(&mut self, now: Instant) -> Result<(), CircuitOpen> {
        match self.state {
            CircuitState::Closed => Ok(()),
            CircuitState::Open => {
                if let Some(opened) = self.opened_at {
                    if now.duration_since(opened) >= self.recovery_timeout {
                        self.state = CircuitState::HalfOpen;
                        return Ok(());
                    }
                }
                Err(CircuitOpen)
            }
            CircuitState::HalfOpen => Ok(()),
        }
    }

    pub fn on_success(&mut self) {
        self.consecutive_failures = 0;
        self.state = CircuitState::Closed;
        self.opened_at = None;
    }

    pub fn on_failure(&mut self, now: Instant) {
        self.consecutive_failures = self.consecutive_failures.saturating_add(1);
        match self.state {
            CircuitState::HalfOpen => {
                self.state = CircuitState::Open;
                self.opened_at = Some(now);
            }
            CircuitState::Closed | CircuitState::Open => {
                if self.consecutive_failures >= self.failure_threshold {
                    self.state = CircuitState::Open;
                    self.opened_at = Some(now);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_failures_open_circuit() {
        let mut cb = CircuitBreaker::new("agent-a");
        let now = Instant::now();
        for _ in 0..3 {
            cb.before_call(now).expect("closed allows calls");
            cb.on_failure(now);
        }
        assert_eq!(cb.state(), CircuitState::Open);
        assert_eq!(cb.before_call(now), Err(CircuitOpen));
    }

    #[test]
    fn half_open_success_closes() {
        let mut cb = CircuitBreaker::new("agent-a");
        cb.recovery_timeout = Duration::from_millis(1);
        let now = Instant::now();
        for _ in 0..3 {
            let _ = cb.before_call(now);
            cb.on_failure(now);
        }
        let later = now + Duration::from_millis(2);
        cb.before_call(later).expect("probe allowed");
        assert_eq!(cb.state(), CircuitState::HalfOpen);
        cb.on_success();
        assert_eq!(cb.state(), CircuitState::Closed);
    }
}
