# Nine system-twin profiles (#56)

Each row is a **profile**, not a live twin. Models named here are listed research (#48), not shipped weights.
Enforced copy: `gaia-earth::SystemProfile`.

| System | Variables | Sources | Models (listed) | Cadence | Live in phase |
| --- | --- | --- | --- | --- | --- |
| Atmosphere | temp, wind, precip | stations | GraphCast/AIFS | 6-hourly | 2 |
| Ocean | SST, AMOC | Argo/NDBC | NEMO-stub | hours | 2 |
| Land | cover, fire | Landsat/Sentinel | TerraMind-stub | 30m–1m | 2 |
| Biosphere | species, biome | GBIF | biodiversity-stub | days | 2 |
| Cryosphere | ice, snow | ICESat-2 | mass-stub | days | 1 |
| Lithosphere | quake | USGS | catalog | minutes | 1 |
| Anthroposphere | cities, emissions | OSM | inventory-stub | days | 2 |
| Technosphere | energy, transport | public stats | inventory-stub | hours | 2 |
| Magnetosphere | storm, aurora | SWPC | space-stub | minutes | 1 |

## Explicit gaps

- **Noosphere** — knowledge/culture layer is not a tenth operational twin
- **Commercial imagery** — not a first-party source; do not ingest Maxar/Planet as a default feed

Machine table: `systems.csv`.
