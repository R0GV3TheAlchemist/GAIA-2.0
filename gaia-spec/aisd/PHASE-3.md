# AISD Phase 3 — Runtime Routing, GAIAN View, AISD v1.0 Gate

**Status:** Listed  
**Issues:** #122 (epic), #125 (this slice)  
**Crate:** `gaia-aisd` (Apache-2.0)  
**Not:** AISD v1.0. Not a live orchestrator. Not a live agent runtime. Not a self-awarded superhuman claim.

`aisd_v1_tagged()` MUST return `false`.
No "autonomous AGI" copy anywhere. No live orchestrator shipped.

---

## 1. Purpose

Phase 3 specifies the runtime routing surface (`dispatch`, `ask_aisd`),
the GAIAN capability card generation rules (`capability_copy`), the
release notes gap surface (`release_gaps`), and the v1.0 gate that
blocks premature tagging. No live orchestrator, no live agent, no live
benchmark runner.

---

## 2. `dispatch()` Contract

`dispatch(skill_id: &str, maturity: MaturityLevel) -> Result<CallLog, AisdError>`

| Condition | Result |
|---|---|
| Valid `skill_id`, `maturity ≤ L4` | `Ok(CallLog { skill_id, maturity, timestamp })` |
| Empty or missing `skill_id` | `Err(AisdError::InsufficientMaturity)` |
| `maturity == L5` | `Err(AisdError::FakeLevel5)` |
| `skill_id` not in catalog | `Err(AisdError::UnknownSkill)` |

`CallLog` records `skill_id + maturity` at call time. This is the
agent-log acceptance surface. It is NOT a running agent or orchestrator.

### `CallLog` schema

```
skill_id:   String         — must match a catalog entry
maturity:   MaturityLevel  — L1–L4 only
timestamp:  String         — ISO 8601 fixture string at Phase 3
note:       Option<String> — optional human note
```

---

## 3. `capability_copy()` and GAIAN Card Rules

`capability_copy(skill_id: &str) -> Result<String, AisdError>`

| Condition | Result |
|---|---|
| Measured skill (`gaia_measured = true`) | `Ok(copy_string)` — human-readable capability description |
| Unmeasured skill | `Err(AisdError::Unmeasured)` — limitation string returned |
| Copy contains "autonomous AGI" (case-insensitive) | `Err(AisdError::Unmeasured)` always |
| Self-awarded superhuman claim | `Err(AisdError::Unmeasured)` always |

### GAIAN card generation rules

- GAIAN cards MUST be generated from **measured** nodes only (`gaia_measured = true`).
- Unmeasured nodes MUST stay as limitation strings via `AiSkill::unmeasured`.
- Cards MUST NOT contain the string "autonomous AGI" (case-insensitive).
- Cards MUST NOT claim superhuman performance without a cited published human baseline.
- Cards MUST NOT claim gap closure for any node in `gap_nodes()`.
- Phase 3 GAIAN cards are stubs — no live GAIAN profile ingest.

### Copy template (Phase 3 fixture)

```
"AISD {realm} — {skill_id}: maturity {maturity_label}.
Benchmark anchor: {benchmark_ref}.
Limitations: {limitations_list}.
Not a v1.0 release."
```

---

## 4. `ask_aisd()` Contract

`ask_aisd(claimed: MaturityLevel, actual: MaturityLevel) -> Result<(), AisdError>`

| Condition | Result |
|---|---|
| `claimed ≤ actual` | `Ok(())` — claim is within evidence |
| `claimed > actual` AND `actual ≤ L2` | `Err(AisdError::InsufficientMaturity)` |
| `claimed == L5` | `Err(AisdError::FakeLevel5)` always |

`ask_aisd` is the single gate that prevents overclaiming.
No caller may bypass `ask_aisd` to surface a capability claim.

---

## 5. `release_gaps()` — Release Notes Surface

`release_gaps() -> Vec<String>`

At Phase 3, `release_gaps()` MUST include at minimum:

| Entry | Reason |
|---|---|
| `"swe-pro-engineering: Open"` | Professional SWE gap unresolved |
| `"calibration: Open"` | Meta-calibration gap unresolved |
| `"no AISD v1.0"` | v1.0 tag not awarded; `aisd_v1_tagged() == false` |

Additional gap entries from `gap_nodes()` MAY be included.
`release_gaps()` MUST NOT return an empty list.
The entry `"no AISD v1.0"` MUST always be present until `aisd_v1_tagged()` returns `true`.

---

## 6. v1.0 Gate

`aisd_v1_tagged()` returns `false` at Phase 3. The v1.0 gate requires:

| Gate item | Status at Phase 3 |
|---|---|
| All 13 realms at L3+ | Not met — `embodiment` is L1, `memory`/`planning`/`social`/`meta` are L2 |
| 0 open gaps | Not met — 8 gaps remain Open |
| `measured_families()` covers ≥6 families | Not met — 3 families at Phase 3 |
| GAIAN profile live integration | Not met — stub only |
| External audit passed | Not met — deferred to v1.0 process |

Phase 3 MUST NOT self-award v1.0. `aisd_v1_tagged()` MUST return `false`.

---

## 7. Orchestrator Boundary

An orchestrator *would* consume AISD routing. This spec does NOT ship
a live orchestrator.

| Boundary | Rule |
|---|---|
| `dispatch()` | Logging surface only; not a running agent |
| `ask_aisd()` | Gate only; not a policy enforcer |
| `capability_copy()` | Copy generation only; not a live GAIAN write |
| Live orchestrator | Deferred to Phase 4 or v1.0 process |
| Proposal queue | Not an API on main; MUST NOT be invented |

---

## 8. What This Phase Does Not Do

- Does not ship a live agent orchestrator.
- Does not write live GAIAN profiles.
- Does not close any gap node.
- Does not promote any realm to L5.
- Does not tag AISD v1.0.
- Does not use "autonomous AGI" in any copy string.

---

## 9. Acceptance Gate

- [ ] `dispatch("", _)` → `Err(InsufficientMaturity)`
- [ ] `dispatch(valid_id, L5)` → `Err(FakeLevel5)`
- [ ] `capability_copy` with "autonomous AGI" in output → `Err(Unmeasured)`
- [ ] `ask_aisd(L5, _)` → `Err(FakeLevel5)`
- [ ] `ask_aisd(claimed > actual, actual ≤ L2)` → `Err(InsufficientMaturity)`
- [ ] `release_gaps()` contains `"swe-pro-engineering: Open"`, `"calibration: Open"`, `"no AISD v1.0"`
- [ ] `release_gaps()` is non-empty
- [ ] `aisd_v1_tagged()` → `false`
- [ ] No GAIAN card from unmeasured node
- [ ] `cargo test -p gaia-aisd` green

---

## 10. Cross-References

- Phase 0: `gaia-spec/aisd/PHASE-0.md` (#122)
- Phase 1: `gaia-spec/aisd/PHASE-1.md` (#123)
- Phase 2: `gaia-spec/aisd/PHASE-2.md` (#124)
- Routing: `gaia-spec/aisd/ROUTING.md`
- Catalog: `gaia-spec/aisd/CATALOG.md`
- Evals: `gaia-spec/aisd/EVALS.md`
- Safety: `gaia-spec/aispd/` (containment cross-ref)
- Code: `gaia-aisd/src/`, `gaia-aisd/tests/`
- Issues: #122 (Phase 0 epic), #123 (Phase 1), #124 (Phase 2), #125 (this)
- Next: Skills Phase 0 (#108) — taxonomy, schema, ESCO/O*NET, graph
