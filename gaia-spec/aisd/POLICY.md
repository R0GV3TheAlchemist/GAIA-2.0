# AISD component policy map

Routing policy, not marketing. Machine-readable file already on main: `gaia-spec/aisd/policy.json`.
Crate already on main: `gaia-aisd::{allow, ComponentPolicy}`.
Issue this slice: #483 / #129.

## Table (names)

| component | min safety | human gate | notes |
| --- | --- | --- | --- |
| Agents | L3 | yes | `ComponentPolicy::agents()` |
| Earth Twin | — | science claims | reuse #48 / #104; no live forecast |
| GAIAN | — | watermark/consent | reuse #66; twin-as-record |
| UKD | — | TEK sealed default | #88 |

## Deny

`allow(task, maturity)` denies a multi-day task at ≤ L2 → `InsufficientMaturity`.
That is the unsupervised multi-day coding gate. Not a running coder.

`policy.json` already lists `deny: ["unsupervised-multi-day-coding"]` and `agents.min_safety = 3`.

## Refuse

- agent tool use below safety L3
- live agent corporation
- AISD v1.0
