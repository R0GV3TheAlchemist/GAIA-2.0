//! #22 Broker: kill/reschedule, no inbound ports, queue + health metrics.

use gaia_memos::MemOs;
use gaia_orchestrator::{Broker, IntentEngine, TaskPlanner};

fn accepted_plan() -> gaia_orchestrator::Plan {
    let mem = MemOs::new();
    let g = IntentEngine::local_stub()
        .parse("research and summarize", &mem)
        .unwrap();
    let mut plan = TaskPlanner::from_intent(&g);
    plan.accept();
    plan
}

#[test]
fn broker_never_requires_inbound_ports() {
    let b = Broker::new();
    assert!(!b.requires_inbound_ports());
}

#[test]
fn metrics_queue_and_health() {
    let mut b = Broker::new();
    let plan = accepted_plan();
    b.enqueue_plan(&plan);
    let m = b.metrics();
    assert_eq!(m.queue_depth, plan.nodes.len());
    assert!(m.executor_health.iter().any(|(id, ok)| id == "specialist-a" && *ok));
}

#[test]
fn kill_reschedules_inflight() {
    let mut b = Broker::new();
    let plan = accepted_plan();
    b.enqueue_plan(&plan);
    let job = b.pull("specialist-a").unwrap().expect("job");
    assert_eq!(b.metrics().queue_depth, plan.nodes.len() - 1);
    b.kill("specialist-a");
    assert!(!b.workers.iter().find(|w| w.id == "specialist-a").unwrap().alive);
    assert!(b.metrics().queue_depth >= 1);
    let again = b.pull("specialist-b").unwrap().expect("rescheduled");
    assert_eq!(again, job);
    assert_eq!(b.live_specialists().len(), 1);
}

#[test]
fn carbon_window_blocks_pull() {
    let mut b = Broker::new();
    b.enqueue_plan(&accepted_plan());
    b.carbon_ok = false;
    assert!(b.pull("specialist-a").is_err());
}
