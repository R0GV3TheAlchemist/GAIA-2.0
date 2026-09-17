# GAIA HAL tiers

Issue #196 defines honest deployment targets. These are not claims of a new kernel, production autonomy, or measured availability.

| Tier | Target | Required boundary | Explicit non-goal |
| --- | --- | --- | --- |
| T0 | Existing-OS supervisor plus tiny Wasm agent | Process sandbox, signed component bundle, explicit capability check | No GAIA kernel; no ambient authority; no autonomous actuator |
| T1 | Desktop or developer host | OS sandbox and local audit spool | No safety-critical deployment |
| T2 | Gateway or home-lab node | Device identity and mutually authenticated authority link | No cross-device ambient access |
| T3 | Managed node or cluster | Authoritative policy service and durable audit | No five-nines claim without measured SLOs |
| T4 | Federated multi-domain deployment | Explicit federation agreements and independent audit | No global GAIA control plane |

## T0 footprint

T0 is userspace-first: an existing operating system supervises a tiny component agent. `gaia-sos` adds zero kernel KLOC. The historical 64 KB goal remains aspirational until measured on a named target.

## Revocation contract

The conformance authority is single-authority and deterministic. When `revoke` returns success, it advances the authority epoch and every later decision from that authority denies the revoked capability and every descendant capability. Production ECDSA/DID, offline use, and distributed consensus remain future work.
