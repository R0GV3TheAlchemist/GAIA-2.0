# VirelaiOS revival intake

Status: evidence only. Not v1.0. Not a second kernel.

Canonical predecessor map: [`LEDGER.md`](LEDGER.md) (#366).
Claim classes: [`../CLAIM_CLASSES.md`](../CLAIM_CLASSES.md) (#367).

## Promotion gate (#378)

A predecessor idea enters `main` only if it unlocks one of:

1. a schema
2. code
3. a test
4. an audit / trace event
5. a revival ledger row

This folder does not accept “implement the 12-layer stack” children.

## Hygiene (#377)

This repository is public. Revival notes may name private repos. They must not include credentials, `.env` bodies, raw conversation dumps, personal health/location detail, or how-to for refused capabilities.

## What is true on this repo now

- Phase 1 kernel / SFS / MemOS first cuts exist and are tested.
- Phase 2 orchestrator first cut exists: signed intent, DAG, broker, MCP AT-01..AT-06 local only.
- Autonomy default Suggest(1); levels 4–5 refused in software.
- `live_mcp()`, `live_owm()`, `virelai_736_claimed()`, `mcre_suite_restored()` are false.

See `MAPPING.md`, `LEDGER.md`, `RED_CI.md`.
