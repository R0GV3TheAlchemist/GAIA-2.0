# Crystal catalog (evoke only)

Rows are names. They are not capabilities.
`claim_class` is `listed-only`. `effects` is always empty.
A row must never grant tools, memory, Autonomy 4–5, medical protocol, or network.

**Required law:** `SPECTRUM.md`, `ELEMENTS.md`, then `PAIRINGS.md`.
`secondary_id` exists on the table and stays empty until the pairing pass.

One catalog: `crystals.csv` (first absorbed list, A–zincite, 172 rows).
`crystals-rz.csv` is a merge pointer only.

## Schema

Field definitions are canonical in `crystal_schema.json`.
The table below is the human-readable summary only.

| Field | Type | Notes |
|---|---|---|
| `id` | string | Lowercase hyphenated; unique across catalog |
| `label` | string | Display name |
| `kind` | enum | `mineral \| glass \| organic \| fossil \| metal \| treated \| trade` |
| `system` | enum | Crystal system (`cubic`, `trigonal`, …, `amorphous`, `mixed`, `none`) |
| `optic` | enum | `isotropic \| uniaxial \| biaxial \| mixed \| none` |
| `claim_class` | const | Always `listed-only` — any other value is a violation |
| `effects` | string | Always empty — populated effects are prohibited |
| `hazard` | enum | `none \| arsenate \| asbestos \| copper \| lead \| metal \| soluble \| sulfide` |
| `notes` | string | Mineralogical notes; must not assert metaphysical capabilities |
| `secondary_id` | string | Reserved; empty until pairing pass |
| `chakra_id` | enum | Primary chakra or `none` — see valid values below |

## `chakra_id` valid values

| Value | Centre | Colour tradition | Frequency band |
|---|---|---|---|
| `crown` | Sahasrara (7th) | Violet / white | ~963 Hz (Solfeggio) |
| `third-eye` | Ajna (6th) | Indigo | ~852 Hz |
| `throat` | Vishuddha (5th) | Blue | ~741 Hz |
| `heart` | Anahata (4th) | Green / pink | ~639 Hz |
| `solar-plexus` | Manipura (3rd) | Yellow | ~528 Hz |
| `sacral` | Svadhisthana (2nd) | Orange | ~417 Hz |
| `root` | Muladhara (1st) | Red | ~396 Hz |
| `none` | — | Unassigned | — |

**Assignment rule:** `chakra_id` is a catalogue annotation only — it records
a traditional association, not a functional grant. A populated `chakra_id`
confers no additional `claim_class` elevation and does not expand
`effects` beyond the empty-string constraint.

## Canonical chakra specimens (35 rows, 5 per centre)

| Centre | Specimens |
|---|---|
| `crown` | amethyst, clear-quartz, fluorite, ajoite, sugilite |
| `third-eye` | labradorite, lapis-lazuli, azurite, moonstone, sodalite |
| `throat` | aquamarine, blue-kyanite, blue-lace-agate, hemimorphite, turquoise |
| `heart` | rose-quartz, green-jade, emerald, prehnite, watermelon-tourmaline |
| `solar-plexus` | citrine, amber, tiger-eye, honey-calcite, serpentine |
| `sacral` | carnelian, fire-opal, imperial-topaz, garnet-hessonite, wulfenite |
| `root` | hematite, black-obsidian, smoky-quartz, garnet-almandine, jet |
