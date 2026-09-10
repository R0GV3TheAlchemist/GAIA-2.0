//! #4 end-to-end CLI proof: text -> unsigned local plan; no execution.

use std::process::Command;

#[test]
fn gaia_intent_prints_inspectable_local_plan() {
    let exe = env!("CARGO_BIN_EXE_gaia");
    let output = Command::new(exe)
        .args(["intent", "research and summarize CARE"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("signed=false"));
    assert!(stdout.contains("privacy=local-only"));
    assert!(stdout.contains("execution=not-started"));
    assert!(stdout.contains("nodes=3"));
}

#[test]
fn gaia_intent_requires_text() {
    let exe = env!("CARGO_BIN_EXE_gaia");
    let output = Command::new(exe).args(["intent"]).output().unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr).unwrap().contains("usage:"));
}
