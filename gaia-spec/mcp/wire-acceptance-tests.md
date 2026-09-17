# GAIA–MCP Wire Acceptance Tests

Status: draft 0.1.0. Issue: #23. These are required integration tests; `AT-01` through `AT-06` are the minimum completion gate for the current issue.

## Test environment

Use an isolated registry containing a signed local `research.summarize` tool and an `agent.researcher` entry with an AIP manifest. Use a trusted Ed25519 test principal, an unknown principal, and a revoked principal. Capture audit events without recording plaintext fixture data.

## Required completion gate

| ID | Given / when | Then |
|---|---|---|
| AT-01 signed intent invocation | A valid intent targets registered `research.summarize`; client signs canonical `tools/call` | Server admits once; handler runs once; result has matching intent/resource IDs plus provenance, quality, freshness, and audit ID |
| AT-02 registry and AIP visibility | Registry includes `agent.researcher` with an AIP manifest | `resources/list` or equivalent GAIA registry query exposes the stable id, agent type, manifest reference, capabilities, trust, and endpoint without secret material |
| AT-03 unsigned deny | Request omits `gaia_signature` | `GAIA_UNAUTHENTICATED`; no handler invocation; denial audit event exists |
| AT-04 forged or tampered deny | Signature is invalid, or signed params are modified | `GAIA_SIGNATURE_INVALID`; no handler invocation; denial audit event exists |
| AT-05 untrusted/revoked deny | Message verifies cryptographically but key is unknown or revoked | `GAIA_IDENTITY_UNTRUSTED`; no handler invocation; denial audit event exists |
| AT-06 policy deny | Trusted caller requests resource/capability/jurisdiction not allowed by policy | `GAIA_POLICY_DENIED`; no handler invocation; audit identifies policy id/version and decision |

## Required before a live network-MCP claim

| ID | Given / when | Then |
|---|---|---|
| AT-07 external authenticated transport | Independent MCP client connects over a supported external transport | Initialize, tool discovery, and signed call succeed; unsigned call fails closed over the same transport |
| AT-08 replay protection | The same signed message nonce is sent twice within the replay window | First request may execute; second returns `GAIA_REPLAY_REJECTED`; handler executes once |
| AT-09 stream policy continuity | Admitted streaming call sends updates then policy changes or client cancels | Updates preserve correlation and policy context; prohibited updates stop; cancellation/policy event is audited |
| AT-10 discovery admission | Candidate is discovered through an implemented mechanism | It is not callable until registry validation, identity verification, and policy admission complete |

## Harness requirements

Tests MUST assert observable handler invocation counts, structured errors, and audit records. They MUST use fixed canonicalization test vectors, deterministic clocks/nonces where necessary, and no production private keys. A passing unit test of an in-process stub alone does not satisfy AT-07 through AT-10.
