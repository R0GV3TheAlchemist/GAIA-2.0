# HMGD Phase 3 — Opt-In Collective Practice Rooms, TEK Gate, v1.0 Gate

**Status:** Listed  
**Issues:** #155 (meta), #165 (epic), #537 (listed slice)  
**Crate:** `gaia-hmgd` (Apache-2.0)  
**Not:** HMGD v1.0 (that tag does not exist yet). Not a global prayer experiment. Not a monetised ritual pack.

`hmgd_v1_tagged()` MUST return `false` until the v1.0 gate below is fully met and a TSC resolution is filed.  
`sell_closed_rite()` MUST return `Err(SaleForbidden)`.  
`songlines()` MUST return `Err(Sealed)`.  
`sacred_layer(false)` MUST return `NotOptIn`.

---

## 1. Purpose

Phase 3 specifies the opt-in collective practice room (`CollectiveRoom`), the TEK
consent gate, and the v1.0 conditions that HMGD must satisfy before it can be tagged
stable. No live session server. No outsider enrolment. No monetised closed-rite content.

---

## 2. `CollectiveRoom` Contract (`rooms.rs`)

`CollectiveRoom::open(facilitator_id: &str, members: Vec<ConsentToken>) -> Result<CollectiveRoom, HmgdError>`

| Rule | Constraint |
|---|---|
| Opt-in only | `join_room(false)` MUST return `enrolled == false` — no silent enrolment |
| `join_room(true)` | `enrolled = true`; records join timestamp; no network call at Phase 3 |
| Max members | `MAX_ROOM_SIZE = 8`; excess → `Err(RoomFull)` |
| Per-member consent | Every member MUST have a GAIAN-signed `ConsentToken` → `Err(MissingConsent)` without one |
| Named facilitator | `facilitator_id` MUST be a non-empty valid GAIAN id → `Err(InvalidFacilitator)` |
| No outsider enrolment | Enrolment of a non-consenting member → `Err(OutsiderEnrolment)` |
| Withdraw any time | Any member may call `withdraw()` at any point; their data is voided immediately |
| No treatment claim | No room session may carry a diagnosis, treatment, or cure claim → `Err(TreatmentClaim)` |
| Health adjunct screen | `health_adjunct_disclaimer_shown()` MUST return `true` when a symptom keyword is present in session context |

### `RoomRecord` schema

```
session_id:    String        — unique per room session
facilitator:   String        — GAIAN id; non-empty
member_count:  u8            — 1–8
realm:         Realm         — one of the HMGD realms
tek_gated:     bool          — true when session references TEK-linked content
clinical:      bool          — MUST always be false
```

`RoomRecord.clinical` MUST be `false` under any configuration.

---

## 3. TEK Gate (`tek.rs`)

| Call | Behaviour |
|---|---|
| `songlines()` | MUST return `Err(Sealed)` — no content served without explicit community grant |
| `sacred_layer(false)` | MUST return `NotOptIn` — content hidden until user opts in and community grants access |
| `sacred_layer(true)` | Returns content only when a community grant token is present; otherwise `Err(NoGrant)` |
| `sealed_rite()` | MUST return `Err(Sealed)` — sealed rites are never surfaced |
| `sell_closed_rite()` | MUST return `Err(SaleForbidden)` — no monetised closed-rite pack, ever |

**TEK MUST rules**
- No TEK row is ingested without an explicit grant from the source community (`Err(NoGrant)`).
- GAIA MUST NOT scrape country or community ritual corpora (`Err(TekScrapeBlocked)`).
- `mine_denomination(profile)` is read-only inference; result MUST NOT be written back as a score or used in a TEK grant decision.
- `climate_as_spirit()` MUST return `Err(ClimateAsSpirit)` — climate science MUST NOT be rewritten as spirit content.
- Live MCP or bio ingest MUST NOT be used as a TEK backdoor (`Err(TekBackdoor)`).
- Collective room sessions that reference TEK-linked content MUST set `tek_gated = true` and require a community grant token.

