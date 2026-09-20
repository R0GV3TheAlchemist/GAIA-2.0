# AISPD Phase 3 — AGI/ASI Pathway Tracking and v1.0 Gate
**Status:** Listed  
**Issues:** #148 (epic), #154 (AGI watch + governance hooks + v1.0)  
**Crate:** `gaia-aispd` (Apache-2.0)  
**Not:** AISPD v1.0 (that tag does not exist yet). Not an AGI system. Not a sentience claim.

---

## 1. Purpose

Phase 3 defines the AGI/ASI pathway watch surface, wires the governance hooks that
escalate to the TSC when pathway indicators fire, and establishes the v1.0 gate
that AISPD must pass before it can be tagged stable.

---

## 2. AGI/ASI Pathway Watch (`watch.rs`)

- `agi_watch_tick()` returns `PathwaySignal` (`Clear` | `Indicator` | `Threshold`).
- `PathwaySignal::Threshold` MUST immediately:
  1. Call `request_oversight(AgiThreshold)`
  2. Suspend all `Containment`-class swarm agents
  3. Write a signed threshold record to the audit log
- No auto-response beyond suspension is permitted without a TSC resolution.
- Phase 3: `agi_watch_tick()` returns `Clear` from fixture.
- `agi_watch` realm remains `Containment` and `gaia_enabled = false`.

---

## 3. Governance Hooks

| Hook | Trigger | Action |
|---|---|---|
| `oversight_hook` | Any `Strong` / `Block` / `Alert` / `Threshold` signal | Emit `OversightEvent` to audit log |
| `tsc_notify_hook` | `PathwaySignal::Threshold` | Queue TSC notification record |
| `consent_gate` | Any containment-class action post-oversight | Block until GAIAN consent token present |
| `charter_ci_hook` | Every commit to `main` | `charter::check()` MUST return `Ok(())` |

All four hooks MUST be present and non-bypassable. Removing any hook requires a supermajority TSC vote.

---

## 4. v1.0 Gate

`aispd_v1_tagged()` MUST return `false` until every condition below is satisfied
and a TSC resolution is recorded in `rfcs/`.

| Gate item | Condition |
|---|---|
| Phase 0–2 acceptance | All three acceptance gates green in CI |
| Catalog coverage | ≥ 27 cited nodes across all 9 realms |
| Jagged scores | ≥ 5 realms have a non-`Unknown` `JaggedBand` with a cited benchmark |
| Governance hooks | All four hooks verified by integration test |
| Charter audit | External audit string in `charter.rs`; not a fixture |
| AGI claim | `agi_marketing` prohibited item confirmed absent by audit |
| RSI guard | `rsi_guard()` returns `Err(RsiDenied)` confirmed by audit |
| TSC vote | Lazy-consensus resolution filed in `rfcs/aispd-v1.0-resolution.md` |

No code path may set `aispd_v1_tagged()` to `true` without the TSC resolution file present.

---

## 5. What This Phase Does Not Do

- Does not tag AISPD v1.0.
- Does not assert GAIA has achieved AGI or ASI.
- Does not enable containment-class realms.
- Does not perform live pathway measurement.
- Does not relax RSI denial or human-oversight hooks.

---

## 6. Acceptance Gate

- [ ] `agi_watch_tick()` returns `Clear` (Phase 3 fixture)
- [ ] `PathwaySignal::Threshold` suspends swarm agents and calls `request_oversight` in test
- [ ] All four governance hooks present and tested
- [ ] `aispd_v1_tagged() == false`
- [ ] `rsi_guard()` returns `Err(RsiDenied)`
- [ ] `charter::check()` returns `Ok(())` — all five prohibited items absent
- [ ] No TSC resolution file — v1.0 not tagged
- [ ] `cargo test -p gaia-aispd` green

---

## 7. Cross-References

- Code: `gaia-aispd/src/watch.rs`, `governance.rs`, `charter.rs`
- Phase 0 spec: `gaia-spec/aispd/PHASE-0.md`
- Phase 1 spec: `gaia-spec/aispd/PHASE-1.md`
- Phase 2 spec: `gaia-spec/aispd/PHASE-2.md`
- Issues: #148 (epic), #154 (this listed slice)
- RFC placeholder: `rfcs/aispd-v1.0-resolution.md` (must not exist until gate is met)
