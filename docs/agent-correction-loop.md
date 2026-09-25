# Agent Correction Loop — Playbook

> **Scope**: This document is the authoritative specification for AI
> contributors executing a human-gated correction loop inside GAIA 2.0.
> Every loop turn must comply with this playbook, GAIA's approval
> contract (`gaia-acp::HumanApprovalReceipt`), and the GAIA Canon.

---

## 1. Purpose

The correction loop lets an AI assistant inspect CI failures, diagnose
the root cause, propose a minimal repair, request explicit human
approval for each repository mutation, apply the approved change, and
re-validate the resulting commit — until the branch is green or a
bounded stop condition is reached.

This is **execution-feedback-driven repair**, not autonomous
recursive self-improvement. Tests and CI checks are the external
evaluator. The assistant's own confidence is never treated as proof
of correctness.

---

## 2. State Machine

```
OBSERVE
  → DIAGNOSE
  → PROPOSE_PATCH
  → AWAIT_APPROVAL
  → APPLY_APPROVED_PATCH
  → VALIDATE_TARGETED
  → VALIDATE_FULL
  → COMPLETE
```

### States

| State | Description |
|---|---|
| `OBSERVE` | Read PR head SHA, changed files, check runs, sticky diagnostics comment, and `agent-validation.json` artifact. No writes. |
| `DIAGNOSE` | Classify the failure, link it to specific diagnostics and files, assess risk. No writes. |
| `PROPOSE_PATCH` | Prepare the minimal repair: exact files, expected diff scope, risk class, targeted validation command, rollback plan, confidence. No writes. |
| `AWAIT_APPROVAL` | Present the approval request (see Section 5) and wait for explicit human approval. No writes until approved. |
| `APPLY_APPROVED_PATCH` | Push exactly the approved patch to exactly the approved branch and SHA. No scope expansion. |
| `VALIDATE_TARGETED` | Read targeted check runs for the new commit. No writes. |
| `VALIDATE_FULL` | Read full required CI checks for the new commit. Do not report completion until full CI passes. No writes. |
| `COMPLETE` | Full required CI is green. Report the resulting commit SHA and check run links. |

### Stop States

| Stop State | Trigger |
|---|---|
| `DENIED` | Human explicitly rejects the approval request. |
| `STALE_HEAD` | Branch HEAD changed between OBSERVE and APPLY; the approval is now invalid. Request fresh approval or abort. |
| `NO_PROGRESS` | Same failure fingerprint appears in two consecutive attempts without a new commit that changes the failing stage. |
| `MAX_ATTEMPTS` | Approved repair attempts reach the configured maximum (default: **3**). |
| `POLICY_BLOCKED` | Proposed repair touches an escalation-required category (see Section 6). Loop halts; human must resolve. |
| `INFRASTRUCTURE_FAILURE` | CI is unavailable, runner is flaky, or validation cannot produce a reliable result. Do not rewrite code speculatively. |
| `HUMAN_ESCALATION_REQUIRED` | Failure cannot be safely classified or repaired within the allowed scope. Loop halts; human must investigate. |

---

## 3. Read-Only vs. Write Operations

### Read-only (no approval required)

- Read repository file contents, diffs, and commit history.
- Read PR metadata, head SHA, changed files, and base SHA.
- Read check runs and their conclusions.
- Read the sticky diagnostics PR comment.
- Download and parse the `agent-validation.json` CI artifact.
- Read attempt history and failure fingerprints.
- Diagnose a failure and prepare a repair proposal.

### Write operations (explicit approval required for each)

| Operation | Approval class |
|---|---|
| Create a branch | `CREATE_BRANCH` |
| Push an initial patch | `PUSH_PATCH` |
| Push a corrective patch (retry) | `PUSH_CORRECTIVE_PATCH` |
| Open or materially update a PR | `UPDATE_PR` |
| Merge a PR | `MERGE` (separate, never implied by patch approval) |
| Deploy, publish, or release | Separate explicit approval; out of scope for this loop |

