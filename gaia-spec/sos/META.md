# Super OS META — listed bind

Design contracts on top of #1–#32. Not a second kernel.
Crate already on main: `gaia-sos`.
Phases already listed: `phase-0.md` … `phase-3-4.md`.
Issue this slice: #511 / #190.

## Rules already on main

- `second_kernel()` / `rsi()` / `sentience()` / `five_nines_claimed()` → false.
- `sos_v1_tagged()` / `formal_verify_done()` → false.
- `live_mcp()` → false.
- `submit_intent(false, _)` → `Unsigned`.
- `HOST_CALLS` is the only host surface listed in this crate.
- `god_coordinator()` default-deny.
- `refuse_consciousness_runtime()` holds.

Asterinas remains a citation, not a crate.

## Refuse

- second kernel fork
- sentience runtime
- fake five-nines
- SOS v1.0 tag
