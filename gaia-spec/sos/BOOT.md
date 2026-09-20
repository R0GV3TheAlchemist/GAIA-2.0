# Boot stages + L9 map + threats

Names only. Not a boot daemon. Not QEMU.
Crate already on main: `gaia-sos::{threats, live_mcp, sos_v1_tagged, HOST_CALLS}`.
Issue this slice: #527 / #199. Phase 3–4 already lists the same threat names.

## Threats

`threats()`: prompt-injection, capability-escalation, memory-poison, mitm, supply-chain.
Emergency stop is the existing SI/AISPD refuse path — not a planet off-switch.

## L9 prefixes (owner issues, not HTTP)

| Prefix | Owner |
| --- | --- |
| earth | Earth Twin / #33 |
| gaian | #57 / #141 vault |
| knowledge | UKD #77 / AIKD #93 |
| agent | AISD #121 |
| infra | scale layers #451 |
| governance | #200 |

This slice does not add routes. `HOST_CALLS` remains the host surface.

## Boot flags already on main

`second_kernel`, `live_mcp`, `sos_v1_tagged` stay false. That is the harness.

## Refuse

- QEMU product
- L9 consciousness actuator
- SOS v1.0 tag
