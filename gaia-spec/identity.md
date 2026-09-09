# Identity and zero-trust model

**Version:** spec v0.1  
**Status:** normative policy; DID method is an open RFC

## Principals

A principal is one of: `human`, `gaian`, `agent`, `node`, `service`.

Every principal MUST have:

- a stable `principal_id` (UUID v4 is acceptable in v0.1)
- an Ed25519 key pair
- a capability list (what it may `invoke`, `observe`, `context`, `declare`)

v0.1 SHOULD mint `did:key` identifiers from the Ed25519 public key when a
DID method is not otherwise configured. A project-specific DID method is
**not** specified here (RFC-ID-001).

## Zero-trust rules

1. No implicit trust between nodes. Every cross-node operation SHOULD be
   signed (`sign`) and verified (`verify`).
2. Passwords and bearer tokens are not part of the v0.1 identity model.
   They MAY exist at an HTTP gateway as a *translation* of a capability,
   not as the source of truth.
3. Agents MUST run with the intersection of (caller capabilities, AIP
   `tool_permissions`, AIP `memory_access`).
4. Implementations SHOULD write an append-only audit record for `intent`,
   `invoke`, `declare`, and capability grants. Hash-chaining the log is
   recommended; durability is Phase 1 (#19).

## Trust levels (agents)

`experimental` < `community` < `verified`

A host MUST NOT silently upgrade a trust level. Promotion is an operator
or marketplace action.
