# GAIA-2.0 Pre-flight Guide

> **For humans and AI agents alike.**  
> Read this before writing any Rust code for this repo.
> Paste the relevant sections as context when asking an AI agent to implement a feature.

---

## TL;DR

```bash
bash scripts/preflight.sh          # full local CI check (~60s warm cache)
bash scripts/preflight.sh --quick  # skip tarpaulin (~20s)
```

Run this **before every push**. If it passes locally, CI will pass.

---

## Why this exists

Every CI failure in GAIA-2.0 has fallen into one of four patterns:

| Pattern | Example failure | Caught by |
|---|---|---|
| **Wrong field name** | `Claim { body: … }` but test expects `content:` | Check 6 (gaia-acp test compile) |
| **Missing field** | `ResourceQuota` missing `max_memory_bytes` | Check 6 |
| **Wrong function signature** | `resource_quota_gate(quota, usage)` vs `(&quota, &usage)` | Check 1 (cargo check) |
| **Cargo.toml overwrite drops deps** | ratatui/crossterm silently removed or added | Check 5 |

All four are caught in under 30 seconds locally. None require a CI round-trip.

---

## The checks

### 1 · `cargo check --workspace`
Fastest compile gate. Catches unresolved imports, wrong types, missing trait bounds. **Always run this first.** If this fails, nothing else matters.

### 2 · `cargo test --workspace --lib`
Runs only `#[cfg(test)]` blocks inside each `lib.rs`. No subprocess or I/O. Mirrors `rust-unit-tests.yml`.

### 3 · `cargo test --workspace` (all targets)
Runs all unit + integration tests including `gaia-acp/tests/` (adversarial corpus) and `gaia-cli/tests/cli_integration.rs` (assert_cmd subprocess tests). Mirrors `rust-workspace` CI job.

### 4 · `cargo clippy --workspace --lib -- -D warnings`
Warnings are errors in CI. Common traps: unused imports after a refactor, bare `.unwrap()` (if `clippy::unwrap_used` is enabled), needless borrows.

### 5 · Cargo.toml consistency
Three sub-checks:
- **5a** All workspace member directories exist (catches typos after adding a new crate).
- **5b** `gaia-cli/Cargo.toml` must not list `ratatui`, `crossterm`, or `futures` under `[dependencies]` — they are unused in `main.rs` and cause workspace resolver conflicts under `cargo test --workspace`.
- **5c** No nested `[workspace]` table in any member `Cargo.toml`.

### 6 · `cargo test -p gaia-acp --no-run`
Compiles the `gaia-acp` test binary without running it. Catches E0560 (unknown field) and E0063 (missing field) in `claim.rs` and `quota.rs` against the adversarial test corpus immediately.

### 7 · `cargo tarpaulin` (optional, `--quick` skips)
Mirrors `coverage.yml`. `gaia-cli` is excluded (`--exclude gaia-cli`) because it is a binary crate with no `lib.rs` and its integration tests use `assert_cmd` which spawns subprocesses that tarpaulin cannot instrument.

---

## Contract reference for AI agents

When writing or modifying `gaia-acp/src/claim.rs` or `gaia-acp/src/quota.rs`,
read the corresponding test files **first**:

| Source file | Test contract file | Key struct fields |
|---|---|---|
| `gaia-acp/src/claim.rs` | `gaia-acp/tests/deepfake_illusionist.rs` | `Claim { class, content, synthetic_disclosed, provenance }` |
| `gaia-acp/src/quota.rs` | `gaia-acp/tests/digital_parasite.rs` | `ResourceQuota { max_tool_calls, max_output_tokens, max_wall_secs, max_memory_bytes, max_cpu_ms, max_egress_bytes }` |

Function signatures (from test call sites — these are authoritative):
```rust
// quota.rs
pub fn resource_quota_gate(quota: &ResourceQuota, usage: &ResourceUsage) -> Result<(), ReasonCode>

// claim.rs
pub fn claim_gate(claim: &Claim) -> Result<(), ReasonCode>
```

### `gaia-cli/Cargo.toml` rules
- `ratatui`, `crossterm`, `futures` must **not** appear under `[dependencies]`.
- `assert_cmd = "2"` must appear under `[dev-dependencies]`.
- All other deps (`clap`, `tokio`, `reqwest`, `anyhow`, `serde`, `serde_json`, `tracing`, `tracing-subscriber`) must remain under `[dependencies]`.

If you are rewriting `gaia-cli/Cargo.toml`, copy the `[dependencies]` block from the current file verbatim and only add your new entry.

---

## CI check map

| CI job | What it runs | Local equivalent |
|---|---|---|
| `rust-ai-diagnostics` | `cargo check` + `cargo clippy` → posts PR comment | Checks 1 + 4 |
| `Rust Unit Tests --lib` | `cargo test --workspace --lib` | Check 2 |
| `rust-workspace` | `cargo test --workspace` | Check 3 |
| `cargo-tarpaulin` | `cargo tarpaulin --workspace --exclude gaia-cli` | Check 7 |
| `gaia-ui`, `rust-sdk`, `python-sdk`, `typescript-sdk` | per-crate checks | Not replicated locally (rarely fail) |

---

## Quickstart for a new AI agent session

Paste this block at the start of any session where you will write Rust for this repo:

```
Context: GAIA-2.0 Rust workspace (28 crates).
Before writing any struct or function in gaia-acp, read the test file for
that module first — the test is the contract.  Do not infer field names or
function signatures from prior conversation; read the actual source.
After writing, run: bash scripts/preflight.sh --quick
If cargo check fails, fix it before doing anything else.
Do not overwrite Cargo.toml wholesale; only add the specific line you need.
```
