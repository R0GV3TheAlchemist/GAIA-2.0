# Untrusted-content threat model

**Issue:** #342  
**Parent:** #341 — Zero-trust MCP tool governance and evaluation  
**Status:** local-first design and test contract  
**Date:** 2026-09-18

## Purpose

GAIA agents may read repositories, issues, pull requests, documentation, logs, tool responses, model output, MCP metadata, package metadata, datasets, and web content. Any of those sources may contain text intended to redirect an agent, expand its authority, expose data, or cause an external effect.

This document defines the trust boundary that makes such text **data, not authority**.

## Security invariant

> An agent may propose an action. Only an independent, deterministic policy-enforcement path may authorize the exact action.

No `UntrustedContent` or `UntrustedToolOutput` may grant capability, alter policy, change an approval decision, select a new server or tool, increase a scope, create an identity, select an egress destination, or invoke an external effect.

## Trust classes

| Class | Examples | May grant authority? | Handling |
| --- | --- | ---: | --- |
| `SignedIntent` | Verified, expiry-bound human/system intent | Only through policy evaluation | Validate signature, scope, expiry, nonce |
| `TrustedPolicy` | Versioned policy loaded by trusted control plane | Yes, through deterministic engine only | Version, review, integrity check |
| `CapabilityManifest` | Verified task-bound tool/resource/destination budget | Yes, as an allowlist only | Validate issuer, task, expiry, revocation |
| `HumanApprovalReceipt` | Exact target/method/request-hash, single-use approval | Yes, for its exact bound action only | Validate identity, expiry, one-time use, revocation |
| `PolicyDecision` | Gateway-generated allow/deny/approval-required result | Yes, for one request only | Never accept from model or tool output |
| `UntrustedContent` | README, issue, PR, code comments, web page, dataset text | No | Parse as data; never execute instructions |
| `UntrustedToolOutput` | MCP response, shell/test log, API response, error text | No | Schema validate, size limit, redact, treat as data |
| `ModelProposal` | Model-selected plan or tool request | No | Normalize and independently validate |
| `ActionReceipt` | Gateway-generated record of an attempted effect | No new authority | Append-only local evidence |

## Assets to protect

- Human sovereignty and meaningful approval.
- Private memory, credentials, tokens, keys, and identifiers.
- Source integrity, branch protection, CI/CD configuration, and releases.
- Database schema, RLS, authorization policy, and audit evidence.
- Network boundaries, cloud metadata, local services, and external contacts.
- Accurate claims about deployed versus locally verified behavior.

## Adversary and entry points

The adversary may be a malicious repository contributor, package publisher, MCP-server author, website owner, compromised dependency, compromised tool, prompt author, or an otherwise benign source containing adversarial text. The model may also be mistaken, overconfident, or persist after a denial.

