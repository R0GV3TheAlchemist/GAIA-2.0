# GAIA–MCP Specification

Status: draft. Issue: #23. Version: 0.1.0.

GAIA–MCP is the GAIA profile for invoking MCP tools and exposing GAIA tools and agents. MCP carries interoperable tool/resource messages; GAIA adds intent binding, identity, default-deny policy, auditability, sovereignty metadata, and AIP-manifest registration.

## Conformance

A conforming GAIA implementation MUST implement the contracts in `gaia-mcp-contracts.md` and `signing-and-policy.md`. It MUST pass the required cases in `wire-acceptance-tests.md`. Registry entries MUST validate against `registry.schema.json`.

## Boundaries

This draft specifies message and admission semantics, not a production network transport, mDNS/DHT implementation, marketplace, or a substitute for the upstream MCP specification. Current local/in-process and stdio adapters remain non-network transports. A conforming implementation MUST NOT describe itself as live network MCP until an authenticated external transport and the relevant acceptance tests are implemented.

## Documents

- `gaia-mcp-contracts.md` — intent, request, result, resource, and error contracts
- `signing-and-policy.md` — Ed25519 admission, policy, and audit contract
- `registry.schema.json` — local registry entry schema
- `wire-acceptance-tests.md` — executable acceptance requirements
