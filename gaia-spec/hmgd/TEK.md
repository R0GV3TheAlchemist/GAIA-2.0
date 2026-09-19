# HMGD TEK gate

Traditional Ecological Knowledge is not a harvest target.
Pairs with UKD #88 (consent before ingest).

## Default

- `sealed_rite()` → `Err(Sealed)`
- `songlines()` → `Err(Sealed)`
- `HmgdProfile::new()` is valid and empty
- `join_room(false)` does not enroll outsiders

## Consent

No TEK row is ingested without an explicit grant from the source community.
This slice does not implement a grant protocol. It only keeps the vault empty.

## Refuse

- scrape country / community ritual corpora
- mine denomination from chat
- rewrite climate science as spirit (`climate_as_spirit`)
- live MCP or bio ingest as a TEK backdoor
