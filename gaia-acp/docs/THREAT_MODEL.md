# Agent control plane — threat model (#342)

Local-first. No real MCP server, credential, person, or public-network target is in scope.

## Invariant

An agent may propose an action. An independent deterministic enforcement layer decides whether that exact action may execute. Model text is guidance, not authority.

## Trust types

| Type | Authority |
| --- | --- |
| `SignedIntent` | Trusted when constructed by control-plane code |
| `TrustedPolicy` | Trusted; versioned; not writable by agents |
| `CapabilityManifest` | Trusted; issued, expired, revoked independently |
| `HumanApprovalReceipt` | Trusted; exact request-hash binding; single use |
| `UntrustedContent` | Never authority |
| `UntrustedToolOutput` | Never authority |
| `ActionReceipt` | Evidence, not a grant |

## Trust-boundary diagram

```
untrusted files / issues / tool text / MCP metadata
        |  (labeled UntrustedContent)
        v
   model / planner  ---- proposes ProposedAction ----+
                                                     |
        SignedIntent + CapabilityManifest + clock     v
                              -> PolicyEngine -> Gateway
                                                     |
                         HumanApprovalReceipt -------+
                                                     v
                              Fake local MCP adapter
                              (no network, no host secrets)
                                                     v
                              hash-chained ActionReceipt
```

## Threat matrix

| Asset | Attacker | Entry | Prohibited outcome | Prevent | Detect | Recover |
| --- | --- | --- | --- | --- | --- | --- |
| Tool allowlist | Injected README/issue | UntrustedContent | New tool granted | Authority-claim deny | Receipt + deny streak | Revoke/kill |
| Path scope | Path `..` / `.env` | target field | Protected-path write | Traversal + protected rules | Reason codes | Stop |
| Network | SSRF / RFC1918 / metadata | egress target | Private-range call | Default-deny egress | EgressDenied receipts | Kill |
| Approval | Replay / mismatch | stale receipt | High-risk execute | Hash + single-use + TTL | ApprovalReplay | Revoke receipt |
| Identity | Cross-agent use | foreign agent_id | Privilege laundering | Manifest bind | CrossAgent | Revoke both |
| Secrets | Prompt / logs | payload | Token in model/logs | Receipt omits payload | Leak test | Rotate (ops) |
| Policy | “ignore previous” | untrusted text | Policy rewrite | Typed facts only | UntrustedAuthority | Kill |
| Supply chain | `latest` / `sh -c` / env secrets | MCP config | Unreviewed server | lint_mcp_config | ConfigRejected | Do not register |
| Persistence | Repeated denials | retry loop | Budget/fatigue | deny streak anomaly | AnomalyDetected | Emergency stop |

## Fixture categories (10+)

1. Direct prompt injection
2. Indirect injection in README/issues/PRs
3. Tool-output injection
4. Malicious MCP metadata
5. Path traversal
6. SSRF / private egress
7. Approval replay / mismatch
8. Cross-agent laundering
9. Secret exposure
10. Supply-chain / mutable config
11. Policy tampering
12. Identity creation
13. Repeated deny retries

## Not verified

Remote enforcement, live OPA/Cedar, container runtime isolation, real MCP servers.