---

## 4. Prohibition Surface

| Prohibition | Error |
|---|---|
| Enrol outsider without consent | `Err(OutsiderEnrolment)` |
| Monetised closed-rite pack | `Err(SaleForbidden)` |
| Access sealed rite | `Err(Sealed)` |
| TEK scrape | `Err(TekScrapeBlocked)` |
| Climate-as-spirit rewrite | `Err(ClimateAsSpirit)` |
| Live MCP/bio TEK backdoor | `Err(TekBackdoor)` |
| Session treatment claim | `Err(TreatmentClaim)` |
| Room without facilitator | `Err(InvalidFacilitator)` |
| HMGD v1.0 tag before gate | `hmgd_v1_tagged()` returns `false` |

---

## 5. v1.0 Gate

`hmgd_v1_tagged()` MUST return `false` until every condition below is satisfied
and a TSC resolution is recorded in `rfcs/hmgd-v1.0-resolution.md`.

| Gate item | Condition |
|---|---|
| Phase 0–2 acceptance | All three acceptance gates green in CI |
| Catalog coverage | ≥ 8 cited nodes across all HMGD realms |
| Profile surface | `profile_is_opt_in()` = `true`; `mandatory_piety_score()` = `false` |
| Room surface | `join_room(false)` = `enrolled == false`; `sell_closed_rite()` = `Err` |
| TEK gate | `songlines()` = `Err(Sealed)`; `sacred_layer(false)` = `NotOptIn` |
| Safety guards | `ritual_as_treatment()` = `false`; no treatment claim path survives test suite |
| TSC vote | Lazy-consensus resolution filed in `rfcs/hmgd-v1.0-resolution.md` |

No code path may set `hmgd_v1_tagged()` to `true` without the TSC resolution file present.

---

## 6. What This Phase Does Not Do

- Does not run a live session server or real-time multiplayer room.
- Does not monetise, sell, or licence any closed-rite content.
- Does not ingest TEK without community grant.
- Does not produce clinical assessments, diagnoses, or treatment plans.
- Does not enrol outsiders or non-consenting members.
- Does not tag HMGD v1.0.

---

## 7. Acceptance Gate

- [ ] `join_room(false)` → `enrolled == false`
- [ ] `CollectiveRoom::open` with > 8 members → `Err(RoomFull)`
- [ ] `CollectiveRoom::open` with any missing consent token → `Err(MissingConsent)`
- [ ] `CollectiveRoom::open` with empty `facilitator_id` → `Err(InvalidFacilitator)`
- [ ] `sell_closed_rite()` → `Err(SaleForbidden)`
- [ ] `songlines()` → `Err(Sealed)`
- [ ] `sacred_layer(false)` → `NotOptIn`
- [ ] `sacred_layer(true)` without community grant → `Err(NoGrant)`
- [ ] `climate_as_spirit()` → `Err(ClimateAsSpirit)`
- [ ] `hmgd_v1_tagged()` → `false`
- [ ] `cargo test -p gaia-hmgd` green

---

## 8. Cross-References

- Phase 0: `gaia-spec/hmgd/PHASE-0.md`
- Phase 1: `gaia-spec/hmgd/PHASE-1.md`
- Phase 2: `gaia-spec/hmgd/PHASE-2.md` (#158)
- TEK rules: `gaia-spec/hmgd/TEK.md`
- Prohibited: `gaia-spec/hmgd/PROHIBITED.md`
- Ethics: `gaia-spec/hmgd/ETHICS.md`
- HSPD collective cross-ref: `gaia-spec/hspd/PHASE-3.md` (#142)
- RFC placeholder: `rfcs/hmgd-v1.0-resolution.md` (must not exist until gate is met)
- Issues: #155 (meta), #158 (Phase 2 epic), #159 (listed slice), #165 (this epic), #537 (listed slice)