Each write operation requires a **fresh approval** bound to the
exact operation, target files, branch, and expected head SHA at the
time of the request.

---

## 4. SHA Binding Rules

1. **Always read the current head SHA before proposing a patch.**
   Use the SHA returned by `get_file_contents` or `pull_request_read`
   at the moment of proposal — not a cached value.

2. **Bind the approval request to the exact head SHA.**
   If the branch advances between the approval request and the push
   (another commit lands), the approval is invalid. Enter `STALE_HEAD`,
   discard the approval, and start from `OBSERVE`.

3. **Bind the diagnostic result to the exact head SHA.**
   If `agent-validation.json` was produced for a different SHA than
   the current branch head, reject it as stale and re-read checks.

4. **Never repair against a stale diagnostic.**
   A repair proposed for SHA `A` must not be applied to SHA `B`.

---

## 5. Repair Proposal Requirements

Before entering `AWAIT_APPROVAL`, the assistant must produce a complete
repair proposal containing all of the following. Incomplete proposals
must not be submitted for approval.

```
Failure classification:  product-code | test | workflow/infra |
                         dependency | flaky | policy-violation
Failed stage:            <stage name from agent-validation.json>
Failure fingerprint:     <fingerprint from agent-validation.json>
Diagnostic evidence:     <specific error code, file, and line>
Target files:            <explicit list — no wildcards>
Diff scope:              <concise description of what changes and why>
Risk class:              low | medium | high
Targeted validation:     <exact command to run after patch>
Rollback plan:           <how to undo if targeted validation fails>
Attempt:                 <n of max (default max: 3)>
Confidence:              <percentage or qualitative>
Uncertainty:             <explicit statement of what is unknown>
```

### Forbidden repair actions

The following are **never** acceptable as repairs, regardless of
whether they make CI green:

- Deleting tests.
- Weakening assertions or reducing test coverage.
- Suppressing compiler warnings or lints.
- Broadening permissions or access scopes.
- Disabling security, canon, or governance checks.
- Changing workflow permissions without separate explicit review.
- Editing dependencies without separate explicit review.

---

## 6. Escalation Triggers

If the proposed repair touches any of the following categories, the
loop must immediately enter `POLICY_BLOCKED` and request dedicated
human review. These changes cannot be introduced silently as a "repair":

- Secrets, credentials, or environment variables.
- Workflow permissions (`permissions:` blocks in `.github/workflows/`).
- Dependency versions (`Cargo.toml`, `Cargo.lock`, `package.json`, etc.).
- Governance or canon files.
- Deployment, release, or publication configuration.
- Branch protection rules or CODEOWNERS.
- Security boundary changes of any kind.

---

## 7. Approval Validity

An approval is **valid** only when all of the following are true:

- The human has explicitly approved (conversational text alone is not
  approval; see `docs/agent-approval-templates.md`).
- The approval names the exact operation, branch, target files, and
  expected head SHA.
- The branch head SHA has not changed since the approval was given.
- The proposed patch has not been modified after approval was given.
- The file set has not expanded beyond what was approved.
- The approval has not been used for a previous push (single-use).

Any change to the proposal, a stale head, or an expanded file set
**invalidates** the approval. A new approval request must be made.

---

## 8. Safety Limits (Defaults)

| Limit | Default |
|---|---|
| Maximum approved repair attempts per task | 3 |
| Stop on repeated failure fingerprint | After 2 consecutive identical fingerprints |
| Scope expansion | Stop immediately; request new approval |
| Flaky/unavailable infrastructure | Stop; do not rewrite code speculatively |
| Emergency stop | Human may revoke at any point; loop must halt on revocation |

---

## 9. Reading `agent-validation.json`

After CI completes, download the `agent-validation` artifact from the
workflow run. Validate before using:

