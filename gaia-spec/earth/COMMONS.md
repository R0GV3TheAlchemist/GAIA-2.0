# Data Commons policy (#40)

Not a nonprofit foundation. This page is the charter an ingest ticket must cite.

## Licenses

| Kind | SPDX |
| --- | --- |
| Raw sensor | CC0-1.0 |
| Processed product | CC-BY-4.0 |
| Model weights (if any later) | Apache-2.0 |
| Specs | CC0-1.0 |

## Mandatory on every ingest

FAIR identifiers, STAC-shaped collection id, uncertainty, quality tier, provenance, license (`IngestTicket` in `gaia-earth`).

## Ethics (listed)

equity · transparency · uncertainty-as-truth · non-weaponization · data sovereignty · species inclusion · 7th-generation impacts

Forbidden purposes: targeting, individual surveillance, harm. Same gate as `allow_purpose`.

National node: if `residency` is set, `process_in` must match.
