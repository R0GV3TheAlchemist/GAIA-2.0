# UKD Phase 4 — modes + v1.0 gate

Five GAIAN modes. Six UN language codes. Not 100 languages. Not a release.
Crate already on main: `gaia-ukd::{GaianMode, teach_offline, earth_twin_cites, tek_export, UN_LANGS, ukd_v1_tagged}`.
Issue this slice: #553 / #82.

## What exists

`GaianMode::all()` — Discover, Explore, Learn, Research, Contribute.
`teach_offline(false, _)` → `UnknownNode`; true returns an offline pack string.
`earth_twin_cites` needs a nonempty UKD id.
`tek_export(false)` → `NoAgreement`.
`UN_LANGS` has six codes. `switch_ui` only accepts those.
`ukd_v1_tagged()` stays **false**.
`release_notes()` already says no UKD v1.0 tag.

## Refuse

- UKD v1.0 tag
- 100-language product
- TEK export without grant
