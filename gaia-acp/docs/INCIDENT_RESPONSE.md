# Local incident response (#347)

No autonomous public notification. No remote pager.

## Valid / invalid transitions

See `PlaneState::can_transition`. Calls are allowed only in `ManifestIssued`.

## Workflow

1. Stop / `kill` the plane (emergency stop + `Killed`).
2. Revoke agent, server, and approval IDs.
3. Preserve the local hash-chained `ActionReceipt` list (no raw prompts or tokens).
4. Classify: injection, scope, egress, replay, supply-chain, cross-agent.
5. Remediate policy/manifest/config.
6. Retest the matching fixture.

## Anomaly signals already emitted

Repeated denials (`deny_streak >= 5`), scope-expansion attempts, identity creation, private-range egress, approval replay, policy-tamper text, budget exhaustion, emergency stop.
