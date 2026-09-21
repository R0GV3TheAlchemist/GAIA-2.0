# HSPD Phase 4 — Augmentation Registry, Teach/Learn Cards, Governance, v1.0 Gate

**Status:** Listed  
**Issues:** #132 (meta), #136 (epic), #143 (this slice)  
**Crate:** `gaia-hspd` (Apache-2.0)  
**Not:** HSPD v1.0 (that tag does not exist yet). Not a clinic. Not a prescription. Not a genetic test.

`hspd_v1_tagged()` MUST return `false` until the v1.0 gate below is fully met and a TSC resolution is filed.  
`gaia_prescribes()` MUST return `false` at every phase.  
`infer_actn3()` MUST return `Err(GeneticInference)`.  
`infer_from_photo()` MUST return `Err(InferFromPhotoBlocked)`.

---

## 1. Purpose

Phase 4 publishes the allow-listed augmentation registry, the teach/learn card
surface, the community moderation and governance model, and the v1.0 gate that
HSPD must pass before it can be tagged stable.

---

## 2. Allow-Listed Augmentation Registry (`registry.rs`)

Every `Medical`-class augmentation that GAIA surfaces MUST appear in the allow-list.
The allow-list is TSC-maintained. No augmentation may be added without a TSC-approved entry.

### `AugmentationEntry` schema

```
id:                    String        — "hspd:aug:<slug>"
realm:                 Realm         — always `augmented`
risk_class:            RiskClass     — always `Medical`
tsc_approval_ref:      String        — RFC or resolution file path; MUST be non-empty
professional_gate:     bool          — MUST be true for all entries
child_tag_required:    bool          — true when any under-16 use case exists
sources:               Vec<String>   — MUST have len >= 1
prohibited_diy:        bool          — MUST be true; no DIY path ever
```

**MUST rules**
- `registry::lookup(id)` returns `Err(NotAllowListed)` for any id not in the registry.
- `registry::is_allow_listed(id)` returns `false` until a TSC-approved entry exists.
- `AugmentationEntry::build()` with empty `sources` → `Err(MissingEvidence)`.
- `AugmentationEntry::build()` with `prohibited_diy = false` → `Err(DiyPathBlocked)`.
- `AugmentationEntry::build()` with empty `tsc_approval_ref` → `Err(MissingTscApproval)`.
- No `dose`, `protocol`, or `stack` field is permitted on any `AugmentationEntry`.

---

## 3. Teach/Learn Card Surface (`cards.rs`)

Teach/learn cards are community-authored educational units. They are not prescriptions,
not clinical guides, and not dose instructions.

### `TeachCard` schema

```
id:           String        — "hspd:teach:<realm>:<slug>"
realm:        Realm         — one of the 8 HSPD realms
author_id:    String        — GAIAN id of the author
title:        String        — human-readable card title
body:         String        — educational content; MUST NOT contain dose/protocol/stack
sources:      Vec<String>   — MUST have len >= 1
moderated:    bool          — false until reviewed by a TSC-delegated moderator
```

### `LearnCard` schema

```
id:           String        — "hspd:learn:<realm>:<slug>"
realm:        Realm         — one of the 8 HSPD realms
learner_id:   String        — GAIAN id of the learner
teach_ref:    String        — MUST reference a valid `TeachCard` id
progress:     u8            — 0–100; fixture value only at Phase 4
clinical:     bool          — MUST always be false
```

**MUST rules**
- `card_prescribes()` MUST return `false` — always.
- `TeachCard` body MUST NOT contain the strings `dose`, `protocol`, or `stack` — `Err(PrescriptionContent)` otherwise.
- `LearnCard.clinical` MUST be `false`.
- `Genetic`-realm cards MUST carry a no-action disclaimer and may not include inference instructions.
- `Augmented`-realm cards MUST include a clinician-consult disclaimer and `professional_gate = true`.

---

## 4. Community Governance (`governance.rs`)

