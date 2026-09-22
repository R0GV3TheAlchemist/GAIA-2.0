# GAIA Host ABI and Intent ABI

**Status:** Frozen v1.0  
**Issue:** #715  
**Scope:** Userspace host ABI and canonical Intent record. This document does not add kernel syscalls, does not grant authority, and does not alter MCP transport behavior.

## 0. Architecture Decision — Path C (userspace-only declaration)

**Decision date:** 2026-09-21  
**Resolves:** #718, #713  
**Decision:** Path C — Explicit userspace declaration.

GAIA does **not** implement a kernel in the OS sense. There is no ring-0 boundary, no custom kernel module, no CPU privilege level transition, and no hardware address-space management. The layer previously called "L1 kernel" is formally renamed **GAIA Runtime**.

| Old term | New term | Notes |
|---|---|---|
| `gaia-kernel` crate | `gaia-kernel` crate (kept for historical reasons) | Internal crate name unchanged to avoid churn; public-facing docs use "GAIA Runtime" |
| L1 kernel | GAIA Runtime | A privileged-userspace executor and policy enforcement layer |
| "kernel syscall" | GAIA Runtime call / host call | Dispatch handled entirely in userspace via the host ABI defined below |

### What GAIA Runtime IS
- A userspace executor with capability-enforced dispatch.
- A policy and audit boundary between agents, tools, and the host OS.
- An intent-lifecycle manager (submit → admit → plan → invoke → observe → learn).
- A WASM sandbox host (via `wasmtime`) for agent isolation.

### What GAIA Runtime is NOT
- Not a kernel module (`.ko`).
- Not a ring-0 / EL1 privilege boundary.
- Not an OS kernel fork.
- Not a syscall interceptor at the hardware level.

All "syscall" numbers in Section 12 of this document are **GAIA Runtime call numbers** dispatched in userspace. They are not Linux syscall numbers and do not require kernel modifications.

---

## 1. Purpose

GAIA is an intent-centric system. The Intent ABI is the common, signed request boundary used by local executors, planners, agents, CLI/API surfaces, memory services, and later protocol adapters. An intent describes a requested outcome and its boundaries; it never grants authority by itself.

The host MUST validate an intent before planning, context resolution, network access, tool selection, or handler execution. Default policy is deny.

## 2. Normative language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHOULD**, **SHOULD NOT**, and **MAY** are normative.

## 3. Security invariants

1. An intent without a valid signature MUST be denied.
2. Schema validation, freshness checks, replay checks, signature verification, grant resolution, revocation checks, consent checks, and policy evaluation MUST occur before dispatch.
3. A denial MUST result in zero target-handler invocations.
4. An Intent record MUST NOT be treated as a capability grant.
5. Effective authority is the intersection of requested scope, subject grant, target AIP Manifest, host policy, resource policy, consent state, and jurisdiction restrictions.
6. A declared tool or agent capability is descriptive only and MUST NOT confer authority.
7. `gaia.learn.append` MUST append a signed experience record; it MUST NOT modify model weights, policy, grants, manifests, identity, or authorization.
8. Privacy, data-minimization, local-default, audit, and revocation rules remain in force at every operation.

\[
\text{effective\_authority} =
\text{requested\_scope}
\cap \text{subject\_grant}
\cap \text{target\_manifest}
\cap \text{host\_policy}
\cap \text{resource\_policy}
\cap \text{consent}
\cap \text{jurisdiction}
\]

If the resulting set is empty, the host MUST deny the request.

## 4. Intent lifecycle

1. **Construct:** a human, agent, device, service, or organization creates an Intent record.
2. **Sign:** the authorized signer signs canonical payload bytes. Private key material MUST remain outside the record.
3. **Submit:** `gaia.intent.submit` receives the record.
4. **Admit or deny:** the host validates schema, time window, nonce, signature, grant, revocation, consent, and policy.
5. **Plan:** only an admitted intent can be converted to a task graph.
6. **Resolve context:** only the subset of MemCubes and data allowed by effective authority may be returned.
7. **Invoke:** the coordinator may invoke an allowlisted target only within the intent bounds.
8. **Observe:** signed/auditable progress and result events remain correlated to `intent_id`.
9. **Learn:** a separately authorized learning action appends a provenance-bearing experience MemCube.

## 5. Canonicalization and signatures

