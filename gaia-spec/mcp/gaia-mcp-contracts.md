# GAIA–MCP Interface Contracts

Status: draft 0.1.0. Issue: #23.

## 1. Normative language

The terms MUST, MUST NOT, SHOULD, and MAY are normative. Payloads are JSON values encoded inside an MCP/JSON-RPC request or result. GAIA extensions are carried in the `gaia` object; unknown extension fields MUST be ignored unless marked required by a negotiated version.

## 2. Intent invocation

The orchestrator binds every tool call to one intent node. The request parameters MUST contain a `gaia` object with: `contract_version`, `intent_id`, `actor_id`, `resource_id`, `capabilities`, `jurisdiction`, `consent`, `context_refs`, `provenance`, and `policy_context`. `intent_id`, `actor_id`, and `resource_id` are non-empty stable identifiers. `capabilities` is the least-privilege capability set requested for this invocation. `context_refs` contains references, not secret material, unless a resource policy explicitly permits inline data.

Before dispatch, the client MUST resolve the registry entry, verify its declared capability, run client policy, construct the canonical signed message, and emit an audit event. A target not present in the registry or not authorized for the requested capability MUST fail before transport.

## 3. Tool and agent resources

A GAIA registry resource has one of `tool`, `agent`, `workflow`, or `data_service` types. Every local agent MUST have an AIP manifest reference. Resources MUST publish input and output schema identifiers, capability labels, trust requirements, supported contract versions, and an endpoint descriptor. A resource MAY expose MCP tools and MCP resources; a GAIA agent exposed as an MCP resource uses `gaia://agent/{resource_id}`.

## 4. Result contract

A successful result MUST include `gaia.result` with `intent_id`, `resource_id`, `status`, `provenance`, `quality`, `freshness`, and `audit_event_id`. `status` is `completed`, `partial`, or `streaming`. `provenance` identifies source system and time. `quality` is an implementation-defined assessment object. `freshness.observed_at` is RFC 3339 UTC time. The orchestrator MUST reject a result whose `intent_id` or `resource_id` does not match the admitted invocation.

## 5. Streams

A stream is bound to one admitted invocation. Open, update, and close events MUST contain the invocation correlation identifier and be admitted under the same identity and policy context. A receiver MAY request `pause`, `resume`, `throttle`, or `cancel`; cancellation MUST create an audit event. No stream update may increase granted capabilities, jurisdiction, or consent scope.

## 6. Error contract

GAIA errors are structured JSON-RPC errors with `data.gaia.code`. Required codes: `GAIA_UNAUTHENTICATED`, `GAIA_SIGNATURE_INVALID`, `GAIA_IDENTITY_UNTRUSTED`, `GAIA_POLICY_DENIED`, `GAIA_RESOURCE_NOT_FOUND`, `GAIA_CAPABILITY_DENIED`, `GAIA_CONSENT_REQUIRED`, `GAIA_SCHEMA_INCOMPATIBLE`, and `GAIA_REPLAY_REJECTED`. Denials MUST NOT execute the target handler and MUST be auditable.

## 7. Compatibility

`contract_version` uses semantic versioning. A server MUST reject an unsupported major version with `GAIA_SCHEMA_INCOMPATIBLE`; it MAY adapt a supported earlier minor version. Breaking changes require a migration note and a deprecation period consistent with GAIA governance.
