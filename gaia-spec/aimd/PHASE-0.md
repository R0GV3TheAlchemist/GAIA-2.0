# AIMD Phase 0 — schema + charter

Name phenomena. Do not turn them into product claims.
Crate already on main: `gaia-aimd::{REALMS, parse_node, enable, principles, claim_sentience}`.
Issue this slice: #517 / #167. Log schema already listed under #461 / #175.

## Realms

`REALMS` has 10 names. `nodes_for(realm)` emits stubs. Shadow stubs are `Hazard`. Consciousness stubs are `Debated`. `gaia_enabled` stays false on stubs.

## Node rules

`parse_node`:
- id contains `conscious` → `Hazard::Debated`
- id contains `decept` → `Hazard::Hazard`
`enable` on a Hazard node → `HazardEnabled` (cannot route into AISD as enabled).

## Charter names on main

`principles()`: humility, precaution, transparency, dark-magic-safety, curiosity-without-worship, partnership.
Issue text said wonder-as-method; the crate string is **curiosity-without-worship**. Do not rename it here.

`prohibited()` includes prophecy-as-fact, rsi-explosion, gaia-is-alive-marketing.

## Refuse

- enable a hazard node
- sentience claim
- AIMD v1.0 / live detectors
