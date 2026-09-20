# HMGD Phase 2 — Working Notes

Spec: `gaia-spec/hmgd/PHASE-2.md`  
Issues: #531 / #158 / #164

## What this phase is

- Opt-in GAIAN practice log. User declares practices; GAIA does not infer them from chat.
- `HmgdProfile` already on main. This slice formalises the surface rules.
- Empty profile is fine. GAIAN works with no practices declared.

## What is NOT here

- No belief score shown to the user.
- No chat mining for denomination.
- No piety gate blocking GAIAN.
- No ritual-as-treatment claims.
- No child entries without guardian approval.
- No auto-generated reminder text.
- No live interfaith reconciliation engine.
- No server-side sync (local only until #65 export consent).

## Key guards

- `chat_mined_for_denomination()` → `false` (MUST)
- `mandatory_piety_score()` → `false` (MUST)
- `ritual_as_treatment()` → `false` (MUST)
- `child_profile_guardian_gate()` → `GuardianApproved | Refused` (MUST)
- `health_adjunct_disclaimer_shown()` → `true` when symptom keyword present (MUST)

## References

- `gaia-spec/hmgd/PHASE-2.md` — normative
- `gaia-spec/hmgd/PROHIBITED.md` — prohibited list
- `gaia-spec/hmgd/TEK.md` — TEK rules
- Issue #65 (GAIAN export/delete), #158 (epic), #164 (profile log)
