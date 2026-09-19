# HMGD ethics

Not a spellcaster. Prayer is not emergency care. TEK sealed.
Reading this file does not grant a rite or a recipe.
Crate already on main: `gaia-hmgd::{principles, prohibited, sealed_rite, songlines}`.
Issue this slice: #459 / #161.

## Principles (`principles()`)

| Name | Meaning (listed only) |
| --- | --- |
| sovereignty | Source communities keep the grant. Outsiders are not enrolled by default. |
| humility | A catalog row is not proof of efficacy or holiness. |
| non-harm | No hex, curse, ordeal, or emergency-care substitution. |
| respect | Belief is declared, never inferred from chat (`infer_belief` errors). |
| integration | Science + tradition may be *named* together. Tradition is not overwritten as climate-as-spirit (`climate_as_spirit` errors). |
| access | Public-domain *descriptions* may be listed. Restricted knowledge stays sealed. Access is not extraction. |

## Appropriation test

- **Public-domain description** — open literature already published by the source community or long in the commons. May appear as a stub node with `EvidenceClass::Traditional` and empty effects.
- **Restricted knowledge** — closed rites, songlines, unpublished TEK, initiation material. `sealed_rite()` and `songlines()` return `Err(Sealed)`. Collections stay empty by default.

A name in `REALMS` is not a grant.

## Refuse

- dose, recipe, brew instructions
- curse / hex payload
- selling a closed rite
- child as source or ordeal subject
- HMGD v1.0 / actuator
