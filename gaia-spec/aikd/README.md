# gaia-spec/aikd

Listed-only open-model registry (#100 / #457).
Reading this directory does not run a model or start an eval harness.

Crate already on main: `gaia-aikd` (`model_card`, `BenchRow`, `system_tags`, `published_closed_score_is_ours`, `aikd_v1_tagged`).
This folder is the data shelf those symbols already enforce.

## Files

| File | What |
| --- | --- |
| models.csv | local fixture + closed reference rows |
| tiers.csv | T1–T5 names only |
| tags.csv | 19 system-type tags |
| benches.csv | published vs gaia_measured columns |

Claim class: `listed-only`. Effects empty.
`aikd_v1_tagged()` stays false.
`published_closed_score_is_ours()` stays false.

## Refuse

- GPT / Claude published score as `gaia_measured=true`
- Qdrant / MemOS / Ollama-as-default / live harness in this slice
- registry row as a practice license or actuator
