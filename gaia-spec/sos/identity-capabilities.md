# GAIA identity, capability, and component contracts

**Status:** Draft specification for #196.  
**Scope:** Identity vocabulary, authorization-token semantics, revocation semantics, component categories, and HAL deployment targets. This document is normative only where it uses **MUST**, **MUST NOT**, **SHOULD**, or **MAY**.

This specification extends the identity model in [`../identity.md`](../identity.md) and the package declaration model in [`../aip-manifest.md`](../aip-manifest.md). An AIP Manifest declares what a package requests; a capability token records a particular authority grant. A declaration is not a grant.

## 1. Design invariants

1. Deny by default: absence of a valid capability grant denies an operation.
2. Authority is explicit: every authorization decision is attributable to an issuer, subject, resource selector, operation, and policy version.
3. Delegation cannot amplify authority: a child grant is never broader, longer-lived, or less constrained than its parent.
4. Revocation is authoritative: after successful revocation, a conforming authorization point MUST deny the revoked grant and grants delegated from it.
5. Execution is separate from specification: this document does not claim that every runtime currently enforces these rules. Implementations MUST identify which claims they enforce.

## 2. Entity identifiers and trust roots

GAIA recognizes these entity kinds:

| Kind | Meaning | Typical trust root |
| --- | --- | --- |
| `human` | A natural person acting through a GAIA identity | User-controlled DID/key material |
| `agent` | A packaged or system software actor | Signed package identity plus deployment attestation |
| `device` | A physical or virtual execution environment | Device key/attestation where available |
| `org` | An organization that may issue policy or credentials | Organization-controlled DID/key material |
| `service` | A network or local service endpoint | Service key, deployment identity, or mutual-auth identity |

An entity identifier MUST be globally unambiguous in the implementation's trust domain and SHOULD use a DID-compatible identifier. A capability token names its `issuer`, `subject`, and optional `audience` using entity identifiers.

Identity proof, key rotation, recovery, and DID-method selection remain governed by `identity.md`; this document does not choose a DID method or cryptographic suite.

## 3. Capability token

A capability token is a signed, bounded authorization grant. It MAY be represented as a signed JSON, CBOR, or other canonical envelope, but an implementation MUST define canonical bytes before signing and verification.

### 3.1 Required claims

| Claim | Meaning |
| --- | --- |
| `token_id` | Unique, stable grant identifier |
| `issuer` | Entity that grants authority |
| `subject` | Entity authorized to act |
| `resource` | Selector for the protected resource or namespace |
| `operations` | Non-empty allowed-operation set |
| `constraints` | Bounds such as locality, memory scope, rate, time, or resource budget |
| `issued_at` | Issuance time |
| `expires_at` | Expiry time; finite by default |
| `policy_version` | Policy vocabulary/version used to interpret the token |
| `signature` | Issuer proof over canonical token bytes |

Examples of `constraints` include `local_only`, allowed MemCube identifiers or labels, host/API allowlists, maximum CPU or memory budget, rate limit, maximum delegation depth, and a required user-consent reference.

The protected resource and requested operation MUST both match before a token authorizes an action. A token MUST NOT authorize an operation merely because its subject has a matching identity or package declaration.

### 3.2 Authorization decision

Before an operation, an authorization point MUST verify all of the following:

1. The token signature is valid for the issuer's current trust material.
2. `subject`, optional `audience`, resource selector, and operation match the request.
3. Time and all constraints are satisfied.
4. The token is not revoked.
5. Every ancestor in the delegation chain is valid, unexpired, and not revoked.
6. The request does not exceed the declared policy and deployment boundary.

A failure of any check MUST deny the operation and create an auditable denial record. Runtime- or transport-specific error representations are intentionally out of scope.

## 4. Delegation

Delegation is optional. A delegating token MUST include a `parent_token_id`; implementations MUST retain enough lineage to validate the chain.

A child token MUST be a strict subset or equal subset of its parent with respect to resource scope, operation set, constraints, expiry, and remaining delegation depth. A child token MUST NOT:

- Add an operation absent from its parent.
- Broaden a resource selector.
- Remove a parent constraint.
- Expire later than its parent.
- Delegate when the parent disallows delegation or has no remaining delegation depth.

Revoking a parent revokes all descendants for authorization purposes.

## 5. Strongly consistent revocation

