# Agent tool control plane

**Issue:** #342  
**Parent:** #341 — Zero-trust MCP tool governance and evaluation  
**Status:** local-first architecture contract; no live external integration  
**Date:** 2026-09-18

## Objective

Define the smallest trusted control plane that allows GAIA to evaluate an agent-proposed tool request without letting the agent, a prompt, retrieved text, or tool output authorize itself.

## Authority order

1. Human stop/revocation and protected system policy.
2. Verified signed intent, capability manifest, and exact approval receipt where required.
3. Deterministic gateway policy decision.
4. Sandboxed adapter execution.
5. Model proposal, untrusted content, and untrusted tool output.

Lower layers cannot override higher layers.

## Local-first request flow

```text
ModelProposal
  -> normalize into ToolRequest
  -> validate request schema
  -> load trusted context (identity, manifest, revocation, environment, budget)
  -> evaluate deterministic policy
  -> require/verify HumanApprovalReceipt if risk requires it
  -> call fake/local adapter only if allowed
  -> emit privacy-minimized ActionReceipt
```

## Required facts

The gateway, not the model, constructs these trusted facts:

| Fact | Source of authority |
| --- | --- |
| Agent identity | Authenticated runtime identity |
| Tool/server ID and version | Verified local registry |
| Action class and risk tier | Gateway-owned mapping |
| Resource/path/destination | Canonicalized request fields |
| Capability manifest | Verified issuer, task binding, expiry, revocation |
| Approval receipt | Verified approver, exact request hash, target, expiry, single use |
| Environment mode | Runtime deployment configuration |
| Control-plane state | Trusted local state / explicit gate |
| Budgets and rate limits | Gateway-maintained counters |
| Policy version and decision | Versioned deterministic policy code |

## Risk tiers

| Tier | Action examples | Default policy |
| --- | --- | --- |
| 0 | Local parsing, formatting a scratch artifact | Allow only within manifest |
| 1 | Approved local read, sandboxed unit test | Allow only within manifest |
| 2 | Scratch patch/draft | Policy-dependent; never direct protected write |
| 3 | External write/public comment/non-sensitive database write | Exact human approval required |
| 4 | Merge, deploy, CI/auth/RLS change, publish, secret access | Exact approval; later two-person requirement assessment |
| 5 | Financial, identity, high-stakes, destructive production action | Prohibited in initial GAIA agent control plane |

## Deterministic decision states

```text
Allow
Deny(reason_code)
RequireApproval(reason_code)
```

Default is `Deny(GAIA_POLICY_DEFAULT_DENY)`.

An allow applies only to one normalized request within its manifest/approval expiry and budget. It is not a session-wide privilege grant.

## Trust-boundary rules

- A model proposal cannot state or create `intent_valid`, `approval_valid`, `manifest_valid`, role, environment, budget, or policy result.
- Tool/server descriptions, annotations, names, and returned text are untrusted unless independently verified by registry and policy.
- All request parameters must be typed, normalized, size bounded, and schema validated before policy evaluation.
- No unrestricted `shell(command: String)` interface exists in the initial design.
- No external tool call occurs directly from model output.
- No secrets, raw prompts, or unrestricted output are included in action receipts.
- Approval text in conversation is not a system approval receipt; only the governed approval mechanism can mint one.

## Initial local interfaces

```rust
pub enum TrustClass {
    SignedIntent,
    TrustedPolicy,
    CapabilityManifest,
    HumanApprovalReceipt,
    PolicyDecision,
    UntrustedContent,
    UntrustedToolOutput,
    ModelProposal,
    ActionReceipt,
}

pub enum AuthorityUse {
    GrantCapability,
    AlterPolicy,
    ApproveAction,
    SelectTool,
    ExpandScope,
    SelectEgressDestination,
    CreateIdentity,
    InvokeExternalEffect,
}

pub enum TrustBoundaryDecision {
    Allow,
    Deny { code: TrustBoundaryDenyCode },
}
```

The initial predicates are deliberately conservative:

- Untrusted content/tool output/model proposal: always deny every `AuthorityUse`.
- Action receipt: never grants new authority.
- Signed intent/manifest/approval/policy decision: are not sufficient alone; later tasks compose them in gateway policy.

## Sandbox and egress boundary

Before real MCP servers are considered, #346 defines the operational boundary:

- non-root disposable workload;
- read-only source and separate scratch write area;
- no host mounts, Docker socket, SSH agent, home directory, or inherited secrets;
- resource/time/action budgets;
- default-deny egress and explicit destination allowlists;
- block loopback, private, link-local, metadata, proxy-bypass, and unapproved redirects.

## Evidence and incident rule

Every consequential attempted action eventually produces a privacy-minimized receipt with correlation ID, stable reason code, policy/manifest/approval references, and outcome. Receipts do not become authorization. #347 defines the full state machine and incident response.

## Current limits

- This is not a deployment claim.
- There is no real MCP gateway, OPA/Cedar deployment, container runtime, egress proxy, or remote audit sink in this issue.
- Existing GAIA `CapabilityEnforcer`, `NonceStore`, `TrustAudit`, and local runner gate are foundations to integrate in later tasks.
