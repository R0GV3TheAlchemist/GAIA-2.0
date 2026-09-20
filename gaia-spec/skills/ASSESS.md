# Session assessment + badges

User starts the session. No ambient scoring.
Crate already on main: `gaia-skills::{Session, Badge, hidden_profile_api}`.
Issue this slice: #499 / #119.

## Session

`Session::start(false)` → `AmbientDenied`.
`start(true)` sets camera/mic flags true as *session state*, not a live capture stack.
`end()` / `withdraw()` set camera and mic false.
That is the acceptance. Do not add an on-device drill product here.

## Badge

`Badge::mint(skill_id, clinical)`:
- clinical true → `ClinicalCert` (no practice license)
- otherwise skill id, level `novice`, issuer `gaia-skills-fixture`, owner key `owner-local`

Third parties verify the badge fields. `hidden_profile_api()` stays false.
Open Badges 3.0 is a *name* for that export shape, not a live issuer network.

## Refuse

- ambient scoring
- clinical practice license
- hidden profile API
- live capture stack