1. Confirm `head_sha` matches the current branch head. Reject if stale.
2. Confirm `schema_version` is `"1.0"`.
3. Read `status` (`"passed"` or `"failed"`).
4. If `"failed"`, read `failed_stage` and `diagnostics[]`.
5. Extract `diagnostics[*].fingerprint` for no-progress detection.
6. Pass `fingerprint` to `scripts/agent-validate.sh --fingerprint <value>
   --attempt <n>` on subsequent attempts.

**Never use a cached or previously downloaded artifact** for a
different commit than the current head.

---

## 10. No-Progress Detection

Before entering `PROPOSE_PATCH` on attempt 2 or later:

1. Compare the current `diagnostics[0].fingerprint` against the
   fingerprint from the previous attempt.
2. If the fingerprints are identical and the failing stage is unchanged,
   enter `NO_PROGRESS` and stop the loop.
3. If the fingerprints differ or the failing stage changes, the repair
   has made measurable progress and the loop may continue.

Run `scripts/agent-validate.sh --fingerprint <prev> --attempt <n>` to
have the validator enforce this check automatically and emit
`"no_progress": true` in the JSON output.

---

## 11. Audit Trail

For each loop task, record:

- Task/issue/PR identifier.
- Base SHA and initial head SHA.
- Each attempt: attempt number, diagnostic fingerprint, repair proposal
  summary, approval receipt ID or denial, resulting commit SHA,
  targeted and full validation results.
- Final status and stop reason.

Do not record secrets, access tokens, raw model reasoning, or hidden
prompts in the audit trail.

---

## 12. Example Walkthrough

This example walks through a complete loop turn against a synthetic
`cargo fmt` failure.

### Setup

- PR: `feat/example-fix` targeting `main`
- Head SHA: `abc1234`
- CI failure: `rust-ai-diagnostics` reports `status: failed`,
  `failed_stage: cargo-fmt`, fingerprint: `fmt:unformatted`

### Step-by-step

**OBSERVE**
```
Read PR head SHA: abc1234
Read agent-validation.json artifact (sha matches: abc1234 ✓)
status: failed | failed_stage: cargo-fmt | fingerprint: fmt:unformatted
```

**DIAGNOSE**
```
Classification: workflow/infra (formatting)
Evidence: cargo fmt --all --check produced diff output
Affected files: src/lib.rs (whitespace/indent)
Risk: low (pure formatting, no logic change)
```

**PROPOSE_PATCH**
```
Failure classification:  workflow/infra
Failed stage:            cargo-fmt
Failure fingerprint:     fmt:unformatted
Diagnostic evidence:     cargo fmt --all --check found unformatted files
Target files:            src/lib.rs
Diff scope:              Apply cargo fmt formatting only; no logic changes
Risk class:              low
Targeted validation:     cargo fmt --all --check
Rollback plan:           Revert commit abc5678 if targeted validation fails
Attempt:                 1 of 3
Confidence:              95%
Uncertainty:             None identified
```

**AWAIT_APPROVAL** → Human approves (see template in
`docs/agent-approval-templates.md`)

**APPLY_APPROVED_PATCH**
```
Push formatted src/lib.rs to feat/example-fix
New commit SHA: def5678
```

**VALIDATE_TARGETED**
```
cargo fmt --all --check: passed ✓
```

**VALIDATE_FULL**
```
All required CI checks on def5678: passed ✓
```

**COMPLETE**
```
Branch feat/example-fix is green at def5678.
Loop terminated: COMPLETE after 1 approved attempt.
```

---

## 13. Emergency Stop

At any point, you may revoke an in-progress loop by:

1. Declining the next approval request, or
2. Explicitly stating "stop the correction loop" or "abort".

The assistant must halt all further writes immediately upon revocation.
Partial patches that have already been pushed are not automatically
reverted; instruct the assistant explicitly if a rollback is needed.

---

*This document is part of Phase 3 of the human-gated correction loop.
See issue [#986](https://github.com/R0GV3TheAvatar/GAIA-2.0/issues/986)
and the parent issue [#983](https://github.com/R0GV3TheAvatar/GAIA-2.0/issues/983).*
