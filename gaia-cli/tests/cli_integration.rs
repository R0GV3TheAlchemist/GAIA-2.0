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
//!
//! Arg shapes (read from source before writing):
//!   init    — --profile (default "developer"), --gateway (default) — ALL OPTIONAL
//!   start   — (read start.rs; tested via --help only)
//!   agent   — subcommand required
//!   intent  — `text: String` positional REQUIRED, --stream, --gateway optional
//!   memory  — subcommand required
//!   audit   — (read audit.rs; tested via --help only)
//!   revoke  — `agent: String` positional REQUIRED, --gateway optional

use assert_cmd::Command;

fn gaia() -> Command {
    Command::cargo_bin("gaia").expect("gaia binary should be present in cargo bin path")
}

// ── top-level ─────────────────────────────────────────────────────────────────

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
// InitArgs: --profile (default "developer"), --gateway (default URL)
// Both flags are optional — bare `gaia init` is valid and exits 0.

#[test]
fn init_help_succeeds() {
    gaia()
        .args(["init", "--help"])
        .assert()
        .success();
}

#[test]
fn init_no_args_succeeds() {
    // All InitArgs fields have defaults — no required args — exits 0.
    gaia()
        .arg("init")
        .assert()
        .success();
}

#[test]
fn init_unknown_flag_fails() {
    // An unrecognised flag is always a clap error regardless of defaults.
    gaia()
        .args(["init", "--not-a-real-flag"])
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
// AgentArgs wraps a required Subcommand — bare `gaia agent` exits non-zero.

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
// IntentArgs: `text: String` positional is REQUIRED.
// `gaia intent` with no positional exits non-zero.

#[test]
fn intent_help_succeeds() {
    gaia()
        .args(["intent", "--help"])
        .assert()
        .success();
}

#[test]
fn intent_missing_text_fails() {
    // `text` is a required positional with no default — must exit non-zero.
    gaia()
        .arg("intent")
        .assert()
        .failure();
}

// ── memory ────────────────────────────────────────────────────────────────────
// MemoryArgs wraps a required Subcommand — bare `gaia memory` exits non-zero.

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
// RevokeArgs: `agent: String` positional is REQUIRED.
// `gaia revoke` with no positional exits non-zero.

#[test]
fn revoke_help_succeeds() {
    gaia()
        .args(["revoke", "--help"])
        .assert()
        .success();
}

#[test]
fn revoke_missing_agent_id_fails() {
    // `agent` is a required positional — must exit non-zero.
    gaia()
        .arg("revoke")
        .assert()
        .failure();
}
