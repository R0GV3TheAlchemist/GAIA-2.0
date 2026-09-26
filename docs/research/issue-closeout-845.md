# Closeout map for #845

| Category | Evidence |
| --- | --- |
| Cross-module contracts | `gaia-integrity/tests/contracts.rs` + ingest_to_memos + memos_to_aikd (#935) |
| Falsification | `tests/falsification/` four corpora + marker gate |
| RL-001 Terra hex | `tests/canon/test_canon_integrity.py` already on main |
| Regression policy | `CONTRIBUTING.md` + #936 |
| Smoke | `tests/smoke/test_smoke.py` |
| CI | `.github/workflows/integration-tests.yml` advisory (`continue-on-error`) |
| Mapping schema | `gaia-spec/mapping/schema.yaml` |

## Honest leftover

#815 blind review and two independent reviewers are not this PR.
Workflow is advisory, not a required merge gate.
No sentient runtime. No live harvest.
