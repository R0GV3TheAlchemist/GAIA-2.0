# PROOF-GAIA20-AGENT-001 — CT-003 Safety Circuit Breaker and Failover

## Proof

**Type:** Empirical
**Status:** Proven
**Method:** The seeded `safety-primary` agent received three recorded failures, each 10 seconds apart, through `public.report_agent_failure`. This is within CT-003’s 30-second failure window. `public.failover_governance_agent('safety', 'safety-primary')` was then executed. Agent health, incident, consent, and execution-gate state were read from the live GAIA 2.0 Supabase project.

**Results:**

| Check | Expected | Observed |
|---|---|---|
| Circuit-breaker trip | OPEN at 3 failures in 30 seconds | `safety-primary` OPEN at failure count 3 |
| Execution safety response | Suspend execution | `consent_events`: `suspend`, scope `execution`, cause `governance circuit breaker opened` |
| Hot-standby promotion | Standby becomes primary | `safety-standby`: primary, CLOSED, failure count 0 |
| Failed primary | Isolated | `safety-primary`: non-primary, OPEN, failure count 3 |
| Governance ledger | Failover event recorded | `consent_events`: `failover`, scope `governance`, failed `safety-primary`, promoted `safety-standby` |
| Execution gate after failover | Allowed | `public.execution_permitted() = true` |

**Committed:** 2026-09-17, GAIA 2.0 Supabase project `yylqoiqobydrdsnnulip`

## Scope

This proof validates CT-003’s core resilience guarantees for the Safety governance agent:

- Three consecutive failures within 30 seconds open the circuit breaker.
- A tripped Safety circuit generates a consent-ledger execution suspension event.
- The hot-standby Safety instance can be promoted to primary.
- The incident and consent ledgers retain evidence of the event.
- Execution eligibility is restored only after a healthy closed-circuit Safety primary exists.