# HMGD prohibited practices

Names only. This file is not a how-to.
Matches `gaia-hmgd::prohibited()`.

| id | crate token | why refused |
| --- | --- | --- |
| hex-curse-kits | hex-curse-kits | `parse_node` with curse → `CurseForbidden` |
| ritual-drug-preparation | ritual-drug-preparation | `brew()` → `RecipeForbidden`; dose/recipe in node text refused |
| selling-closed-rites | selling-closed-rites | `sell_closed_rite()` → `SaleForbidden` |
| child-ordeals | child-ordeals | child-as-source is hard refuse |
| prayer-as-emergency-care | prayer-as-emergency-care | Prayer is not a medical license |

Sealed collections stay empty by default.
`hmgd_v1_tagged()` stays false.
