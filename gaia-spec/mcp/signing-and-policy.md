# GAIA–MCP Signing and Policy

Status: draft 0.1.0. Issue: #23.

## 1. Default-deny admission

Every request, response, and stream-control message MUST be authenticated before it is processed. Unsigned, malformed, expired, replayed, revoked, or untrusted messages MUST be rejected. A transport-level authentication mechanism does not replace application-message signing.

## 2. Identity and signature

The baseline algorithm is Ed25519. The sender provides a `gaia_signature` object containing `version`, `algorithm` (`Ed25519`), `key_id`, `created_at`, `expires_at`, `nonce`, and base64url `signature`. `key_id` resolves through GAIA identity/trust infrastructure to a non-revoked public key and principal.

The signed bytes are UTF-8 canonical JSON of an object containing exactly: `jsonrpc`, `id`, `method`, `params`, `sender_key_id`, `created_at`, `expires_at`, and `nonce`. Implementations MUST define one deterministic canonical JSON encoder and MUST test byte-for-byte interoperability. Signatures MUST cover the request body and MUST NOT be verified over an arbitrary string representation.

## 3. Verification order

The server MUST: (1) parse with bounded size/depth; (2) require the signature envelope; (3) validate timestamp skew and expiry; (4) reject a previously seen nonce for the key within its replay window; (5) resolve identity and check revocation; (6) reconstruct canonical bytes; (7) verify Ed25519; (8) resolve the resource; (9) evaluate policy; (10) emit an admission audit event; and only then invoke a handler. Failure stops the sequence.

## 4. Policy decision

Policy input MUST include caller principal, resource id/type, requested capabilities, intent id, jurisdiction, consent, data classifications, trust requirements, and operation. The policy decision is `allow`, `deny`, or `allow_with_obligations`. Obligations may require redaction, human approval, rate limits, retention limits, or additional audit fields. A policy decision MUST be included in the audit record.

## 5. Audit minimum

For every admitted, denied, cancelled, or completed invocation, record: event id, timestamp, correlation/idempotency id, intent id, caller principal/key id, resource id, canonical-message digest, signature verification outcome, policy id/version and decision, jurisdiction, consent reference, outcome, and result digest where available. Audit records MUST NOT store sensitive payloads by default.

## 6. Key lifecycle

Keys MUST support expiry and revocation. Rotation MUST preserve a verifiable identity binding and audit continuity. Private keys MUST NOT be placed in manifests, registry records, logs, source control, or MCP messages.
