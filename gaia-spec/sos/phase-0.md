# SOS Design Phase 0 — listed contracts

Write the contracts a kernel fork *would* implement. This repo does not fork a kernel.
Crate already on main: `gaia-sos::{submit_intent, HOST_CALLS, second_kernel, sos_v1_tagged, hal_tiers}`.
Issue this slice: #465 / #191. Parent META #190 stays open.

## Acceptance already on main

A prototype can submit a signed intent and be refused without the right capability:

- `submit_intent(false, true)` → `Unsigned`
- `submit_intent(true, false)` → `NoCapability`
- `submit_intent(true, true)` → `Ok`

That is the Phase-0 gate. It is not a running OS.

## 9-layer stack (names only)

Listed in `layers.csv`. Layer 9 (planetary consciousness) is **prohibited as runtime**.

## `gaia.*` host ABI

`HOST_CALLS` is the only host surface: intent, context, invoke, observe, sign, verify, declare, learn.
`learn(true)` (weight rewrite) → `WeightRewrite`.

Identity / capability *types* already exist in `gaia-sos::identity`. ECDSA+DID and dual-sign stay design-only (#389 / #392). Do not implement them here.

## WASI component names

agent, tool, sensor, knowledge, GAIAN — names in `wasi-components.csv`. Not a live component model.

## HAL tiers

`hal_tiers()` returns T0–T4 as **targets**. T0 64KB is aspirational, not a measured footprint.
`live_containerd`, `live_mcp`, `live_slurm`, `live_whisper` stay false.

## Refuse

- second kernel / Asterinas crate
- RSI / sentience / five-nines claim / SOS v1.0
- live MCP / containerd / slurm / whisper as default
