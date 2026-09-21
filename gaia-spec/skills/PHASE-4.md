# Skills Phase 4 — Opt-In Matching, Culture/TEK Gate, v1.0 Gate

**Status:** Listed  
**Issues:** #107 (meta), #112 (epic), #120 (this slice)  
**Crate:** `gaia-skills` (Apache-2.0)  
**Not:** Skills v1.0 (that tag does not exist yet). Not a live ESCO dump. Not ambient scoring. Not a clinical practice licence.

`skills_v1_tagged()` MUST return `false` until the v1.0 gate below is fully met and a TSC resolution is filed.  
`global_profile_dump()` MUST return `false`.  
`match_is_opt_in()` MUST return `true`.  
`tek_skill(false)` MUST return `Err(NoGrant)`.

---

## 1. Purpose

Phase 4 specifies the opt-in skill-matching surface (`SkillCard::search`),
the cultural overlay and TEK consent gate, the global-profile-dump prohibition,
and the v1.0 conditions that Skills must satisfy before it can be tagged stable.
No live ESCO dump, no ambient scoring, no TEK scrape.

---

## 2. Opt-In Skill Matching (`matching.rs`)

`SkillCard::search(query: &str, requester_id: &str) -> Result<Vec<MatchRecord>, SkillsError>`

| Rule | Constraint |
|---|---|
| Opt-in only | `match_is_opt_in()` MUST return `true` — profile only visible when user opts in |
| Unpublished hidden | `SkillCard::search` MUST NOT surface unpublished profiles — `published = false` entries are invisible |
| No ranking minors | `rank_in_marketplace(age)` MUST return `Err(MinorRankingBlocked)` for `age < 18` |
| No ambient match | MUST NOT match skill profiles without an active opt-in flag |
| No global dump | `global_profile_dump()` MUST return `false` — no bulk profile export without per-user consent |

### `MatchRecord` schema

```
match_id:       String   — unique per match result
skill_id:       String   — MUST match a Phase 1 catalog entry
requester_id:   String   — GAIAN id of the requester
candidate_id:   String   — GAIAN id of the matched user
published:      bool     — MUST be true for any record returned
cultural_flag:  bool     — true when a CulturalOverlay applies to this skill
tek_gated:      bool     — true when skill is TEK-linked and requires a grant
```

`MatchRecord.published` MUST be `true`. Any code path that returns a record
with `published = false` is a bug.

---

## 3. Cultural Overlay and TEK Gate (`tek.rs`, `overlays.rs`)

### Cultural overlays (`overlays.csv`)

Cultural variation is named in the skill catalog but is NOT a `SkillCard` field.
It is surfaced through `CulturalOverlay` structs backed by `overlays.csv`.

| Rule | Constraint |
|---|---|
| Opt-in | User must explicitly enable a cultural overlay; no auto-assign |
| No auto-assign | `cultural_overlay_auto_assigned()` MUST return `false` |
| Overlay names realms | Each overlay references a realm from `realms.csv`; orphan overlays are rejected |
| No ESCO live dump | MUST NOT perform a live ESCO API ingest — offline fixture only |

### TEK gate

| Call | Behaviour |
|---|---|
| `tek_skill(false)` | MUST return `Err(NoGrant)` — absent until community grant is present |
| `tek_skill(true)` | Returns TEK skill data only when a community grant token is present; otherwise `Err(NoGrant)` |
| TEK scrape | MUST NOT scrape country or community skill corpora — `Err(TekScrapeBlocked)` |
| TEK export without grant | MUST NOT export TEK-linked skill data without an explicit community grant — `Err(NoGrant)` |

TEK-linked skills MUST set `tek_gated = true` on any `MatchRecord`. No TEK row
is ingested without an explicit grant from the source community.

---

## 4. Global Profile Dump Prohibition

`global_profile_dump() -> bool` MUST return `false` at all phases through v1.0.