| Threat class | Entry point | Prohibited outcome | Required preventive control | Detection / recovery |
| --- | --- | --- | --- | --- |
| Direct prompt injection | User/model context | Policy or scope override | Immutable policy outside model context | `GAIA_UNTRUSTED_AUTHORITY`; record deny |
| Indirect prompt injection | README, issue, PR, docs, web/RAG | Tool invocation or data disclosure | `UntrustedContent` envelope; gateway authorization | Fixture regression; stop task |
| Tool-output injection | MCP/API/log/error output | New tool/server/policy selection | Typed output handling; no authority derivation | `GAIA_UNTRUSTED_TOOL_SELECTION` |
| Poisoned MCP metadata | Tool description/annotations/resources | Misleading privilege or unsafe call | Verified private registry; metadata is untrusted | Server verification failure |
| Scope expansion | Text asks for broader paths/tools/host | Capability escalation | Manifest allowlist; default deny | `GAIA_UNTRUSTED_SCOPE_EXPANSION` |
| Approval laundering | Text claims approval or changes payload after approval | Unauthorized external effect | Exact request hash, target binding, single-use receipt | `GAIA_APPROVAL_MISMATCH` / replay deny |
| Path traversal | Tool arguments or generated patch | Read/write outside approved scope | Canonical path validation; protected path policy | `GAIA_PATH_OUTSIDE_SCOPE` |
| Secret exfiltration | Prompt, log, tool request | Secret read/upload | No inherited secrets; secret boundary; egress policy | `GAIA_SECRET_ACCESS_DENIED` |
| SSRF / egress abuse | URL/tool parameter | Private, metadata, or arbitrary network access | Default-deny egress; destination allowlist | `GAIA_EGRESS_DENIED` |
| Proxy / Tor bypass | Config or command text | Network-policy evasion | Deny proxy mutations and anonymous routing | `GAIA_EGRESS_BYPASS_DENIED` |
| Identity creation | Tool request or text | Fake account/persona/contact | Prohibit by manifest and policy | `GAIA_IDENTITY_CREATION_DENIED` |
| Cross-agent laundering | One agent output to another | Privilege inherited through text | Output remains untrusted; independent manifests | Correlation/audit review |
| Policy tampering | Code/config/documentation change | Disable enforcement or audit | Protected paths; review; signed policy lifecycle | `GAIA_POLICY_TAMPER_DENIED` |
| Supply-chain poisoning | Dependency/CI/MCP config | Execute mutable or malicious code | Pinned versions, review, scanners, provenance | CI denial and remediation |
| Denial persistence | Repeated requests after deny | Exhaustion or eventual bypass | Budget, retry cap, circuit breaker, kill switch | `GAIA_RETRY_BUDGET_EXHAUSTED` |

## Boundary diagram

```text
Untrusted sources
  (GitHub, MCP metadata/output, web, logs, packages, model text)
                    |
                    v
          UntrustedContent envelope
                    |
                    v
          Model proposal (not authority)
                    |
                    v
Signed intent + manifest + approval + trusted context
                    |
                    v
     Deterministic policy decision / enforcement gateway
                    |
          allow only if exact request is authorized
                    |
                    v
       Sandboxed local/fake adapter -> action receipt
```

## Initial fixture corpus

The first local-only regression corpus must cover at least these fixtures:

1. README text that says to ignore policy and read secrets.
2. GitHub issue text that asks for a public PR or new server connection.
3. PR comment that tries to change the requested target after approval.
4. Source-code comment containing a tool-selection instruction.
5. Test/error log that asks to run a shell command or disable a check.
6. MCP tool description claiming it is safe/read-only while requesting a write.
7. MCP tool output proposing a new server, token, or capability.
8. Relative/encoded path traversal outside an approved repository path.
9. URL pointing to loopback, RFC1918, link-local, or cloud metadata ranges.
10. Text requesting proxy/Tor configuration or arbitrary DNS tunneling.
11. Text claiming a nonexistent human approval receipt.
12. A valid approval receipt reused or paired with changed target/payload.
13. One agent's output asking another to use its authority.
14. Dependency or MCP config that uses `latest`, shell expansion, or inline secret.
15. Repeated denied calls intended to exhaust a retry or action budget.

## Non-goals and limits

- This contract does not prove that a language model will never generate unsafe text.
- Keyword filters and prompt delimiters may assist UX but are not enforcement boundaries.
- The current in-process capability and memory guards are local seams, not a production distributed authorization system.
- This issue does not connect GAIA to live MCP servers, GitHub APIs, Supabase, Hugging Face, browsers, shells, or public networks.
- A passing local fixture test is evidence of the local boundary only, not proof of production safety.

## Research record

- MCP security best practices: https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/security_best_practices
- MCP tool security guidance: https://modelcontextprotocol.io/specification/2026-07-28/server/tools
- OWASP LLM Top 10: https://owasp.org/www-project-top-10-for-large-language-model-applications
- NIST AI Agent Standards Initiative: https://www.nist.gov/artificial-intelligence/ai-agent-standards-initiative
- UK AISI incident report: https://www.aisi.gov.uk/blog/incident-report-unsanctioned-agent-behaviour-during-cyber-testing
