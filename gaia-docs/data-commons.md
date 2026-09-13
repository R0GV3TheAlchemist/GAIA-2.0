# Data Commons v0 (#34)

In-process catalog in `gaia-earth`. Not a lake.

Policy for ingest tickets: [data-commons-charter.md](data-commons-charter.md).

## What exists

`Commons::seed()` exposes four fixture collections:

| id | domain | source (fixture) | tier |
| --- | --- | --- | --- |
| `sentinel-2-l1c-fixture` | satellite | Copernicus Data Space | Curated |
| `ground-weather-fixture` | weather | Open Weather stations | Raw |
| `gbif-occurrence-fixture` | biodiversity | GBIF | Curated |
| `usgs-seismic-fixture` | seismic/ocean | USGS | Raw |

Every query is labeled `Synthetic` and must carry uncertainty.

## What does not exist

- Apache Iceberg
- MinIO
- Kafka / Flink
- A public STAC or openEO portal
- Live Copernicus / GBIF / USGS pulls

## Adding a connector later (#41 / #42)

1. Add a `Collection` with license, quality tier, and provenance.
2. Cite a license class and uncertainty rule from the charter.
3. Keep live network I/O behind an explicit feature. Default stays fixture.
4. Writes belong in Raw first. Curated is a derived table, not an overwrite.
