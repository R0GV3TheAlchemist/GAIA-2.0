//! Parallel agent/task fan-out (#937).
//!
//! Independent jobs in one DAG tier run on `tokio::spawn` + `JoinSet`.
//! A single failure does not drop the other results.

use std::time::{Duration, Instant};

use tokio::task::JoinSet;
use tokio::time::timeout;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskTimeout {
    pub agent_id: String,
    pub elapsed: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobError {
    pub agent_id: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobOk {
    pub agent_id: String,
    pub output: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialFailure {
    pub successes: Vec<JobOk>,
    pub errors: Vec<JobError>,
    pub timeouts: Vec<TaskTimeout>,
}

#[derive(Debug, Clone)]
pub struct FanoutJob {
    pub agent_id: String,
    pub work: Duration,
    pub fail: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FanoutOutcome {
    AllOk(Vec<JobOk>),
    Partial(PartialFailure),
}

/// Run `jobs` in parallel. Each job is bounded by `per_job_timeout`.
pub async fn run_fanout(jobs: Vec<FanoutJob>, per_job_timeout: Duration) -> FanoutOutcome {
    let mut set: JoinSet<(String, Result<Result<String, String>, Duration>)> = JoinSet::new();

    for job in jobs {
        let agent_id = job.agent_id.clone();
        let work = job.work;
        let fail = job.fail;
        set.spawn(async move {
            let started = Instant::now();
            let ran = timeout(per_job_timeout, async move {
                if !work.is_zero() {
                    tokio::time::sleep(work).await;
                }
                if fail {
                    Err(format!("job failed: {agent_id}"))
                } else {
                    Ok(format!("ok:{agent_id}"))
                }
            })
            .await;
            let mapped = match ran {
                Ok(inner) => Ok(inner),
                Err(_) => Err(started.elapsed()),
            };
            (job.agent_id, mapped)
        });
    }

    let mut successes = Vec::new();
    let mut errors = Vec::new();
    let mut timeouts = Vec::new();

    while let Some(joined) = set.join_next().await {
        match joined {
            Ok((agent_id, Ok(Ok(output)))) => successes.push(JobOk { agent_id, output }),
            Ok((agent_id, Ok(Err(message)))) => errors.push(JobError { agent_id, message }),
            Ok((agent_id, Err(elapsed))) => timeouts.push(TaskTimeout { agent_id, elapsed }),
            Err(join_err) => errors.push(JobError {
                agent_id: "unknown".into(),
                message: format!("join: {join_err}"),
            }),
        }
    }

    if errors.is_empty() && timeouts.is_empty() {
        FanoutOutcome::AllOk(successes)
    } else {
        FanoutOutcome::Partial(PartialFailure {
            successes,
            errors,
            timeouts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn three_jobs_run_in_parallel() {
        let jobs = vec![
            FanoutJob {
                agent_id: "a".into(),
                work: Duration::from_millis(80),
                fail: false,
            },
            FanoutJob {
                agent_id: "b".into(),
                work: Duration::from_millis(80),
                fail: false,
            },
            FanoutJob {
                agent_id: "c".into(),
                work: Duration::from_millis(80),
                fail: false,
            },
        ];
        let start = Instant::now();
        let out = run_fanout(jobs, Duration::from_secs(2)).await;
        let wall = start.elapsed();
        match out {
            FanoutOutcome::AllOk(ok) => assert_eq!(ok.len(), 3),
            FanoutOutcome::Partial(p) => panic!("expected all ok, got {p:?}"),
        }
        assert!(
            wall < Duration::from_millis(220),
            "wall {wall:?} should be less than sum of 240ms"
        );
    }

    #[tokio::test]
    async fn one_of_three_failure_is_partial() {
        let jobs = vec![
            FanoutJob {
                agent_id: "a".into(),
                work: Duration::from_millis(1),
                fail: false,
            },
            FanoutJob {
                agent_id: "b".into(),
                work: Duration::from_millis(1),
                fail: true,
            },
            FanoutJob {
                agent_id: "c".into(),
                work: Duration::from_millis(1),
                fail: false,
            },
        ];
        match run_fanout(jobs, Duration::from_secs(2)).await {
            FanoutOutcome::Partial(p) => {
                assert_eq!(p.successes.len(), 2);
                assert_eq!(p.errors.len(), 1);
                assert_eq!(p.errors[0].agent_id, "b");
            }
            FanoutOutcome::AllOk(_) => panic!("expected partial failure"),
        }
    }

    #[tokio::test]
    async fn timeout_returns_task_timeout() {
        let jobs = vec![FanoutJob {
            agent_id: "slow".into(),
            work: Duration::from_millis(200),
            fail: false,
        }];
        match run_fanout(jobs, Duration::from_millis(20)).await {
            FanoutOutcome::Partial(p) => {
                assert_eq!(p.timeouts.len(), 1);
                assert_eq!(p.timeouts[0].agent_id, "slow");
            }
            FanoutOutcome::AllOk(_) => panic!("expected timeout"),
        }
    }
}
