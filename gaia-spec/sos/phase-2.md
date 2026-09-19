# SOS Design Phase 2 — listed contracts

Emergent coordination. Pull-based, signed, no inbound-port religion on edge.
Crate already on main: `gaia-sos::{discover, god_coordinator, evaluate}`.
Issue this slice: #469 / #193. Parent META #190 stays open.

## Acceptance already on main

Two nodes can discover a capability from an intent without a pre-baked team roster:

- `discover(true)` → `"intent-discover-capsule-result"`
- `discover(false)` → `"none"`
- `god_coordinator()` → `GodCoordinator`

That is the Phase-2 gate. It is not a running cluster.

## ColonyOS notes (research citation)

Task broker + blueprint reconciliation are **notes vs ColonyOS**, not a vendor integration.
No live containerd. `live_containerd()` stays false.

## EACN

Discovery / negotiate / adjudicate are names in `eacn.csv`.
Only `discover` is implemented as a label. Negotiate and adjudicate are not APIs on main — do not invent them.

## Registries as OS services (names only)

Earth Twin + GAIAN connectors are listed names, not HTTP services.
Admission already default-denies via `evaluate` unless registry, consent, purpose, whitelist, and payload all pass.

## Consistency

AP default. Strong consistency is required only for capability revoke (identity types already on main). This slice does not add a consensus crate.

## Refuse

- god coordinator
- live ColonyOS / MCP / containerd / slurm
- second kernel / SOS v1.0
- inbound-port requirement on edge nodes
