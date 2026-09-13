# Data lake v0 (#41)

In-process zones in `gaia-earth::Lake`. Not a running MinIO or Iceberg cluster.

## Zones

Raw → Curated → Enriched → Serving → Archive.

Only Raw and Curated are implemented. Raw is append-only. Curated is a copy, not an overwrite of Raw.

## STAC / openEO

`Lake::stac_collection` and `Lake::openeo_graph` return stub text. There is no HTTP portal.

Persistent IDs use `gaia:10.placeholder/{id}` until a real DOI prefix exists.

## Adding a connector

1. Write the observation into Raw with `Lake::write_raw`.
2. Promote to Curated after QC.
3. Do not put network I/O in the default build.
4. A later compose file may start MinIO. CI does not run it today.

See `deploy/minio-skeleton.yml` for the not-yet-running service list.
