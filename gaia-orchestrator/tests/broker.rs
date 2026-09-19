//! #22 depth: FIFO pull, no listen socket, kill/reschedule, carbon.

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
    assert!(b.listen("0.0.0.0:0").is_err());
}

#[test]
fn pull_is_fifo_in_plan_order() {
    let mut b = Broker::new();
    let plan = accepted_plan();
    let first = plan.nodes[0].id.to_string();
    b.enqueue_plan(&plan);
    let job = b.pull("specialist-a").unwrap().unwrap();
    assert_eq!(job, first);
}

#[test]
fn manager_does_not_execute() {
    let mut b = Broker::new();
    b.enqueue_plan(&accepted_plan());
    assert_eq!(b.pull("manager").unwrap(), None);
}

#[test]
fn metrics_queue_and_health() {
    let mut b = Broker::new();
    let plan = accepted_plan();
    b.enqueue_plan(&plan);
    let m = b.metrics();
    assert_eq!(m.queue_depth, plan.nodes.len());
    assert!(m
        .executor_health
        .iter()
        .any(|(id, ok)| id == "specialist-a" && *ok));
}

#[test]
fn kill_reschedules_inflight() {
    let mut b = Broker::new();
    let plan = accepted_plan();
    b.enqueue_plan(&plan);
    let job = b.pull("specialist-a").unwrap().expect("job");
    b.kill("specialist-a");
    let again = b.pull("specialist-b").unwrap().expect("rescheduled");
    assert_eq!(again, job);
}

#[test]
fn reconcile_revives_to_desired_count() {
    let mut b = Broker::new();
    b.enqueue_plan(&accepted_plan());
    let _ = b.pull("specialist-a").unwrap();
    b.kill("specialist-a");
    let report = b.reconcile();
    assert_eq!(report.revived, vec!["specialist-a".to_string()]);
    assert_eq!(report.live_specialists, 2);
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
    assert!(b
        .pull("specialist-a")
        .unwrap_err()
        .contains("carbon window"));
    b.set_hour_utc(12);
    assert!(b.pull("specialist-a").unwrap().is_some());
}
