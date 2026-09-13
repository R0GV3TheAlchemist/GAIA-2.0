//! #22 Broker: kill/reschedule, reconcile, carbon timetable, no inbound ports.

use gaia_memos::MemOs;
use gaia_orchestrator::{Broker, IntentEngine, TaskPlanner};

fn accepted_plan() -> gaia_orchestrator::Plan {
    let mut mem = MemOs::new();
    let g = IntentEngine::local_stub()
        .parse("research and summarize", &mut mem)
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
    assert_eq!(m.desired_specialists, 2);
    assert_eq!(m.live_specialists, 2);
    assert!(m.carbon_ok);
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
fn reconcile_revives_to_desired_count() {
    let mut b = Broker::new();
    b.enqueue_plan(&accepted_plan());
    let _ = b.pull("specialist-a").unwrap();
    b.kill("specialist-a");
    assert_eq!(b.live_specialists().len(), 1);
    let report = b.reconcile();
    assert_eq!(report.revived, vec!["specialist-a".to_string()]);
    assert_eq!(report.live_specialists, 2);
    assert_eq!(b.live_specialists().len(), 2);
    assert!(b.workers.iter().find(|w| w.id == "specialist-a").unwrap().alive);
    assert!(b.workers.iter().find(|w| w.id == "specialist-a").unwrap().inflight.is_none());
}

#[test]
fn carbon_window_blocks_pull() {
    let mut b = Broker::new();
    b.enqueue_plan(&accepted_plan());
    b.carbon_ok = false;
    assert!(b.pull("specialist-a").is_err());
}

#[test]
fn carbon_timetable_blocks_peak_hours() {
    let mut b = Broker::new();
    b.enqueue_plan(&accepted_plan());
    b.set_hour_utc(17);
    assert!(!b.carbon_ok);
    assert!(b.pull("specialist-a").unwrap_err().contains("carbon window"));
    b.set_hour_utc(12);
    assert!(b.carbon_ok);
    assert!(b.pull("specialist-a").unwrap().is_some());
}
