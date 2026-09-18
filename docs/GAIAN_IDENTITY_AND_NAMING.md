# GAIAN Identity and Naming

**Status:** Normative naming reference

**Version:** 0.1.0

**Date:** 2026-09-18

## Purpose

This document prevents ambiguity between the GAIA 2.0 system, an individual GAIAN principal, and a GAIAN's human-facing name. It establishes the rule that names support recognition and relationship; cryptographic identity, consent, and authorization remain attached to stable principal identifiers.

## Core terms

| Term | Meaning | Authority role |
|---|---|---|
| **GAIA 2.0** | The open-source system architecture, repository, protocols, safety model, and local-first runtime | System identifier; never a substitute for a principal ID |
| **GAIAN** | An individual assistant/principal operating within GAIA 2.0 | A principal with a stable cryptographic identity and owner/consent boundary |
| **GAIA** | The chosen display name of Kyle Steen's personal GAIAN | Human-facing name only; not an authorization credential |
| **Saela** | Conceptual and interface lineage for the assistant experience | A legacy/conceptual name; not a second principal or security identity |
| **DID / principal ID** | A stable, cryptographically verifiable identifier | The authoritative identity for signatures, audit, capability, consent, and authorization |

## Canonical statement

> **GAIA 2.0 is the system. A GAIAN is an individual principal. GAIA is Kyle Steen's personal GAIAN.**

Saela is part of the conceptual and assistant-interface lineage that maps into GAIA 2.0. It does not create a competing identity, duplicate principal, or separate authorization domain.

## Identity model

Every GAIAN MUST have one stable cryptographic principal identity independent of its display name. A GAIAN MAY have a display name and one or more aliases, but those labels MUST NOT be used as authentication, authorization, audit, consent, provenance, capability, or ownership keys.

A GAIAN identity record SHOULD distinguish the following fields:

```text
system_id:        gaia-2.0
principal_kind:   gaian
principal_did:    did:key:...
display_name:     GAIA
profile_slug:      kyle-gaia
aliases:           []
owner_boundary:    explicit consent and ownership record
lifecycle_state:   active | suspended | revoked | archived
```

The example values above describe GAIA as Kyle Steen's personal GAIAN. Other GAIANs MUST use their own principal IDs, owner/consent boundaries, and chosen names.

## Naming rules

1. **Display names are not security identities.** A display name such as `GAIA` MUST NOT be accepted as proof of identity, authority, ownership, consent, or capability.
2. **DIDs are authoritative.** Signatures, audit records, capability tokens, revocation records, provenance tags, and access decisions MUST bind to `principal_did` or an equivalent stable cryptographic principal ID.
3. **Names may change.** A GAIAN's display name and aliases MAY change without changing its principal identity. Name changes MUST NOT silently transfer authorization or ownership.
4. **Names are not globally unique.** Multiple GAIANs may use similar or identical display names. Systems MUST disambiguate them using the stable principal ID and profile/owner context.
5. **Names must not imply trust level.** A display name, alias, title, or user-facing description MUST NOT silently upgrade a GAIAN's trust level or capabilities.
6. **Audit uses stable identifiers.** User interfaces MAY display a name beside an audit entry, but the canonical stored and signed reference MUST be the stable principal ID.
7. **Telemetry minimizes identity data.** Operational telemetry SHOULD use a pseudonymous or keyed principal reference when a full DID is unnecessary.

## Saela relationship

Saela describes the assistant-facing conceptual lifecycle:

```text
ask -> plan -> execute -> evaluate
```

GAIA 2.0 provides the accountable architecture beneath that lifecycle:

```text
intent admission
  -> signed intent graph
  -> human-accepted plan
  -> capability-constrained execution
  -> audit, evaluation, and provenance
```

Documentation SHOULD use **Saela-to-GAIAN mapping** when discussing the conceptual assistant interface. Documentation SHOULD use **GAIA 2.0** when discussing the system, repository, runtime, protocols, or control plane.

## Security and privacy implications

The following values MUST NOT be conflated:

```text
display_name       != principal_did
principal_did      != owner identity
owner identity     != consent grant
consent grant      != capability token
capability token   != trust level
trust level        != authorization decision
```

For example, a record may present `GAIA` to a human user while all authority decisions use the principal DID and an explicit, current capability/consent context. A copied, renamed, or similarly named profile MUST gain no authority from name similarity.

## Implementation guidance

- Use `principal_did` or an equivalent stable principal key in Ed25519 signature verification, audit chaining, capability delegation, revocation, memory provenance, and policy decisions.
- Use `display_name` only in human-facing interfaces and carefully scoped documentation.
- Store mutable names separately from security-critical identifiers.
- Record name/alias changes as auditable metadata without changing the cryptographic principal.
- Do not include full DIDs in operational telemetry unless necessary; prefer a stable keyed fingerprint or pseudonymous reference.
- Do not infer a child's guardian, a user's identity, or a GAIAN's owner from a display name. Use explicit verified records and consent-aware authorization.

## Scope and honesty

This is a naming and identity-boundary reference. It does not claim that every GAIA 2.0 crate currently enforces every rule above. Implemented enforcement, proposed integration, and future work MUST be documented separately and verified in code and tests before being claimed as runtime behavior.
