# Knowledge opening ledger

**Status:** listed ledger  
**Issue:** #699  
**Modelled on:** ancestor `OPENING-LEDGER.md` (human knowledge plane)

Record accepted batches. Do not treat a ledger line as a mounted corpus.

## Batch 0 — naming baseline

| Field | Value |
|---|---|
| Date | 2026-09-21 |
| Issue | #696 |
| PR | #702 |
| Artifact | `DOMAIN-NAMING.md`, `legacy-relabel-map.json` |
| Runtime | false |

## Batch 1 — listed catalog bind

| Field | Value |
|---|---|
| Date | 2026-09-21 |
| Issues | #691 #692 #693 #694 #695 #697 #698 #699 #700 #701 |
| Artifacts | this directory + `scripts/validate_catalog.py` |
| Rows seeded | see `catalog.json` `counts.total` |
| Content copied from ancestor subject folders | none |
| Runtime | false |

## Import rules

1. A later content PR names one canonical id and one ancestor path.
2. The PR adds a ledger row with source revision.
3. `scripts/validate_catalog.py` must stay green.
4. TEK / sacred content needs a grant record before copy.
5. Refused titles get a `BLOCKED` row, not a silent omit.

## Acceptance for #699

- [x] Ledger file exists.
- [x] Batch 0 and Batch 1 recorded.
- [x] Further imports remain explicit.
