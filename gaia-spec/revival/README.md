# VirelaiOS revival intake

Status: evidence only. Not v1.0. Not a second kernel.

The 2026-09-17 transfer notes from `VirelaiOS_Master_Docs` and `VirelaiOS Stable Build` are architectural memory. They do **not** authorize executing archive scripts, importing missing CSVs, or claiming MCRE >= 97%.

Canonical predecessor map: [`LEDGER.md`](LEDGER.md) (#366).

## What is true on this repo now

- Phase 1 kernel / SFS / MemOS first cuts exist and are tested.
- Phase 2 orchestrator first cut exists: signed intent, DAG, broker, MCP AT-01..AT-06 local only.
- #195 Host/Intent ABI spec and #196 capability + HAL tiers are on main.
- `live_mcp()`, `live_owm()`, `virelai_736_claimed()`, `mcre_suite_restored()` are false.
- Autonomy default Suggest(1); levels 4–5 refused in software.

## What is not true

- 736 live modules
- Restored MCRE 2000-case suite
- Live OpenWeatherMap / MQTT connectors
- Saela HTTP APIs as production endpoints
- Consciousness or spirit as runtime state
- Execution of `reanchor_saela_corekit.sh` inside GAIA CI

See `MAPPING.md` and `LEDGER.md`.