- The signed payload is the Intent object with `signature.value` omitted.
- Implementations MUST use a deterministic canonical JSON representation before hashing and signing. The production canonicalization profile MUST be named and versioned before cross-language interoperability is claimed.
- `signature.payload_hash` MUST equal the SHA-256 digest of those canonical payload bytes.
- The verifier MUST resolve `signature.key_id` to a current verification key bound to the subject or a valid delegated signer.
- The verifier MUST reject unsupported algorithms, unknown keys, invalid signatures, payload-hash mismatches, expired keys, revoked keys, and invalid delegation.
- Test fixtures may use non-cryptographic placeholder signatures only where a test explicitly asserts structural schema validation. They MUST NOT be accepted by cryptographic verification.

## 6. Host operations

### 6.1 `gaia.intent.submit`

**Input:** an Intent ABI record conforming to `intent.schema.json`.

**Behavior:** validate and either admit the intent for planning or return a structured denial.

**MUST check, in order before dispatch:**

1. Payload is well-formed and schema-valid.
2. `issued_at` and `expires_at` are valid, with `expires_at > issued_at`.
3. Current time falls in the allowed validity window.
4. `nonce` has not already been accepted for the signing authority in its validity window.
5. Signature algorithm is supported; hash and signature are valid.
6. Signer identity and any delegation chain are valid.
7. `authorization.grant_id` is current, unrevoked, and bound to the subject.
8. Requested capabilities, target, data classes, network policy, and side-effect class are allowed.
9. Required consent and confirmation receipts are present and valid.

**Success:** `IntentAccepted { intent_id, effective_authority, audit_event_id }`

**Denial:** `IntentDenied { intent_id?, code, message, audit_event_id, handler_invocations: 0 }`

### 6.2 `gaia.context.resolve`

**Input:** admitted `intent_id`, requested MemCube references, and a purpose declaration.

**Behavior:** returns only context permitted by the effective authority and applicable data controls. It SHOULD prefer minimum necessary references, summaries, or redacted views.

**MUST NOT:** return raw private, sealed, restricted, or biometric-forbidden data without a matching grant and consent state.

### 6.3 `gaia.invoke`

**Input:** admitted `intent_id`, target declaration, arguments, and proposed side-effect class.

**Behavior:** invokes a tool, agent, executor, or workflow only if it is allowlisted by the target declaration and the effective authority.

**MUST:**

- Enforce capability, resource, time, network, data-class, and side-effect constraints.
- Require a bound human confirmation receipt for actions marked `requires_confirmation`.
- Deny outbound communications, payments, deletions, configuration writes, physical/infrastructure actuation, and other irreversible actions unless separately permitted and confirmed.
- Emit an audit event before and after an accepted invocation.

### 6.4 `gaia.observe`

**Input:** admitted `intent_id` and an observation event.

**Behavior:** publishes status, provenance, structured errors, and results to authorized observers.

**MUST:** preserve intent correlation, redact unavailable content, distinguish observations from synthetic/model-generated outputs, and avoid leaking raw context to unauthorized observers.

### 6.5 `gaia.sign`

**Input:** canonical payload, key reference, and supported algorithm.

**Behavior:** produces a signature envelope.

**MUST NOT:** expose private keys or silently substitute an unrelated identity. Signing requires authority to use the referenced key.

### 6.6 `gaia.verify`

**Input:** an Intent record or signature envelope.

**Behavior:** returns a verification decision and reason codes without granting execution authority. Verification success alone is insufficient for admission; grant, consent, revocation, and policy checks remain required.

### 6.7 `gaia.declare`

**Input:** an agent, tool, sensor, knowledge source, or service declaration.

**Behavior:** records a descriptive capability surface, including supported inputs, outputs, required grants, resource bounds, data classes, network behavior, and AIP Manifest reference.

**MUST NOT:** grant a declared capability, create standing authority, or bypass AIP Manifest verification.

### 6.8 `gaia.learn.append`

**Input:** admitted `intent_id`, an experience payload, provenance, quality/uncertainty fields where applicable, and a signature.

**Behavior:** appends an immutable, signed experience/MemCube record that can be retrieved only under the applicable memory and consent controls.

**MUST:** include source/provenance, timestamp, author/subject binding, integrity hash, retention class, and correction/deletion linkage.

**MUST NOT:** retrain or alter weights; modify policy, grants, manifests, identity, permissions, or previously signed content; bypass deletion/correction; or write memory outside the admitted scope.

## 7. Least-privilege synthesis

The planner and coordinator MUST synthesize the narrowest executable plan. They MUST:

- Request only capabilities required for the stated purpose.
- Select the least-privileged target able to fulfill the request.
- Limit context to necessary MemCube references and redact where possible.
- Prefer local execution and deny network access unless an allowlisted requirement exists.
- Use the smallest practical runtime, resource, region, and data scope.
- Split a broader workflow into independently authorized steps.
- Treat ambiguity as a request for clarification or a denial, not an authorization to expand scope.
- Stop when a revocation, consent withdrawal, policy update, or confirmation failure occurs.

A capability requested by an intent but absent from any intersecting authority source MUST be denied before target selection or handler invocation.

## 8. Error taxonomy

| Code | Meaning | Handler invocations |
|---|---|---:|
| `GAIA_INTENT_SCHEMA_INVALID` | Intent fails structural/schema validation | 0 |
| `GAIA_INTENT_SIGNATURE_REQUIRED` | Signature envelope is missing | 0 |
| `GAIA_INTENT_SIGNATURE_INVALID` | Signature, hash, algorithm, or key binding fails | 0 |
| `GAIA_INTENT_EXPIRED` | Intent is outside its permitted time window | 0 |
| `GAIA_INTENT_REPLAY_DETECTED` | Nonce was previously accepted | 0 |
| `GAIA_GRANT_UNKNOWN` | Grant cannot be resolved | 0 |
| `GAIA_GRANT_REVOKED` | Grant, key, or consent is revoked | 0 |
| `GAIA_CAPABILITY_DENIED` | Requested scope exceeds effective authority | 0 |
| `GAIA_CONFIRMATION_REQUIRED` | A required human confirmation is absent | 0 |
| `GAIA_TARGET_NOT_DECLARED` | Target has no valid declaration/manifest | 0 |
| `GAIA_POLICY_DENIED` | Host, resource, jurisdiction, or data policy denies request | 0 |

Implementations MAY add codes but MUST preserve these meanings.

## 9. Required audit events

At minimum, the host MUST emit: `intent.received`, `intent.accepted`, `intent.denied.*`, `context.resolved`, `invoke.requested`, `invoke.started`, `invoke.completed`, `invoke.failed`, `learn.appended`, `grant.revoked`, and `consent.withdrawn`. Audit events MUST include a correlation identifier and MUST NOT contain unnecessary raw private data.

## 10. Non-goals

This draft does not:

- Define kernel-space syscalls or require kernel modifications.
- Define the final canonical JSON profile, key storage backend, DID method, or credential format.
- Replace the existing identity/capability contract in `identity-capabilities.md`.
- Define MCP wire transport, discovery, or network session behavior from Issue #23.
- Permit autonomous model self-rewrite, recursive self-improvement, or implicit privilege expansion.
- Claim or imply a hardware privilege boundary — see Section 0 (Path C declaration).

## 11. Compatibility

This ABI is frozen at v1.0. Future revisions MUST use a new `spec_version` or an explicitly backward-compatible extension path. Security-sensitive changes, including signature semantics, grant evaluation, or canonicalization, require an RFC before a stable release.

The executable Rust definition lives in `gaia-kernel/src/syscall.rs`. The `ABI_VERSION` constant in that crate is the machine-readable source of truth and MUST match the version declared in this document.

## 12. GAIA Runtime Call Table (Frozen v1.0)

> **Note (Path C):** These are **GAIA Runtime call numbers** dispatched entirely in userspace. They are not Linux syscall numbers and do not require kernel modifications. See Section 0.

The following call numbers are frozen. Renumbering any existing entry is a breaking change requiring a major version bump of `ABI_VERSION`.

| Number | Name | Description |
|--------|------|-------------|
| `0x01` | `IntentCreate` | Create and submit a new signed intent |
| `0x02` | `IntentQuery` | Query the status of an existing intent |
| `0x03` | `ContextRecall` | Recall MemCubes permitted by effective authority |
| `0x04` | `AgentInvoke` | Invoke a registered agent within intent bounds |
| `0x05` | `MemoryRead` | Read from the memory subsystem |
| `0x06` | `MemoryWrite` | Write to the memory subsystem |
| `0x07` | `ResourceDeclare` | Declare a resource for capability tracking |
| `0x08` | `Observe` | Publish an observation event |
| `0x09` | `CapabilityCheck` | Check whether a capability is currently granted |

New calls are assigned the next sequential number (`0x0A`, `0x0B`, …). Adding a new call is non-breaking. Unknown numbers return `NotImplemented` at runtime, allowing callers to probe availability gracefully.
