# Nine system-twin profiles (#56)

Each profile is a catalog entry, not a live model. Phase is when the blueprint says it becomes live — it is not live in this tree.

| System | Variables | Sources | Models | Cadence | Phase |
| --- | --- | --- | --- | --- | --- |
| Atmosphere | temp, wind, precip | stations, reanalysis | GraphCast, AIFS, ESFM, Aurora | 6-hourly → real-time | 2 |
| Ocean | SST, AMOC, heatwaves | Argo, NDBC, altimetry | NEMO + neural state | hours–days | 2 |
| Land | cover, yield, fire | Landsat, Sentinel | TerraMind, LSM | 30 m → 1 m | 2 |
| Biosphere | species, biome | iNaturalist, eBird, GBIF | biodiversity twin | days | 2 |
| Cryosphere | ice, permafrost, snow | ICESat-2, GRACE-FO, Sentinel-1 | mass-balance stubs | days | 1 |
| Lithosphere | quake, volcano, slide | USGS, EMSC, InSAR | catalog only | minutes | 1 |
| Anthroposphere | cities, food, emissions | OSM, WorldPop, night lights | inventory stubs | days | 2 |
| Technosphere | energy, comms, transport | public flow stats | inventory stubs | hours | 2 |
| Magnetosphere | storm, GPS, aurora | NOAA SWPC, Swarm | space-weather stubs | minutes | 1 |

## Explicit gaps

- Noosphere / knowledge layer is not a tenth system twin yet.
- Commercial imagery is not a default source.
