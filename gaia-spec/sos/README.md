# GAIA Super OS specifications

This directory contains userspace-first Super OS design contracts. Issue #195 defines the GAIA host ABI and canonical Intent ABI.

## Issue #195 files

| File | Purpose |
|---|---|
| `abi.md` | Normative GAIA host ABI and security invariants |
| `intent.schema.json` | JSON Schema draft 2020-12 for the signed Intent record |
| `amazon-tipping.example.json` | Structurally valid, read-only Earth Twin research intent |
| `unsigned-intent.invalid.json` | Negative fixture missing `signature`; MUST fail schema validation |
| `identity-capabilities.md` | Existing identity and capability contract referenced by the ABI |

## Validation contract

Schema validation is necessary but not sufficient for execution. A structurally valid fixture may contain a deliberately non-cryptographic test signature. The runtime MUST then verify canonical payload hashing, the signature, key binding, time window, nonce/replay state, grant status, revocation, consent, target declaration, and policy before dispatch.

Expected results:

1. `amazon-tipping.example.json` validates structurally against `intent.schema.json`.
2. `unsigned-intent.invalid.json` fails validation because `signature` is required.
3. A runtime submission of an unsigned intent returns `GAIA_INTENT_SIGNATURE_REQUIRED` with `handler_invocations: 0`.
4. A structurally valid fixture with a placeholder signature MUST fail cryptographic verification until a test keypair produces a real signature.
5. A validly signed intent requesting a capability outside its grant, AIP Manifest, policy, consent, or jurisdiction bound MUST return `GAIA_CAPABILITY_DENIED` before target dispatch.
6. `gaia.learn.append` creates a signed append-only experience/MemCube record and cannot modify model weights, grants, policy, manifests, or identity.

## Suggested test progression

1. Add a JSON Schema validator in the userspace test harness.
2. Add a test that validates `amazon-tipping.example.json`.
3. Add a negative test that rejects `unsigned-intent.invalid.json`.
4. Add test-only Ed25519 keys, replace the placeholder fixture signature at test time, and verify the canonical hash/signature binding.
5. Instrument the target handler and assert count `0` for every admission denial.
6. Add grants, revocation, replay, confirmation, data-class, and target-manifest tests incrementally.

## Scope boundary

This specification does not change MCP transport or discovery behavior from Issue #23. It provides the signed Intent contract that those layers can consume once independently implemented.
