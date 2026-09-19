# SOS Design Phase 3–4 — listed contracts

v1.0 is a gate, not a slogan. Stay on this repo until a TSC exists (`gaia-spec/sos/RFC.md`).
Crate already on main: `gaia-sos::{threats, formal_verify_done, sos_v1_tagged, five_nines_claimed, evaluate}`.
Issue this slice: #471 / #194. Parent META #190 stays open.

## Intent pipeline (what exists)

`evaluate(registered, consent, purpose_allowed, feature_whitelisted, payload_ok)` is the admission path already on main. Five checks, default deny. There is no sixth live stage on main — do not invent one.

## Layer-9 API catalog

Planetary-consciousness as a runtime API is **prohibited** (scale-layer R#20).
Product surfaces that *may* be named later stay listed-only and must reuse existing crate guards (Earth Twin guardian, GAIAN vault, SOS host calls). This slice does not add HTTP.

## Threat model

`threats()` already lists:

- prompt-injection
- capability-escalation
- memory-poison
- mitm
- supply-chain

Defense-in-depth here means: unsigned intent refused, no-capability refused, god coordinator refused, default-deny evaluate. Not a new scanner.

## Boot sequence

Testable *flags* already exist (`second_kernel`, `live_mcp`, `sos_v1_tagged`). They stay false. No boot daemon.

## Formal verify / v1.0 gate

- `formal_verify_done()` → false (plan, not a completed checkbox)
- `sos_v1_tagged()` → false
- `five_nines_claimed()` → false

A future v1.0 checklist must name **measured** SLOs. Marketing availability claims are refused.

## Refuse

- five-nines slogan
- fake completed formal-verify
- SOS v1.0 tag
- Layer-9 consciousness actuator
