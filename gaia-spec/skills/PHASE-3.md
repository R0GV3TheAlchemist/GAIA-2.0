# Skills Phase 3 — Consenting Assessment and Open Badges

**Status:** Listed  
**Issues:** #108 (epic), #111 (this slice)  
**Crate:** `gaia-skills` (Apache-2.0)  
**Not:** Skills v1.0. Not a clinical practice license. Not a live badge issuer network. Not ambient scoring.

`hidden_profile_api()` MUST return `false`.
No ambient scoring. No live capture stack. No clinical license.

---

## 1. Purpose

Phase 3 specifies the consenting assessment session (`Session`), the
`Badge::mint()` contract for Open Badges 3.0-shaped exports, the clinical
certification guard, and the prohibition on ambient scoring and hidden
profile APIs. No live badge issuer network, no live capture stack.

---

## 2. `Session` Contract

`Session::start(consent: bool) -> Result<Session, SkillsError>`

| Condition | Result |
|---|---|
| `consent = true` | `Ok(Session)` with `camera = true`, `mic = true` as session state flags |
| `consent = false` | `Err(SkillsError::AmbientDenied)` |

`Session::end()` sets `camera = false`, `mic = false`.
`Session::withdraw()` sets `camera = false`, `mic = false` and voids any in-progress badge.

| Rule | Constraint |
|---|---|
| No ambient scoring | `start(false)` MUST return `AmbientDenied` — no silent session |
| Camera/mic flags | Session state only; MUST NOT represent a live capture stack |
| Withdraw at any time | `withdraw()` MUST be callable at any point in the session |
| No data retention | Session data MUST NOT be retained after `end()` or `withdraw()` without explicit export consent |

---

## 3. `Badge::mint()` Contract

`Badge::mint(skill_id: &str, clinical: bool) -> Badge`

| Condition | Result |
|---|---|
| `clinical = false` | `Badge { skill_id, level: novice, issuer: "gaia-skills-fixture", owner_key: "owner-local" }` |
| `clinical = true` | `Badge::ClinicalCert` — flagged as clinical; MUST NOT carry a practice license |

### Badge fields (Phase 3 fixture)

```
skill_id:   String  — must match a Phase 1 catalog entry
level:      String  — Dreyfus level string; default "novice"
issuer:     String  — "gaia-skills-fixture" at Phase 3
owner_key:  String  — "owner-local" at Phase 3
clinical:   bool    — true = ClinicalCert flag; no practice license
```

### Clinical cert guard

`ClinicalCert` MUST NOT be interpreted as a clinical practice license.
UI MUST display: *"This badge is not a clinical practice licence."*
No code path may issue a practice license from `Badge::mint()`.

---

## 4. Open Badges 3.0 Export Shape

Open Badges 3.0 is the **export shape name** — not a live issuer network.

| Field | OB 3.0 mapping | Phase 3 value |
|---|---|---|
| `@context` | `https://www.w3.org/ns/credentials/v2` | Fixture stub |
| `type` | `["VerifiableCredential", "OpenBadgeCredential"]` | Fixture stub |
| `issuer.id` | Issuer DID or URL | `"gaia-skills-fixture"` |
| `credentialSubject.achievement.id` | Skill URI | `"skill:<slug>"` |
| `credentialSubject.achievement.name` | Skill name | Phase 1 catalog name |

Source: IMS Global — *Open Badges Specification 3.0*, 2023.
Third parties verify the badge fields. No live issuer network at Phase 3.

---

## 5. `hidden_profile_api()` Gate

`hidden_profile_api() -> bool` MUST return `false` at all phases through v1.0.

| Rule | Constraint |
|---|---|
| No hidden endpoint | No undocumented profile read/write API |
| No silent export | All profile exports require user-initiated action |
| Third-party verification | Badge fields are verifiable; profile is not hidden |

---

## 6. Ambient Scoring Prohibition

| Prohibition | Constraint |
|---|---|
| No ambient session | `Session::start(false)` → `AmbientDenied` — no exceptions |
| No background scoring | MUST NOT score skill level without active `Session` |
| No camera/mic capture | Session flags are state only — no live capture stack |
| No inferred scoring | MUST NOT infer skill level from passive usage data |

---

## 7. What This Phase Does Not Do

- Does not connect to a live Open Badges issuer network.
- Does not issue a clinical practice license.
- Does not run ambient or background scoring.
- Does not expose a hidden profile API.
- Does not run opt-in collective sessions — Phase 4 (#112) or HMGD (#158).
- Does not tag Skills v1.0.

---

## 8. Acceptance Gate

- [ ] `Session::start(false)` → `Err(AmbientDenied)`
- [ ] `Session::start(true)` sets `camera = true`, `mic = true`
- [ ] `Session::withdraw()` sets `camera = false`, `mic = false`
- [ ] `Badge::mint(id, false)` returns badge with `issuer = "gaia-skills-fixture"`
- [ ] `Badge::mint(id, true)` returns `ClinicalCert` with no practice license
- [ ] `hidden_profile_api()` → `false`
- [ ] No ambient scoring code path exists
- [ ] `skills_v1_tagged()` → `false`
- [ ] `cargo test -p gaia-skills` green

---

## 9. Cross-References

- Phase 0: `gaia-spec/skills/PHASE-0.md` (#108)
- Phase 1: `gaia-spec/skills/PHASE-1.md` (#109)
- Phase 2: `gaia-spec/skills/PHASE-2.md` (#110)
- Assessment: `gaia-spec/skills/ASSESS.md`
- Vault: `gaia-spec/skills/VAULT.md`
- Issues: #108 (Phase 0), #109 (Phase 1), #110 (Phase 2), #111 (this), #112 (Phase 4)
- Next: Skills Phase 4 (#112) — opt-in matching, culture/TEK, v1.0
