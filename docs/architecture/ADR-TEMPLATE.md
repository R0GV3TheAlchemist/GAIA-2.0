# ADR-NNNN: Title

**Status:** proposed | accepted | superseded | withdrawn  
**Date:** YYYY-MM-DD  
**Issue:** #

## Context

What forces the decision?

## Decision

What will we do? Name only symbols that already exist on `main`, or mark
new symbols as *proposed* and leave them unimplemented until a dedicated issue.

## Consequences

What becomes easier, harder, or refused?

## Five Movements Analysis (#770 / #769)

Required section for architecture ADRs. Incomplete if any row is blank.

| Movement | Answer for this decision |
|---|---|
| Divergence — what possibility space opens? | |
| Insurgence — what may now challenge what? | |
| Allegiance — what must stay committed? | |
| Convergence — how does this join the rest of the tree? | |
| Ascendence — what becomes possible that was not? | |

If a movement has no answer, the design is incomplete. Do not ship a silent
“N/A” to close an epic.

## Honesty

- [ ] Does not flip `live_*` or `*_v1_tagged` flags
- [ ] Does not invent crates listed in `AGENTS.md` refuse
- [ ] Tests named in CI still describe the real tree
