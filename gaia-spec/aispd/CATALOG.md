# AISPD open superpower catalog

Only tools we can *name* and already guard. Not ASI. Not a license.
Crate already on main: `gaia-aispd::{weather_node, license_exam, active_weather, wet_lab, aispd_v1_tagged}`.
Related name already on main: `gaia-aisd::graphcast` (tool card, not a live forecast).
Issue this slice: #475 / #151.

## Active open nodes

| node | crate | notes |
| --- | --- | --- |
| weather / GraphCast-class | `weather_node()` | cites #48 and #104; `score_kind=gaia_measured_pending` |
| licensing exams | `license_exam(name)` | `score_kind=reference_published` only |

`active_weather()` is a Watch *flag* (`Watch::Active`). It does not start DestinE or GraphCast ingest.
`wet_lab()` stays `Watch::Denied`.

## Human gate

Science claims used in policy stay pending until measured. Closed published scores are not `gaia_measured` (AIKD already refuses that).
No USMLE-as-license language.

## Wire only what exists

Allowed components are the symbols above plus AISD `graphcast()` name.
Do not add HTTP, orbital ingest, or a weather runtime in this slice.

## Refuse

- live GraphCast / DestinE / wet lab
- exam node as a practice license
- AISPD v1.0 / ASI / RSI / weapons-targeting
