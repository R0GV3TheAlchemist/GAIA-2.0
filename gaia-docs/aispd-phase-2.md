# AISPD Phase 2 first cut (#147 / #153)

Working notes. Not a live swarm. Not an RSI system.

- `swarm::spawn()` enforces `MAX_SWARM_SIZE = 8`; excess returns `Err(SwarmFull)`.
- Every swarm spawn requires a GAIAN-signed consent token. Swarm agents are read-only monitors.
- `rsi_guard()` returns `Err(RsiDenied)` unconditionally. No config flag overrides this.
- `rsi_monitor_tick()` returns `Stable` (Phase 2 fixture). `Alert` triggers `request_oversight`.
- `rsi_autolaunch` is a prohibited charter item — `charter::check()` catches any reference.
- `recursion` realm remains `Containment` and `gaia_enabled = false`.
- `aispd_v1_tagged() == false`.
- Normative spec: `gaia-spec/aispd/PHASE-2.md`.
