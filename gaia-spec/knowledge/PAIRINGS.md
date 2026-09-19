# Pairings (pattern, not stack)

Every listed crystal may name **exactly one secondary**.
The pair is an alignment pattern. It does not emit a new effect.
If the pair would create chaos, it is refuse — not a clever combo.

## Pattern

```
primary  →  one secondary
set claim_class = listed-only
set effects = empty
chaos? → refuse
```

Tertiary / quaternary are later overlays on a valid secondary, never a way around it.
The fifth remains the alignment check on the whole pattern: still law, still no grant.

## What "match" means here

A secondary matches when at least one holds:

1. **Same system** (two trigonal, two cubic) — lattice agrees.
2. **Teaching contrast** (isotropic next to uniaxial) — prism lesson, not a fight.
3. **Paragenesis** — they actually grow together in rock.

A secondary does **not** match because a shop, a chakra chart, or The Secret said so.

## Chaos (refuse the pair)

- Either ID missing from the catalog
- Either `claim_class` is prohibited
- Pair treated as a tool, medicine, vault key, or network grant
- Two chemistry hazards stacked as "work" (galena + adamite, galena + bumblebee)
- Trade name + trade name inventing a new species
- Secondary used to cancel a hazard

Hazard on either member is inherited by the pair. It is never erased.

## Column

`crystals.csv` may grow `secondary_id`.
Until a row has a filled secondary, it is primary-only and still valid.
Do not invent matches to look complete.

See `SPECTRUM.md`, `CRYSTALS.md`, `crystals.csv`.

## First pass (#413)

Filled **130 / 172**. Left **42** empty on purpose.

Left empty includes chemistry rows that must not be "completed" by a partner
(adamite, galena, vanadinite, wulfenite, bumblebee-jasper, tremolite, sulfur,
amazonite/lead, cobaltoan-calcite, blue-halite / pink-halite as ingest work)
and trade titles that would only pair to another title
(super-seven, prophecy-stone, auralite-23, septarian, shiva-lingam).

Trade+trade jasper titles were stripped even when the rock looks related.

Prism teaching pair on this pass: `clear-quartz` / `fluorite` / `clear-calcite`.
Paragenesis example: `azurite` ↔ `malachite`. Quartz colors point at `clear-quartz`.
Effects stay empty. No fifth.
