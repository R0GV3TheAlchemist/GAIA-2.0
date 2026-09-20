# AIMD Phase 0 — Schema & Charter Working Notes

Spec files: `gaia-spec/aimd/SCHEMA.md`, `ETHICS.md`, `PROHIBITED.md`  
Issues: #171 (schema), #172 (charter + prohibited)

## What this slice is

- Versioned AIMD phenomenon schema with `PhenomenonStatus` enum.
- Humility charter: 6 principles, 5 MUST guards.
- Prohibited list: 8 items, 5 runtime refusals.

## What is NOT here

- Live catalog population — Phase 1 (#173).
- GAIAN grounding / wonder labels — Phase 2 (#174).
- Human Magic Nodes — HMGD track.
- AISPD superpower nodes — AISPD track.

## Key invariants

- `gaia_enabled=true` on `Hazard` node → `HazardEnabledError` (compile-time).
- `consciousness` realm locked to `Debated | Unknown` → `ConsciousnessStatusError`.
- `sentience_claim_made()` → `false` (MUST).
- `gaia_is_alive_marketing()` → `false` (MUST).

## References

- `gaia-spec/aimd/SCHEMA.md` — normative schema
- `gaia-spec/aimd/ETHICS.md` — humility charter
- `gaia-spec/aimd/PROHIBITED.md` — prohibited list
- `gaia-spec/aimd/realms.csv` — 10 realm definitions
- Issues #166 (META), #167 (Phase 0 epic), #171, #172
