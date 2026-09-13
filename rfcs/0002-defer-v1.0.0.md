# RFC 0002: Defer GAIA 2.0 tag `v1.0.0`

- Status: Proposed
- Layer: release / governance
- Breaking: no
- Issues: #32, #7

## Summary

Do not tag `v1.0.0` until Phases 2–4 are closed or each open epic has an
explicit deferral RFC. Phase 0 and Phase 1 are closed. Phases 2–4 are not.

## Decision

The current tree is `0.1.0-dev`.

A `v1.0.0` tag would mean:

1. Epics #4, #5, and #6 are closed, or each remaining work item is listed
   here as deferred with a reason.
2. Release artifacts are signed and published.
3. `gaia init --profile=developer` works from a published SDK, not only
   from an in-process session in this repo.

None of those three are true today.

## Deferred on purpose

| Item | Why it is not v1 |
| --- | --- |
| Ratatui / Axum / gRPC | L6 adapters are in-process |
| React + Vite Studio | HTML snapshot only |
| Whisper / LLaVA / Flutter | Not wired |
| Live Slurm / MPI | Local pull queue only |
| On-the-wire federation | In-process peering only |
| Signed GitHub Release | No tag, no artifacts |

## Alternatives rejected

- Tag `v1.0.0` now so the blueprint calendar looks finished.
- Rewrite README install commands to imply a published binary.
