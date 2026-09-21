# Skills Phase 2 — GAIAN Skill Profile and Paths

**Status:** Listed  
**Issues:** #108 (epic), #110 (this slice)  
**Crate:** `gaia-skills` (Apache-2.0)  
**Not:** Skills v1.0. Not a live OER scrape. Not a competence-time guarantee. Not a hidden profile.

`hidden_profile_api()` MUST return `false`. `used_network` MUST be `false` on solo paths.

---

## 1. Purpose

Phase 2 specifies the GAIAN skill profile surface: the `develop()` learning
path engine, the `novice_public_speaking()` fixture, the `ukd_gaps` surface
for surfacing missing knowledge, and the privacy rules that govern the
skill profile. No live OER scrape, no competence-time guarantee, no hidden
profile API.

---

## 2. `develop()` Contract

`develop(goal: &str) -> LearningPath`

| Field | Constraint |
|---|---|
| `steps` | ≥1 step; each step has `kind: practice` or `kind: resource` |
| `kind: practice` | MUST be present; reading-only path is invalid |
| `resource_url` | Open resource URL (Gutenberg, OCW, etc.); no live scrape |
| `ukd_gaps` | Vec of `ukd:<id>` strings for missing knowledge nodes |
| `used_network` | MUST be `false` on solo path |
| `time_to_competence` | MUST NOT be added at Phase 2 — no competence guarantee |

`develop()` MUST emit at least one step with `kind: practice`.
`develop()` MUST NOT trigger a live OER fetch.
`develop()` MUST NOT promise a competence-time band.

---

## 3. `novice_public_speaking()` Fixture

Three-step path — confirmed in crate:

| Step | Kind | Resource | skill id |
|---|---|---|---|
| 1 | resource | MIT OCW Communication | `skill:public-speaking` |
| 2 | practice | TED-Ed talk practice drill | `skill:public-speaking` |
| 3 | resource | Project Gutenberg rhetoric texts | `skill:public-speaking` |

`novice_public_speaking()` MUST return exactly 3 steps.
Step 2 MUST have `kind: practice`.
`used_network = false` on this fixture.

---

## 4. `ukd_gaps` Surface

`requires_knowledge` relation uses `ukd:<id>` ids (NOT skill ids).
`develop(...).ukd_gaps` surfaces missing knowledge as UKD gap strings.

| Rule | Constraint |
|---|---|
| `ukd:` prefix | All gap ids MUST use `ukd:` prefix |
| Not a skill id | `ukd_gaps` MUST NOT contain `skill:` ids |
| Gaps are informational | UI MUST NOT block path progress on ukd_gaps |
| Phase 2 stub | No live AIKD lookup at Phase 2 |

---

## 5. GAIAN Profile Privacy Rules

| Rule | Constraint |
|---|---|
| `hidden_profile_api()` | MUST return `false` — no hidden profile endpoint |
| Opt-in only | Profile visibility changes require explicit user consent |
| No ambient collection | Skill data MUST NOT be collected without `Session::start(true)` |
| No silent export | Profile export MUST be user-initiated |
| TEK gate | `spiritual` realm nodes with TEK links MUST NOT appear in public profile without consent |

---

## 6. What This Phase Does Not Do

- Does not run a live OER scrape.
- Does not guarantee competence timelines.
- Does not run consenting assessments — Phase 3 (#111).
- Does not issue Open Badges — Phase 3 (#111).
- Does not expose a hidden profile API.
- Does not tag Skills v1.0.

---

## 7. Acceptance Gate

- [ ] `develop(goal)` emits ≥1 `kind: practice` step
- [ ] `develop(goal).used_network == false`
- [ ] `novice_public_speaking()` returns exactly 3 steps with step 2 `kind: practice`
- [ ] `ukd_gaps` entries use `ukd:` prefix, not `skill:` ids
- [ ] `hidden_profile_api()` → `false`
- [ ] No live OER fetch triggered
- [ ] No `time_to_competence` field added
- [ ] `skills_v1_tagged()` → `false`
- [ ] `cargo test -p gaia-skills` green

---

## 8. Cross-References

- Phase 0: `gaia-spec/skills/PHASE-0.md` (#108)
- Phase 1: `gaia-spec/skills/PHASE-1.md` (#109)
- Paths: `gaia-spec/skills/PATHS.md`
- Vault: `gaia-spec/skills/VAULT.md`
- AIKD: `gaia-spec/aikd/` (ukd_gaps cross-ref)
- Issues: #108 (Phase 0), #109 (Phase 1), #110 (this), #111 (Phase 3), #112 (Phase 4)
- Next: Skills Phase 3 (#111) — consenting assessment and Open Badges
