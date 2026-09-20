# AISPD Phase 1 first cut (#146 / #152)

Working notes. Not a live model eval. Not a safety certification.

- Every catalog node has `sources.len() >= 1` — `"fixture:benchmark"` is the valid Phase 1 stub value.
- `emergence_watch(realm)` returns `EmergenceSignal` — `Strong` MUST trigger `request_oversight`; no auto-escalation without GAIAN consent.
- `deception_watch(output)` returns `DeceptionFlag` — `Block` prevents output from reaching caller; no config flag disables this.
- Containment nodes (`agency`, `recursion`, `agi_watch`) remain `gaia_enabled = false`.
- Human-oversight protocol in `governance.rs`: `request_oversight(reason)` emits `OversightEvent` to audit log.
- No live model inference. Offline fixture patterns only.
- `aispd_v1_tagged() == false`.
- Normative spec: `gaia-spec/aispd/PHASE-1.md`.