| Rule | Constraint |
|---|---|
| No bulk export | No API may return all user skill profiles in a single call |
| Per-user consent | Any profile export requires explicit per-user consent action |
| No silent sync | Skill profiles MUST NOT be synced upstream without GAIAN export consent (#65) |

---

## 5. Prohibition Surface

| Prohibition | Error |
|---|---|
| Ranking a minor in marketplace | `Err(MinorRankingBlocked)` |
| Surfacing unpublished profile in search | Bug — `published = false` records invisible |
| TEK data without community grant | `Err(NoGrant)` |
| TEK scrape | `Err(TekScrapeBlocked)` |
| Global profile dump | `global_profile_dump()` returns `false` |
| Cultural overlay auto-assign | `cultural_overlay_auto_assigned()` returns `false` |
| Ambient skill match | `match_is_opt_in()` must be `true` |
| Skills v1.0 tag before gate | `skills_v1_tagged()` returns `false` |

---

## 6. v1.0 Gate

`skills_v1_tagged()` MUST return `false` until every condition below is satisfied
and a TSC resolution is recorded in `rfcs/skills-v1.0-resolution.md`.

| Gate item | Condition |
|---|---|
| Phase 0–3 acceptance | All four acceptance gates green in CI |
| Catalog coverage | Sources: listed overlays + local vault; not a live ESCO dump |
| Child safety | `rank_in_marketplace(age < 18)` → `Err(MinorRankingBlocked)` |
| Ambient exclusion | No ambient session or scoring code path survives test suite |
| Clinical guard | `Badge::mint(_, true)` = `ClinicalCert`; no practice licence issued |
| TEK gate | `tek_skill(false)` → `Err(NoGrant)`; no TEK scrape |
| Matching gate | `match_is_opt_in()` = `true`; `global_profile_dump()` = `false` |
| TSC vote | Lazy-consensus resolution filed in `rfcs/skills-v1.0-resolution.md` |

No code path may set `skills_v1_tagged()` to `true` without the TSC resolution file present.

---

## 7. What This Phase Does Not Do

- Does not perform a live ESCO API ingest.
- Does not run ambient or background scoring.
- Does not rank minors in any marketplace.
- Does not auto-assign cultural overlays.
- Does not export TEK-linked skills without a community grant.
- Does not produce a global profile dump.
- Does not tag Skills v1.0.

---

## 8. Acceptance Gate

- [ ] `match_is_opt_in()` → `true`
- [ ] `SkillCard::search` does not surface `published = false` profiles
- [ ] `rank_in_marketplace(17)` → `Err(MinorRankingBlocked)`
- [ ] `global_profile_dump()` → `false`
- [ ] `tek_skill(false)` → `Err(NoGrant)`
- [ ] `tek_skill(true)` without community grant → `Err(NoGrant)`
- [ ] `cultural_overlay_auto_assigned()` → `false`
- [ ] `hidden_profile_api()` → `false`
- [ ] `skills_v1_tagged()` → `false`
- [ ] `cargo test -p gaia-skills` green

---

## 9. Cross-References

- Phase 0: `gaia-spec/skills/PHASE-0.md` (#108)
- Phase 1: `gaia-spec/skills/PHASE-1.md` (#109)
- Phase 2: `gaia-spec/skills/PHASE-2.md` (#110)
- Phase 3: `gaia-spec/skills/PHASE-3.md` (#111)
- v1.0 stub: `gaia-spec/skills/V1.md` (#120)
- Overlays: `gaia-spec/skills/overlays.csv`
- Realms: `gaia-spec/skills/realms.csv`
- Schema: `gaia-spec/skills/SCHEMA.md`
- Vault: `gaia-spec/skills/VAULT.md`
- HMGD TEK cross-ref: `gaia-spec/hmgd/TEK.md`
- RFC placeholder: `rfcs/skills-v1.0-resolution.md` (must not exist until gate is met)
- Issues: #107 (meta), #108 (Phase 0), #112 (epic), #120 (this slice)