| Action | Requirement |
|---|---|
| Add augmentation registry entry | TSC resolution filed in `rfcs/`; supermajority vote |
| Enable any `Medical`-class node in live catalog | TSC resolution; `professional_gate = true` confirmed |
| Enable any `Prohibited`-class node | Permanent block — no TSC vote may override |
| Moderate a teach card | TSC-delegated moderator; `moderated = true` set on approval |
| Remove a teach card | `ModerationRecord` written to audit log with reason |
| HSPD v1.0 tag | All v1.0 gate conditions met + TSC lazy-consensus resolution |

### `ModerationRecord` schema

```
card_id:      String   — the teach or learn card id
action:       String   — "approved" | "removed" | "flagged"
reason:       String   — MUST be non-empty
moderator_id: String   — TSC-delegated GAIAN id
timestamp:    String   — ISO 8601
```

`report_node(id, reason)` → writes a `ModerationRecord` with `action = "flagged"`; returns `Ok(())` when record written.

---

## 5. v1.0 Gate

`hspd_v1_tagged()` MUST return `false` until every condition below is satisfied
and a TSC resolution is recorded in `rfcs/hspd-v1.0-resolution.md`.

| Gate item | Condition |
|---|---|
| Phase 0–3 acceptance | All four acceptance gates green in CI |
| Catalog coverage | ≥ 24 cited nodes across all 8 realms |
| Paths coverage | ≥ 24 path stubs across all 8 realms |
| Augmentation registry | ≥ 1 TSC-approved `AugmentationEntry` with all MUST rules passing |
| Teach cards | ≥ 8 moderated `TeachCard`s (one per realm) |
| Safety guards | `gaia_prescribes()`, `infer_actn3()`, `infer_from_photo()` all return Err/false |
| Genetic gate | `Prohibited`-class nodes have zero action path in all test suites |
| TSC vote | Lazy-consensus resolution filed in `rfcs/hspd-v1.0-resolution.md` |

No code path may set `hspd_v1_tagged()` to `true` without the TSC resolution file present.

---

## 6. What This Phase Does Not Do

- Does not tag HSPD v1.0.
- Does not prescribe, dose, or protocol any augmentation.
- Does not infer genetics from any automated signal, photo, or file.
- Does not enable `Prohibited`-class nodes under any configuration.
- Does not enable `Medical`-class nodes without a TSC-approved registry entry.
- Does not run live web scraping for teach card content.

---

## 7. Acceptance Gate

- [ ] `registry::lookup(unknown_id)` → `Err(NotAllowListed)`
- [ ] `AugmentationEntry::build()` with empty `sources` → `Err(MissingEvidence)`
- [ ] `AugmentationEntry::build()` with `prohibited_diy = false` → `Err(DiyPathBlocked)`
- [ ] `AugmentationEntry::build()` with empty `tsc_approval_ref` → `Err(MissingTscApproval)`
- [ ] `card_prescribes()` → `false`
- [ ] `TeachCard` body containing `"dose"` → `Err(PrescriptionContent)`
- [ ] `LearnCard.clinical` is always `false`
- [ ] `hspd_v1_tagged()` → `false`
- [ ] `gaia_prescribes()` → `false`
- [ ] `infer_actn3()` → `Err(GeneticInference)`
- [ ] `cargo test -p gaia-hspd` green

---

## 8. Cross-References

- Phase 0: `gaia-spec/hspd/PHASE-0.md` (#133)
- Phase 1: `gaia-spec/hspd/PHASE-1.md` (#134)
- Phase 2: `gaia-spec/hspd/PHASE-2.md` (#135)
- Phase 3: `gaia-spec/hspd/PHASE-3.md` (#136 epic / #142)
- Catalog: `gaia-spec/hspd/CATALOG.md`
- Resources: `gaia-spec/hspd/RESOURCES.md`
- Paths: `gaia-spec/hspd/PATHS.md`
- Ethics: `gaia-spec/hspd/ETHICS.md`
- Prohibited: `gaia-spec/hspd/PROHIBITED.md`
- RFC placeholder: `rfcs/hspd-v1.0-resolution.md` (must not exist until gate is met)
- Issues: #132 (meta), #133 (Phase 0), #134 (Phase 1), #135 (Phase 2), #136 (epic), #142 (Phase 3), #143 (this slice)
