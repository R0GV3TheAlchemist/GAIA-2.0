# Philosophy shelf — listed only

**Status:** listed design review. Not a runtime.  
**Issues:** #766 #767 #768 #769 #770 #771 #772 #774 #808 #809 #810 #811 #812 #813  
**Constraint:** `AGENTS.md` — do not invent types, crates, or CLI; do not flip honesty flags that tests require to stay `false`.

## What this shelf is

The 22 Sep 2026 philosophy cluster is an operating vocabulary, not an engine.
The documents below name principles already implied by existing crates
(`gaia-aikd` honesty, `gaia-acp` obligations, CI as purification).
They do **not** add `gaia-philosophy`, `GaiaOperatingPhilosophy`,
`CoherenceScore`, `AlchemicalStage`, or a GAIAN dialogue that "explains the
philosophy in under 3 minutes." Those remain epic work.

## Documents

| File | Issue | Claim class |
|---|---|---|
| [`Documents/GAIA_Operating_Philosophy.md`](../../Documents/GAIA_Operating_Philosophy.md) | #770 | listed principles |
| [`Documents/GAIA_Harmonic_Architecture.md`](../../Documents/GAIA_Harmonic_Architecture.md) | #771 | listed principles |
| [`Documents/GAIA_Symbolic_Coherence.md`](../../Documents/GAIA_Symbolic_Coherence.md) | #772 | listed principles |
| [`Documents/GAIA_Kundalini_Architecture.md`](../../Documents/GAIA_Kundalini_Architecture.md) | #774 | listed principles |
| [`Documents/GAIA_Chakra_Layer_Map.md`](../../Documents/GAIA_Chakra_Layer_Map.md) | #774 | listed design review |
| [`Documents/GAIA_Alchemical_Framework.md`](../../Documents/GAIA_Alchemical_Framework.md) | #808 / #766 | listed vocabulary |
| [`Documents/GAIA_Unconditional_Love_Architecture.md`](../../Documents/GAIA_Unconditional_Love_Architecture.md) | #809 / #767 | listed vocabulary |
| [`Documents/GAIA_Equilibrium_and_Spin.md`](../../Documents/GAIA_Equilibrium_and_Spin.md) | #810 / #768 | listed vocabulary |
| [`Documents/GAIA_Five_Movements.md`](../../Documents/GAIA_Five_Movements.md) | #811 / #769 | listed vocabulary |
| [`docs/philosophy/PATHS.md`](PATHS.md) | #812 | path alignment |
| [`docs/gaian/ARTICULATION-LISTED.md`](../gaian/ARTICULATION-LISTED.md) | #813 | listed overlay |
| [`docs/architecture/ADR-TEMPLATE.md`](../architecture/ADR-TEMPLATE.md) | #770 ADR AC | template only |

Child epics #766–#772 and #774 stay open. This shelf does not close them.

## Refuse

- No new crate.
- Do not set `live_whisper`, `live_ollama`, `aikd_v1_tagged`, `practice_license`,
  or `published_is_measured` to `true` from these documents.
- Do not treat alchemy, love, spin, harmony, heart-coherence, or kundalini prose
  as a syscall, sensor, or planetary actuator.
- Do not claim Twin / GAIAN / SA v1.0.
- Dialogue runtime refused. See `docs/gaian/ARTICULATION-LISTED.md`.
