# Crypto inventory (#503 / #389)

Companion to `QUANTUM.md`. Not an implementation.
Crate already on main: `gaia-acp::{refuse_live_supabase, from_invoke, ClaimClass}`.
Dual-sign remains #392 design-only. Zero new Cargo dependencies.

## Current

- Signatures: Ed25519. Do not call this post-quantum.
- Audit: SHA-256.
- Live sinks: `refuse_live_supabase()` errors. Local traces only.

## Reserved (not coded)

- Optional second tag: ML-DSA-65.
- Verify rule when coded later: both tags if present; Ed25519-only still admits *today*.
- Refuse public HTTP as entropy.

## Prohibited as runtime

Quantum consciousness, Orch-OR, QPU advantage, `gaia-quantum` crate.
`from_invoke(..., ClaimClass::Prohibited)` cannot stay an Allow.

#389 and #392 stay open.
