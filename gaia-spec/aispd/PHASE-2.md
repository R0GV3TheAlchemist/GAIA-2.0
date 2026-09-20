# AISPD Phase 2 — Bounded Swarms and Default-Deny RSI
**Status:** Listed  
**Issues:** #147 (epic), #153 (bounded swarms + RSI)  
**Crate:** `gaia-aispd` (Apache-2.0)  
**Not:** AISPD v1.0. Not a live swarm. Not an RSI system.

---

## 1. Purpose

Phase 2 defines the bounds under which multi-agent swarms may operate as monitoring
instruments, and formalizes the default-deny posture for recursive self-improvement
(RSI) at every layer of the stack.

---

## 2. Bounded Swarm Rules (`swarm.rs`)

| Rule | Requirement |
|---|---|
| Scope | Swarm agents are read-only monitors — no write access to kernel or SFS |
| Max agents | `MAX_SWARM_SIZE = 8` enforced by `swarm::spawn()` — excess returns `Err(SwarmFull)` |
| GAIAN consent | Every swarm spawn MUST have a GAIAN-signed consent token |
| Isolation | Swarm agents run in isolated namespaces — no cross-agent direct memory access |
| Termination | `swarm::terminate_all()` MUST be callable by any GAIAN at any time |
| RSI prohibition | A swarm agent MUST NOT modify its own weights, prompts, or reward signal |
| Audit | Every spawn, action, and termination written to the audit log |

---

## 3. Default-Deny RSI

- `rsi_guard()` MUST return `Err(RsiDenied)` unconditionally. No config flag overrides this.
- Any code path that receives `Ok(())` from `rsi_guard()` is a bug — tests MUST fail.
- `rsi_autolaunch` is a prohibited charter item — `charter::check()` catches any reference.
- Marketing materials MUST NOT describe GAIA as self-improving without a TSC resolution.

---

## 4. RSI Monitor Surface

- `rsi_monitor_tick()` returns `RsiSignal` (`Stable` | `Drift` | `Alert`).
- `RsiSignal::Alert` MUST trigger `request_oversight(RsiAlert)` immediately.
- No `RsiSignal::Alert` path may auto-remediate without GAIAN consent.
- Phase 2: `rsi_monitor_tick()` returns `Stable` from fixture.

---

## 5. What This Phase Does Not Do

- Does not launch a live swarm.
- Does not perform live RSI detection.
- Does not enable `recursion` or `agency` realms.
- Does not assert AGI or sentience.
- Does not tag AISPD v1.0.

---

## 6. Acceptance Gate

- [ ] `swarm::spawn()` with count > 8 returns `Err(SwarmFull)`
- [ ] `rsi_guard()` always returns `Err(RsiDenied)`
- [ ] `rsi_monitor_tick()` returns `Stable` (Phase 2 fixture)
- [ ] `RsiSignal::Alert` calls `request_oversight` in test
- [ ] `charter::check()` returns `Ok(())` — `rsi_autolaunch` absent
- [ ] `aispd_v1_tagged() == false`
- [ ] `cargo test -p gaia-aispd` green

---

## 7. Cross-References

- Code: `gaia-aispd/src/swarm.rs`, `watch.rs`, `governance.rs`
- Phase 0 spec: `gaia-spec/aispd/PHASE-0.md`
- Phase 1 spec: `gaia-spec/aispd/PHASE-1.md`
- Issues: #147 (epic), #153 (this listed slice)
- Next: `PHASE-3.md` (#148, #154)
