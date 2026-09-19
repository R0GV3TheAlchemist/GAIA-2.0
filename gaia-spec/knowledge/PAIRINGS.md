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
Table: `pairings-413.csv`.

## Second pass (remainder)

Resolved the 42:

- Filled **15** — lattice / paragenesis / teaching only. Table: `pairings-remainder.csv`.
- Refused **27** — stay empty. Table: `pairings-refuse.csv`.

Filled: aegirine↔bronzite, amblygonite→lepidolite, blue-apatite→turquoise,
brucite→magnesite, chaoite→shungite, credite↔creedite,
dumortierite→clear-quartz, grape-agate→amethyst, meteorite→hematite,
purpurite→rhodochrosite, seraphinite→fuchsite, ulexite→selenite,
zincite→hematite.

Refused: arsenate/lead/cobalt/asbestos/sulfur/soluble-ingest rows;
trade+trade jasper titles; assemblage and sacred-object titles; lab-hopper bismuth.

Running total: **145 / 172** have a secondary. **27** remain empty by law.
Effects stay empty. No fifth. No dual-sign. #341 stays open.
