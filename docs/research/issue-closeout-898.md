# Closeout map for #898 (unit-test wave 2)

Priority 1 only. No new Cargo deps. No live transport.

| Crate | Before | After | Notes |
| --- | --- | --- | --- |
| `gaia-boot` | integration `boot_sequence.rs`, no lib tests | 3 lib tests | `boot()`, default tier2, distinct tiers |
| `gaia-gateway` | integration health/revoke | 3 lib tests | router builds, paths listed, empty agent map |
| `gaia-cli` | integration `cli_integration.rs` only | 3 lib tests on a thin `gaia_cli` lib | Binary stays `src/main.rs`; no HTTP |

Priority 2+ crates already carry lib tests from earlier waves (`gaia-ingest`, `gaia-aikd`, …). This issue does not claim the whole workspace is done.
