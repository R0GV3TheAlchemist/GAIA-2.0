# GAIAN skill profile in the local vault

Stored with the twin-as-record (#65 / #141). Not a cloud persona.
Crate already on main: `gaia-skills::{VaultProfile, SkillProfile}`.
Issue this slice: #495 / #117.

## Offline CRUD (what exists)

- `VaultProfile::offline()` — local profile, `clusters == false`
- `SkillProfile::local()` — `sync == false`, `inferred_se == false`
- `set_goal` / `delete` — edit and wipe on device
- `toggle_clusters(on, age)` — explicit toggle; age &lt; 16 + on → `ChildEiBlocked`
- `enable_inferred_se` same child gate
- `enable_sync(false)` → `SyncDenied`

Evidence types named in the issue (self_report, peer_feedback, portfolio) are **not** struct fields on main. Do not invent them. Silent telemetry is refused by ambient-session rules (#119).

## Refuse

- child inferred EI / clusters under 16
- sync without opt-in
- live cloud profile / twin-as-person
