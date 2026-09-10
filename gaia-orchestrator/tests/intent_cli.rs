//! #4 CLI proof: inspect by default; explicit --accept permits local stub execution.

use std::process::Command;

fn gaia(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_gaia")).args(args).output().unwrap()
}

#[test]
fn gaia_intent_prints_inspectable_local_plan() {
    let output = gaia(&["intent", "research and summarize CARE"]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("trust=PLACEHOLDER-NOT-CRYPTOGRAPHY"));
    assert!(stdout.contains("privacy=local-only"));
    assert!(stdout.contains("execution=not-started"));
    assert!(stdout.contains("nodes=3"));
}

#[test]
fn gaia_intent_accept_runs_local_nodes() {
    let output = gaia(&["intent", "research and summarize CARE", "--accept"]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("execution=completed"));
    assert!(stdout.contains("completed_nodes=3"));
    assert!(stdout.contains("failed_over_nodes=0"));
    assert!(stdout.contains("audit_events=6"));
    assert!(stdout.contains("PLACEHOLDER-NOT-CRYPTOGRAPHY"));
}

#[test]
fn gaia_intent_accept_can_prove_failover() {
    let output = gaia(&[
        "intent",
        "research and summarize CARE",
        "--accept",
        "--kill-specialist-a",
    ]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("execution=completed"));
    assert!(stdout.contains("failed_over_nodes=1"));
    assert!(stdout.contains("completed_nodes=3"));
    assert!(stdout.contains("audit_events=7"));
}

#[test]
fn gaia_intent_requires_text() {
    let output = gaia(&["intent"]);
    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr).unwrap().contains("usage:"));
}
