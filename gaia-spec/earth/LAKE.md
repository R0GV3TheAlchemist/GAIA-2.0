# In-process lake + STAC stub (#41)

Not MinIO, Iceberg, Spark, or a published catalog.

Zones: Raw (append-only) → Curated → Enriched → Serving → Archive.

`Lake::write_raw` then `promote`. Duplicate raw id → `ImmutableRaw`.

STAC-shaped string: `Lake::stac_collection(id)` and `Commons::stac_item(id)`.
openEO-shaped string: `Lake::openeo_graph(id)`.

## Add a connector

1. Name it on `Connector` (#42) or as a `Collection` on `Commons::seed`.
2. Emit an `Observation` with uncertainty (#46).
3. `write_raw` + `promote`.
4. Cite a license on `IngestTicket` (#40).
