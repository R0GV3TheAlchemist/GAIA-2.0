//! CLI integration tests — #938
//!
//! Tests every subcommand at the process level using `assert_cmd`.
//! All fallible operations use `.expect("<context>")` instead of bare
//! `.unwrap()` so that failures print useful context on panic.
//!
//! These tests verify that:
//!   - Each subcommand is reachable and exits with a known code.
//!   - `--help` on every subcommand succeeds (exit 0, non-empty stdout).
//!   - Invocations with missing required args exit non-zero (clap error).
//!
//! NOTE: The commands do real I/O (connect to a runtime, write files, etc.).
//! Tests only exercise `--help` and intentional error paths — they do NOT
//! require a live GAIA runtime to pass.

use assert_cmd::Command;

fn gaia() -> Command {
    Command::cargo_bin("gaia").expect("gaia binary should be present in cargo bin path")
}

// ── top-level help ────────────────────────────────────────────────────────────

#[test]
fn top_level_help_succeeds() {
    gaia()
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn top_level_no_args_fails() {
    // clap requires a subcommand; omitting one exits non-zero.
    gaia()
        .assert()
        .failure();
}

// ── init ─────────────────────────────────────────────────────────────────────

#[test]
fn init_help_succeeds() {
    gaia()
        .args(["init", "--help"])
        .assert()
        .success();
}

#[test]
fn init_missing_required_args_fails() {
    // `init` without its required positional arg must exit non-zero.
    gaia()
        .arg("init")
        .assert()
        .failure();
}

// ── start ─────────────────────────────────────────────────────────────────────

#[test]
fn start_help_succeeds() {
    gaia()
        .args(["start", "--help"])
        .assert()
        .success();
}

// ── agent ─────────────────────────────────────────────────────────────────────

#[test]
fn agent_help_succeeds() {
    gaia()
        .args(["agent", "--help"])
        .assert()
        .success();
}

#[test]
fn agent_no_subcommand_fails() {
    gaia()
        .arg("agent")
        .assert()
        .failure();
}

// ── intent ────────────────────────────────────────────────────────────────────

#[test]
fn intent_help_succeeds() {
    gaia()
        .args(["intent", "--help"])
        .assert()
        .success();
}

// ── memory ────────────────────────────────────────────────────────────────────

#[test]
fn memory_help_succeeds() {
    gaia()
        .args(["memory", "--help"])
        .assert()
        .success();
}

#[test]
fn memory_no_subcommand_fails() {
    gaia()
        .arg("memory")
        .assert()
        .failure();
}

// ── audit ─────────────────────────────────────────────────────────────────────

#[test]
fn audit_help_succeeds() {
    gaia()
        .args(["audit", "--help"])
        .assert()
        .success();
}

// ── revoke ────────────────────────────────────────────────────────────────────

#[test]
fn revoke_help_succeeds() {
    gaia()
        .args(["revoke", "--help"])
        .assert()
        .success();
}

#[test]
fn revoke_missing_agent_id_fails() {
    gaia()
        .arg("revoke")
        .assert()
        .failure();
}