A `revoke(token_id, reason, actor)` request succeeds only when the authorization authority has durably recorded the token as revoked and made the revocation visible to all authorization points in the relevant trust domain.

After a successful revoke response:

- Every subsequent authorization decision in that trust domain MUST deny the token and every descendant token.
- A cache MAY be used only if it cannot authorize a token whose revocation is already successful.
- In-flight work using the revoked grant MUST receive cancellation/revocation notification at the next enforcement checkpoint. It MUST NOT begin a new protected operation with that grant after revocation succeeds.
- If an execution environment is offline or cannot receive the revocation, it MUST fail closed for new protected operations after its freshness guarantee expires; it MUST report degraded enforcement rather than claim strong consistency.
- The revocation result MUST identify the revocation epoch or equivalent monotonic version so clients can display authoritative state.

"Strongly consistent" applies to authorization decisions after successful revocation. It does not imply that already completed side effects can be undone. Implementations MUST audit the known side effects and final run state.

## 6. Audit events

Implementations MUST produce tamper-evident or append-only audit records for capability lifecycle and enforcement events. The record SHOULD include event ID, timestamp, policy version, actor, subject, token ID, parent token ID when relevant, resource selector, operation, decision, reason code, and correlation/intent ID.

Minimum event names are:

- `capability.minted`
- `capability.delegated`
- `capability.authorized`
- `capability.denied`
- `capability.revoked`
- `capability.revocation_observed`
- `capability.execution_cancelled`

Audit visibility is itself policy-controlled; audit access MUST NOT expose plaintext memory or credentials without a separate authorization grant.

## 7. WASI component categories

The following categories describe intended GAIA component roles. They do not by themselves confer capabilities.

| Category | Role | Examples |
| --- | --- | --- |
| `agent` | Plans or executes a bounded task for an intent | Researcher, planner, memory manager |
| `tool` | Performs a discrete effect or computation on request | File transform, database adapter, API bridge |
| `sensor` | Produces observations from local or authorized sources | Camera, microphone, filesystem watcher |
| `knowledge` | Retrieves, indexes, or transforms governed knowledge | MemCube retrieval, document index |
| `gaian` | Represents an artificial twin with explicit identity, memory, and policy boundaries | User-associated digital counterpart |

A package MAY declare one or more roles. The runtime MUST evaluate grants by actual requested resource and operation, not by role name. Sensors and external tools MUST use explicit locality, consent, and data-scope constraints.

## 8. HAL tier deployment targets

T0–T4 are deployment targets, not a claim that one kernel or one binary fits every environment.

| Tier | Intended footprint | Honest minimum assumption |
| --- | --- | --- |
| `T0` | Tiny/supervisor-class node | A small supervisor plus narrowly scoped agent/component; not a full GAIA kernel claim |
| `T1` | Constrained embedded or edge node | Limited local storage and a bounded component set |
| `T2` | Personal device | Local identity, policy, memory, and selected agent services |
| `T3` | Workstation or server node | Multi-component local orchestration and development workload |
| `T4` | Federated or datacenter deployment | Multiple governed nodes with explicit trust and replication boundaries |

Footprint, latency, and memory numbers require measured implementation evidence. Implementations MUST NOT present an aspirational target as a guaranteed runtime capability.

## 9. Conformance examples

### 9.1 Revoked agent use is denied

1. A human grants agent `did:gaia:agent/researcher` read access to `memcube:project/*` until a stated expiry.
2. The agent receives a valid token and reads an authorized cube.
3. The human successfully revokes the token.
4. The agent attempts another `memory.read` under the same token.
5. A conforming authorization point denies the request, emits `capability.denied`, and the running job receives revocation cancellation at its next checkpoint.

### 9.2 Delegation cannot expand

An agent token allowing `knowledge.read` for `memcube:public/*` until 12:00 with delegation depth one may delegate only a subset such as a single public cube, `knowledge.read`, with an expiry no later than 12:00 and no greater delegation depth. It cannot delegate `memory.write`, `network.connect`, private memory access, or an expiry after 12:00.

## 10. Open implementation work

This specification does not itself implement token issuance, key management, storage, distributed revocation, WASI host bindings, package signing, or scheduler cancellation. Those work items must state the subset of these requirements they enforce and add adversarial tests before claiming conformance.
