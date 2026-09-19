# HSPD / GAIAN superpower profile

Declare in the vault. Do not infer DNA.
Reading this file does not create a person or open a clinic.
Crate already on main: `gaia-hspd::{HspdProfile, hspd_v1_tagged}`.
GAIAN already refuses `infer_from_photo` and unsigned vault writes.
Issue this slice: #463 / #141.

## Entries (listed only)

| kind | who may declare | notes |
| --- | --- | --- |
| natural | adult self-declare | name only; not a gene call |
| trained | any age as *practice interest* | reuse skill-mastery language from #117; not a superpower actuator |
| augmented | adult only | under-16 surgical/pharmacologic tags → `ChildTag` |

`genetic_indicators` starts empty. `dna_required` is false.
A user-held report they already possess may be *named*. There is no spit-kit flow.

## Child

- under 18: `child_genetic` → `ChildGenetic`
- under 16: surgical / pharmacologic tags → `ChildTag`
- trained practice interests only

## Delete / export

GAIAN `Vault::wipe` already returns a deletion-receipt fixture (#65).
This slice does not add crypto or cloud sync.

## Refuse

- ACTN3 / MSTN auto-call from raw files (`infer_actn3` errors)
- photo → identity fill (`infer_from_photo` errors)
- required genetic kit
- twin-as-person / GAIAN v1.0 / HSPD v1.0
