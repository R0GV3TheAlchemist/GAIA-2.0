# CI status checks

Workflow: `.github/workflows/ci.yml`

| Job | Required on `main`? (Phase 0) |
| --- | --- |
| `spec-schemas` | Should be required once branch protection is enabled |
| `rust-sdk` | Should be required |
| `python-sdk` | Should be required |
| `typescript-sdk` | Should be required |
| `markdown` | Optional |
| `rust-placeholders` | Informational only |

Branch protection is **not** enforced in this change. A maintainer can tick
the four SDK/spec jobs as required later without editing the workflow.
