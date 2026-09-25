//! Integration tests for `gaia-validate`.
//!
//! These tests work entirely with pre-written fixture JSON and never invoke
//! the shell script, so they run in CI without a Rust toolchain or bash.

use gaia_validate::{
    get_validation_result, get_attempt_history, is_sha_current,
    result::{ValidationResult, ValidationStatus},
};
use std::io::Write;
use tempfile::NamedTempFile;
use tempfile::TempDir;

// ── fixtures ─────────────────────────────────────────────────────────────────

fn passing_result(sha: &str, attempt: u32) -> String {
    format!(
        r#"{{"schema_version":"1.1","head_sha":"{sha}","timestamp":"2026-09-25T00:00:00Z","mode":"changed","attempt":{attempt},"status":"passed","failed_stage":null,"no_progress":false,"stages":[],"diagnostics":[]}}"#
    )
}

fn failed_result(sha: &str, attempt: u32, fp: &str) -> String {
    format!(
        r#"{{"schema_version":"1.1","head_sha":"{sha}","timestamp":"2026-09-25T00:00:00Z","mode":"changed","attempt":{attempt},"status":"failed","failed_stage":"cargo-fmt","no_progress":false,"stages":[],"diagnostics":[{{"level":"error","code":"E0001","path":"src/lib.rs:1","message":"test error","fingerprint":"{fp}"}}]}}"#
    )
}

fn no_progress_result(sha: &str, attempt: u32) -> String {
    format!(
        r#"{{"schema_version":"1.1","head_sha":"{sha}","timestamp":"2026-09-25T00:00:00Z","mode":"changed","attempt":{attempt},"status":"no_progress","failed_stage":"cargo-fmt","no_progress":true,"stages":[],"diagnostics":[{{"level":"error","code":"E0001","path":"src/lib.rs:1","message":"test error","fingerprint":"E0001:src/lib.rs:1"}}]}}"#
    )
}

fn write_temp(content: &str) -> NamedTempFile {
    let mut f = NamedTempFile::new().unwrap();
    write!(f, "{}", content).unwrap();
    f
}

// ── get_validation_result ─────────────────────────────────────────────────────

#[test]
fn read_passing_result() {
    let f = write_temp(&passing_result("abc123", 1));
    let r: ValidationResult = get_validation_result(f.path()).unwrap();
    assert!(r.is_passing());
    assert!(!r.is_no_progress());
    assert_eq!(r.head_sha, "abc123");
    assert_eq!(r.attempt, 1);
}

#[test]
fn read_failed_result() {
    let f = write_temp(&failed_result("def456", 2, "E0001:src/lib.rs:1"));
    let r: ValidationResult = get_validation_result(f.path()).unwrap();
    assert!(!r.is_passing());
    assert!(!r.is_no_progress());
    assert_eq!(r.status, ValidationStatus::Failed);
    assert_eq!(r.fingerprints(), vec!["E0001:src/lib.rs:1"]);
}

#[test]
fn read_no_progress_result() {
    let f = write_temp(&no_progress_result("ghi789", 3));
    let r: ValidationResult = get_validation_result(f.path()).unwrap();
    assert!(r.is_no_progress());
    assert_eq!(r.status, ValidationStatus::NoProgress);
}

#[test]
fn rejects_bad_json() {
    let f = write_temp("not json");
    assert!(get_validation_result(f.path()).is_err());
}

// ── is_sha_current ────────────────────────────────────────────────────────────

#[test]
fn sha_current_match() {
    let f = write_temp(&passing_result("aabbcc", 1));
    let r = get_validation_result(f.path()).unwrap();
    assert!(is_sha_current(&r, "aabbcc"));
}

#[test]
fn sha_current_mismatch() {
    let f = write_temp(&passing_result("aabbcc", 1));
    let r = get_validation_result(f.path()).unwrap();
    assert!(!is_sha_current(&r, "ddeeff"));
}

// ── get_attempt_history ───────────────────────────────────────────────────────

#[test]
fn history_sorted_by_attempt() {
    let dir = TempDir::new().unwrap();

    // Write three numbered files out of order
    for (n, sha) in [(3u32, "sha3"), (1, "sha1"), (2, "sha2")] {
        let path = dir.path().join(format!("agent-validation-{n}.json"));
        std::fs::write(&path, failed_result(sha, n, &format!("fp-{n}"))).unwrap();
    }

    let history = get_attempt_history(dir.path()).unwrap();
    assert_eq!(history.len(), 3);
    assert_eq!(history.entries[0].attempt, 1);
    assert_eq!(history.entries[1].attempt, 2);
    assert_eq!(history.entries[2].attempt, 3);
    assert_eq!(history.latest().unwrap().head_sha, "sha3");
}

#[test]
fn history_is_stuck_when_same_fingerprints() {
    let dir = TempDir::new().unwrap();
    let fp = "E0001:src/lib.rs:1";

    for n in [1u32, 2] {
        let path = dir.path().join(format!("agent-validation-{n}.json"));
        std::fs::write(&path, failed_result(&format!("sha{n}"), n, fp)).unwrap();
    }

    let history = get_attempt_history(dir.path()).unwrap();
    assert!(history.is_stuck());
}

#[test]
fn history_not_stuck_when_different_fingerprints() {
    let dir = TempDir::new().unwrap();

    std::fs::write(
        dir.path().join("agent-validation-1.json"),
        failed_result("sha1", 1, "fp-old"),
    ).unwrap();
    std::fs::write(
        dir.path().join("agent-validation-2.json"),
        failed_result("sha2", 2, "fp-new"),
    ).unwrap();

    let history = get_attempt_history(dir.path()).unwrap();
    assert!(!history.is_stuck());
}

#[test]
fn history_fallback_to_unnumbered_file() {
    let dir = TempDir::new().unwrap();
    std::fs::write(
        dir.path().join("agent-validation.json"),
        passing_result("sha1", 1),
    ).unwrap();

    let history = get_attempt_history(dir.path()).unwrap();
    assert_eq!(history.len(), 1);
    assert!(history.latest().unwrap().is_passing());
}

#[test]
fn history_empty_dir() {
    let dir = TempDir::new().unwrap();
    let history = get_attempt_history(dir.path()).unwrap();
    assert!(history.is_empty());
    assert!(history.latest().is_none());
    assert!(!history.is_stuck());
}
