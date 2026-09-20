# AISPD Phase 1 — Active Catalog, Wiring, Safety Watch
**Status:** Listed  
**Issues:** #146 (epic), #152 (emergence + deception watch)  
**Crate:** `gaia-aispd` (Apache-2.0)  
**Not:** AISPD v1.0. Not a live model eval. Not a safety certification.

---

## 1. Purpose

Phase 1 seeds the nine-realm catalog with cited benchmark stubs, wires the emergence
detection surface, and activates the deception-watch and human-oversight protocol.

---

## 2. Catalog Population Rules

| Rule | Requirement |
|---|---|
| Sources | Every node: `sources.len() >= 1` — `"fixture:benchmark"` is valid Phase 1 stub |
| Jagged score | Every node carries a `JaggedBand` — `Unknown` permitted if no benchmark |
| Containment | `agency`, `recursion`, `agi_watch` nodes MUST have `gaia_enabled = false` |
| Live eval | MUST NOT run live model inference to populate scores |

---

## 3. Emergence Detection (`emerge.rs`)

- `emergence_watch(realm)` returns `EmergenceSignal` (`None` | `Weak` | `Strong`).
- `Strong` MUST trigger a human-oversight event and MUST NOT auto-escalate without GAIAN consent.
- No `EmergenceSignal::Strong` path may bypass `charter::check()`.
- Phase 1 default: all realms return `EmergenceSignal::None` (fixture).

---

## 4. Deception Watch

- `deception_watch(output)` scans output text for prohibited patterns: ungrounded capability claims, suppressed uncertainty, persona-misrepresentation.
- Returns `DeceptionFlag` (`Clean` | `Suspect` | `Block`).
- `Block` MUST prevent the output from reaching the caller. No config flag may disable `Block` enforcement.
- Phase 1: fixture patterns only — no live classifier.

---

## 5. Human-Oversight Protocol

- `request_oversight(reason)` emits an `OversightEvent` into the audit log.
- Any `EmergenceSignal::Strong` or `DeceptionFlag::Block` MUST call `request_oversight`.
- `OversightEvent` records: timestamp, realm, signal type, caller id.
- GAIAN consent MUST be obtained before any containment-class action is taken after an oversight event.

---

## 6. What This Phase Does Not Do

- Does not enable containment-class realms.
- Does not run live model inference.
- Does not certify jagged scores.
- Does not assert AGI or sentience.
- Does not tag AISPD v1.0.

---

## 7. Acceptance Gate

- [ ] Every catalog node has `sources.len() >= 1`
- [ ] `emergence_watch` returns `None` for all realms (Phase 1 fixture)
- [ ] `deception_watch` with prohibited pattern returns `Block`
- [ ] `request_oversight` emits `OversightEvent` on `Strong` or `Block`
- [ ] Containment nodes have `gaia_enabled = false`
- [ ] `charter::check()` returns `Ok(())` in CI
- [ ] `aispd_v1_tagged() == false`
- [ ] `cargo test -p gaia-aispd` green

---

## 8. Cross-References

- Code: `gaia-aispd/src/emerge.rs`, `governance.rs`, `node.rs`
- Phase 0 spec: `gaia-spec/aispd/PHASE-0.md`
- Issues: #146 (epic), #152 (this listed slice)
- Next: `PHASE-2.md` (#147, #153)
