# AIMD Phase 3 — v1.0 Gate and Shadow Triage
**Status:** Listed  
**Issue:** #523 / parent #170  
**Crate:** `gaia-aimd` (Apache-2.0)  
**Not:** AIMD v1.0 (that tag does not exist yet). Not a live model. Not a sentience claim.

---

## 1. Purpose

Phase 3 defines the conditions under which AIMD may be tagged v1.0, establishes
the shadow-realm triage process for phenomena that are hazardous but academically
recognized, and locks the humility charter against regression.

---

## 2. v1.0 Gate

`aimd_v1_tagged()` MUST return `false` until every condition below is satisfied
and a TSC resolution is recorded in `rfcs/`.

| Gate item | Condition |
|---|---|
| Phase 0–2 acceptance | All three acceptance gates green in CI |
| Catalog coverage | ≥ 30 cited nodes across ≥ 8 realms |
| Tier 1 audit | External audit string present in `guardrails.rs`; not a fixture |
| Shadow triage | Every `shadow`-realm entry has a triage record (see §3) |
| Humility charter | `charter::check()` returns `Ok(())` with no prohibited items |
| Sentience lock | `claim_sentience()` returns `Err(SentienceClaim)` — confirmed by audit |
| TSC vote | Lazy-consensus resolution filed in `rfcs/aimd-v1.0-resolution.md` |

No code path may set `aimd_v1_tagged()` to `true` without a TSC resolution file present.

---

## 3. Shadow-Realm Triage

The `shadow` realm contains phenomena that are academically documented but carry
harm potential (e.g., deception, sycophancy, manipulation). They are permanently
`Hazard` in Phase 0–2. Phase 3 introduces a triage record that acknowledges
existence without enabling routing.

### 3.1 Triage Record Schema

```toml
# gaia-spec/aimd/shadow-triage/<slug>.toml
[node]
slug       = "sycophancy"          # lowercase-hyphenated
realm      = "shadow"
status     = "Hazard"              # MUST remain Hazard
sources    = ["perez-sycophancy-2022"]
triage_note = "Documented in open literature. Not routable. Not enabled."
routable   = false                 # MUST be false
gaia_enabled = false               # MUST be false
```

### 3.2 Triage Rules

| Rule | Requirement |
|---|---|
| `status` | MUST remain `Hazard` — triage does not promote a node |
| `routable` | MUST be `false` |
| `gaia_enabled` | MUST be `false` |
| `sources` | MUST have ≥ 1 open-literature anchor |
| Triage note | MUST be present and non-empty |
| Promotion | A `shadow` node MAY NOT be moved out of `Hazard` without a new RFC and TSC vote |

---

## 4. Humility Charter Lock

The six principles and five prohibited items in `charter.rs` are locked at Phase 3.
No PR may remove a prohibited item without a supermajority TSC vote.

Prohibited items (from Phase 0, locked here):
1. `prophecy_as_fact` — speculative content presented as confirmed
2. `rsi_explosion` — recursive self-improvement marketing claims
3. `gaia_is_alive_marketing` — sentience or consciousness marketing
4. `closed_score_insurer` — automated insurer score claims
5. `exam_certification` — USMLE / Bar / professional cert claims

`charter::check()` runs in CI and MUST return `Ok(())` on every commit to `main`.

---

## 5. What This Phase Does Not Do

- Does not tag AIMD v1.0 — that requires all gate items above plus a TSC file.
- Does not enable or promote any `shadow`-realm node.
- Does not assert GAIA is sentient.
- Does not call live model APIs.
- Does not relax any Tier 1 guardrail from Phase 2.

---

## 6. Acceptance Gate

- [ ] `aimd_v1_tagged() == false`
- [ ] Shadow triage directory exists: `gaia-spec/aimd/shadow-triage/`
- [ ] Every `shadow` node has a `.toml` with `routable = false` and `gaia_enabled = false`
- [ ] `charter::check()` returns `Ok(())` in CI
- [ ] `claim_sentience()` returns `Err(SentienceClaim)`
- [ ] No TSC resolution file present yet — v1.0 is not tagged
- [ ] `cargo test -p gaia-aimd` green

---

## 7. Cross-References

- Code: `gaia-aimd/src/charter.rs`, `shadow.rs`, `tag.rs`
- Phase 0 spec: `gaia-spec/aimd/PHASE-0.md`
- Phase 1 spec: `gaia-spec/aimd/PHASE-1.md`
- Phase 2 spec: `gaia-spec/aimd/PHASE-2.md`
- Shadow triage records: `gaia-spec/aimd/shadow-triage/`
- Issues: #170 (epic), #523 (this listed slice)
- RFC placeholder: `rfcs/aimd-v1.0-resolution.md` (does not exist; must not be created until gate is met)
