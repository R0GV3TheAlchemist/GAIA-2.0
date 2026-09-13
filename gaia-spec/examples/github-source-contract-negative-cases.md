# GitHub Source Contract Negative Cases

These examples are intentionally invalid and MUST be exercised by the local schema/policy test harness. They are documented rather than stored as JSON fixtures so generic repository JSON-schema validation does not treat them as valid artifacts.

## Result cases

| ID | Invalid condition | Required rejection |
|---|---|---|
| `result-missing-resolved-commit` | A result omits `resolved_commit_sha` | Reject: symbolic ref alone is not immutable provenance |
| `result-complete-without-content` | `state` is `complete` but `content` is absent | Reject: complete results must carry bounded content |
| `result-truncated-without-metadata` | `state` is `truncated` but `truncation` is absent | Reject: truncation must be explicit |
| `result-denied-without-decision` | `state` is `denied` but `policy_decision` is absent | Reject: denial must be explainable and auditable |
| `result-bad-content-hash` | `content_hash` is not `sha256:` plus 64 lower-case hex characters | Reject: content identity is malformed |

## Policy cases

| ID | Invalid condition | Required rejection |
|---|---|---|
| `policy-write-mode` | `mode` is not `read-only` | Reject: this node has no write mode |
| `policy-empty-repository-allowlist` | `allowed_repositories` is empty | Reject: source access must be scoped |
| `policy-empty-path-allowlist` | `allowed_paths` is empty | Reject: content access must be scoped |
| `policy-audit-disabled` | Audit is disabled or chain requirement is false | Reject: un-audited source access is not permitted |
| `policy-unbounded-limits` | Required size, result, or rate limits are absent | Reject: source responses must be bounded |
