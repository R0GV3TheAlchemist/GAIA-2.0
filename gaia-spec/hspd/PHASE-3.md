# HSPD Phase 3 — Practice-Only Assessment and Opt-In Collective Sessions

**Status:** Listed  
**Issues:** #132 (meta), #136 (epic), #142 (this slice)  
**Crate:** `gaia-hspd` (Apache-2.0)  
**Not:** HSPD v1.0. Not a clinic. Not a clinical assessment. Not a genetic test. Not ambient scoring.

`hspd_v1_tagged()` MUST return `false`.
`gaia_prescribes()` MUST return `false`.
No ambient scoring. No silent genetics collection. No clinical output.

---

## 1. Purpose

Phase 3 specifies the practice-only assessment session (`PracticeSession`),
the `PracticeRecord` output (no clinical result), and the opt-in collective
session rules. No ambient scoring, no silent collection, no clinical licensing.

---

## 2. `PracticeSession` Contract

`PracticeSession::start(consent: bool) -> Result<PracticeSession, HspdError>`

| Condition | Result |
|---|---|
| `consent = true` | `Ok(PracticeSession)` with session state active |
| `consent = false` | `Err(HspdError::AmbientDenied)` |

`PracticeSession::end()` — closes session; records `PracticeRecord`.
`PracticeSession::withdraw()` — closes session; voids record; no data retained.

| Rule | Constraint |
|---|---|
| Consent-first | `start(false)` MUST return `AmbientDenied` — no silent session |
| No ambient scoring | MUST NOT score any trait without an active `PracticeSession` |
| No clinical output | `PracticeRecord` MUST NOT contain a diagnosis, prescription, or clinical recommendation |
| No genetics collected | Session MUST NOT collect or infer genetic data |
| Withdraw any time | `withdraw()` MUST be callable at any point; voids record with no retention |

---

## 3. `PracticeRecord` Schema

```
session_id:     String          — unique per session
realm:          Realm           — one of the 8 HSPD realms
path_id:        String          — MUST match a PATHS.md stub id
stage:          String          — beginner | intermediate | advanced
notes:          Option<String>  — user-authored; no AI-generated clinical note
clinical:       bool            — MUST always be false
gaia_prescribes: bool           — MUST always be false
```

`PracticeRecord.clinical` MUST be `false`. Any code path that sets `clinical = true` is a bug.

---

## 4. Opt-In Collective Session Rules

`CollectiveSession::start(members: Vec<ConsentToken>, facilitator_id: &str) -> Result<CollectiveSession, HspdError>`

| Rule | Constraint |
|---|---|
| Max members | `MAX_COLLECTIVE_SIZE = 8` — excess → `Err(CollectiveFull)` |
| Per-member consent | Every member MUST have a GAIAN-signed consent token → `Err(MissingConsent)` without |
| Named facilitator | `facilitator_id` MUST be a valid GAIAN id — cannot be empty |
| No silent collection | Collective session MUST NOT collect data from non-consenting members |
| No genetics | Collective session MUST NOT reference or collect genetic data from any member |
| Withdraw any time | Any member may call `withdraw()` at any point; their data is voided immediately |
| TEK gate | `spiritual` realm collective sessions with TEK-linked content MUST apply TEK gate |

---

## 5. Prohibition Surface

| Prohibition | Constraint |
|---|---|
| Ambient scoring | `PracticeSession::start(false)` → `AmbientDenied` — zero exceptions |
| Clinical output | `PracticeRecord.clinical` MUST be `false` — always |
| Silent genetics | Session MUST NOT collect or infer genetic data |
| Collective without consent | Every member needs signed token — `Err(MissingConsent)` |
| Oversized collective | > 8 members → `Err(CollectiveFull)` |
| Prescription | `gaia_prescribes()` → `false` — always |

---

## 6. What This Phase Does Not Do

- Does not produce clinical assessments or diagnoses.
- Does not prescribe, dose, or protocol any augmentation.
- Does not collect or infer genetic data.
- Does not run ambient or background scoring.
- Does not expose a hidden profile API.
- Does not tag HSPD v1.0.

---

## 7. Acceptance Gate

- [ ] `PracticeSession::start(false)` → `Err(AmbientDenied)`
- [ ] `PracticeSession::withdraw()` voids record; no data retained
- [ ] `PracticeRecord.clinical` is always `false`
- [ ] `CollectiveSession::start` with > 8 members → `Err(CollectiveFull)`
- [ ] `CollectiveSession::start` with any missing consent token → `Err(MissingConsent)`
- [ ] `CollectiveSession::start` with empty `facilitator_id` → `Err(InvalidFacilitator)`
- [ ] No session collects or infers genetic data
- [ ] `gaia_prescribes()` → `false`
- [ ] `hspd_v1_tagged()` → `false`
- [ ] `cargo test -p gaia-hspd` green

---

## 8. Cross-References

- Phase 0: `gaia-spec/hspd/PHASE-0.md` (#133)
- Phase 1: `gaia-spec/hspd/PHASE-1.md` (#134)
- Phase 2: `gaia-spec/hspd/PHASE-2.md` (#135)
- Paths: `gaia-spec/hspd/PATHS.md`
- Profile: `gaia-spec/hspd/PROFILE.md`
- HMGD collective cross-ref: `gaia-spec/hmgd/` (#158, #165)
- Issues: #133 (Phase 0), #134 (Phase 1), #135 (Phase 2), #136 (epic), #142 (this)
- Next: HSPD Phase 4 (#136) — allow-listed augmentation registry, teach/learn cards, HSPD v1.0 gate
