# Anti-Chaos Control Plane

**Canon:** C01, C30, C32, C34, C42  
**Proof:** PROOF_ANTI_CHAOS_001  
**Supabase project:** `gaia-2-0` (`yylqoiqobydrdsnnulip`)  
**Status:** Live schema applied 2026-09-17

Preventing chaos does not mean freezing GAIA. It means holding the system at the **edge of chaos**: branching ratio σ ≈ 1, QRC Thouless ratio τ ∈ [0.5, 1.5], and issue/ontology growth that cannot avalanche without an epic parent and a proof.

## Four coupled locks

| Lock | Plane | Mechanism |
|---|---|---|
| Ontology | GitHub canon + Supabase `canon_nodes`/`canon_edges` | Requires/supersedes graph; conflict check; no canon without proof |
| Work avalanche | GitHub issues + `work_items` | Epic parent required; `detect_system_gaps()` locks when OPEN > 50 |
| Criticality | `criticality_snapshots` | σ ∈ [0.95, 1.05]; QRC η ∈ [0.45, 0.52]; hardware substrates tagged |
| Runtime | Self-healing + `gap_locks` | OPEN circuit breakers and unresolved gaps block new execution |

## Supabase objects

- `canon_nodes`, `canon_edges` — CanonGraph persistence
- `trace_events` — GAIATrace ingest
- `task_runs`, `task_nodes` — TaskGraph persistence
- `criticality_snapshots` — SOC/QRC/hardware order parameters
- `work_items` — GitHub issue mirror with avalanche weight
- `system_gaps`, `gap_locks` — live gap detection and 24h locks
- `detect_system_gaps()` — issue avalanche, orphan issues, criticality drift, open circuits, empty canon
- `classify_branching_ratio(sigma)`, `classify_qrc_phase(tau, eta)`, `compute_overall_phi(...)`

`overall_phi = 0.35·classical_soc_phi + 0.30·qrc_phi + 0.20·schumann_alignment + 0.15·noospheric_coherence`

## GitHub operating rules

1. New issues use the anti-chaos template and declare a parent epic.
2. Canon PRs must include a `## Proof` block and a `proofs/PROOF_*.md` artefact.
3. Supabase DDL lives only in `supabase/migrations/` and must match the live project.
4. Do not open more than 50 concurrent non-epic issues without ingesting them into `work_items` and running `detect_system_gaps()`.

## Hugging Face

Publish `gaia-criticality` snapshots (branching ratio, Thouless ratio, η, substrate) as a versioned dataset with a dataset card pointing back to this document and PROOF_ANTI_CHAOS_001. No Hub write API is available on the current OAuth grant; files live under `huggingface/gaia-criticality/` until upload.

## Hardware SOC

`criticality_substrate` enum: `software | memristor | qrc | geophysical | neural`. Locally active memristors and QRC reservoirs are first-class substrates, not metaphors. Drift off the edge is a `criticality_drift` lock, not a log line.
