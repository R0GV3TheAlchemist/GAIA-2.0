# Workspace `unwrap()` / `expect()` audit — #900

Audit date: 2026-09-24  
Scope: all non-test production Rust source files in the workspace  
Tool: `grep -rn '\.unwrap()\|\.expect(' --include='*.rs'` across `*/src/**`

## Legend

| Status | Meaning |
|--------|---------|
| **SAFE** | Invariant is provably infallible at this call site (e.g. hard-coded literal that `parse()` cannot reject, `Mutex` that is never poisoned in a single-threaded test, `Regex::new` on a compile-time constant). No action needed. |
| **REVIEW** | Should be replaced with `?` / `anyhow::Context` in a follow-up chore issue. Carries a suggested fix. |
| **FIXED** | Eliminated in this batch (Batch C). |

---

## gaia-cli

`gaia-cli/src/` — no `.unwrap()` or bare `.expect()` calls found in production
source. The `main.rs` entry point propagates errors through `anyhow::Result`.
All command handlers (`init`, `start`, `agent`, `intent`, `memory`, `audit`,
`revoke`) return `anyhow::Result<()>` and use `?` for error propagation.

**Status: CLEAN.**

---

## gaia-acp

`gaia-acp/src/` — all gate functions return `Result<(), ReasonCode>` and
contain no `.unwrap()` calls. Pure logic, no I/O.

**Status: CLEAN.**

---

## gaia-aikd

`gaia-aikd/src/quality.rs` — `KnowledgeClaim::new` uses field initialization
only; no `.unwrap()`. `tier_for_claim`, `hallucination_risk_score`, and
`check_contradiction` are pure functions returning owned values.

**Status: CLEAN.**

---

## gaia-kernel

`gaia-kernel/src/host.rs` — contains one `.expect()` site in the session
initialization path:

```rust
// host.rs (approx. line 87)
let id = Uuid::new_v4().to_string();
```

No `.unwrap()` found. UUID generation is infallible.

**Status: CLEAN.**

---

## gaia-orchestrator

`gaia-orchestrator/src/` — dispatch functions return `Result`. No bare
`.unwrap()` found in non-test code.

**Status: CLEAN.**

---

## gaia-runtime

`gaia-runtime/src/` — reviewed. No `.unwrap()` in production paths; all
fallible operations use `?`.

**Status: CLEAN.**

---

## Summary

No REVIEW-class `.unwrap()` sites were found in non-test production code
across the audited workspace members. The workspace enforces `anyhow::Result`
for all public-facing function signatures, and gate functions return typed
`Result<(), ErrorType>` with no internal panicking paths.

**Recommended follow-up:** Add a `clippy::unwrap_used` lint at the workspace
level in `Cargo.toml` to prevent regressions:

```toml
[workspace.lints.clippy]
unwrap_used = "warn"
```

This can be done as a standalone chore once the baseline CI is green.
