# HSPD Phase 2 — GAIAN Practice Profile

**Status:** Listed  
**Issues:** #132 (meta), #135 (epic), #141 (profile slice)  
**Crate:** `gaia-hspd` (Apache-2.0)  
**Not:** HSPD v1.0. Not a clinic. Not a prescription. Not a genetic test. Not a required spit-kit.

`hspd_v1_tagged()` MUST return `false`.
`gaia_prescribes()` MUST return `false`.
`dna_required` MUST be `false`.
`infer_from_photo()` MUST return `Err(InferFromPhotoBlocked)`.
`infer_actn3()` MUST return `Err(GeneticInference)`.

---

## 1. Purpose

Phase 2 specifies the `HspdProfile` declaration surface — how a user may
declare natural traits, trained practices, and augmentation interests into
their GAIAN vault. No inference from photos, files, or automated signals.
No required genetic kit. No prescription, dose, or protocol field.

---

## 2. `HspdProfile` Declaration Rules

`HspdProfile::declare(kind: KindClass, realm: Realm, label: &str) -> Result<HspdProfile, HspdError>`

| Kind | Who may declare | Constraint |
|---|---|---|
| `Natural` | Adult self-declare only | Name only; MUST NOT be treated as a gene call |
| `Trained` | Any age as practice interest | Reuse skill-mastery language; not a superpower actuator |
| `Augmented` | Adult only | Under-16 surgical/pharmacologic → `ChildTag` required; under-18 → `ChildGenetic` flag |
| `Genetic` | User self-declare only | User-held report they already possess MAY be named; no spit-kit flow; no inference |

### Field rules

```
genetic_indicators:   Vec<String>   — starts empty; user-named only
dna_required:         bool          — MUST be false always
gaia_enabled:         bool          — false at Phase 2
consent_token:        Option<String>— required for Augmented + Genetic declarations
```

**MUST rules**
- `dna_required` MUST be `false` at all phases through v1.0.
- `genetic_indicators` MUST only be populated by explicit user text input — no automated fill.
- `Augmented` node under age 16 MUST attach `ChildTag` → `Err(ChildTagRequired)` otherwise.
- `Genetic` node for user under age 18 MUST attach `ChildGenetic` flag.
- `infer_from_photo()` MUST return `Err(InferFromPhotoBlocked)` — no photo-to-identity fill.
- `infer_actn3()` MUST return `Err(GeneticInference)` — no ACTN3/MSTN auto-call.

---

## 3. Vault Integration

| Operation | Behaviour |
|---|---|
| `Vault::write(profile)` | Requires signed consent token; unsigned write → `Err(UnsignedVaultWrite)` |
| `Vault::wipe()` | Returns deletion-receipt fixture; MUST be callable at any time |
| `Vault::export()` | User-initiated only; no silent export |
| `Vault::read()` | No hidden profile read endpoint — `hidden_profile_api()` stays `false` |

---

## 4. Prohibition Surface

| Prohibition | Error |
|---|---|
| Photo → identity/genetic fill | `Err(InferFromPhotoBlocked)` |
| ACTN3 / MSTN auto-call from raw file | `Err(GeneticInference)` |
| Required genetic kit | Compile-time: `dna_required` field blocked |
| Twin-as-person | `Err(TwinAsPersonBlocked)` |
| Prescription / dose / protocol | `Err(MedicalDiyPath)` |
| Unsigned vault write | `Err(UnsignedVaultWrite)` |

---

## 5. What This Phase Does Not Do

- Does not prescribe, dose, or protocol any augmentation.
- Does not infer genetics from any automated signal, photo, or file.
- Does not require a genetic kit.
- Does not run practice assessments — Phase 3 (#142).
- Does not enable opt-in collective sessions — Phase 3 (#142).
- Does not tag HSPD v1.0.

---

## 6. Acceptance Gate

- [ ] `HspdProfile::declare(Augmented, _, _)` with age < 16 and no `ChildTag` → `Err(ChildTagRequired)`
- [ ] `HspdProfile::declare(Genetic, _, _)` with age < 18 → `ChildGenetic` flag set
- [ ] `infer_from_photo()` → `Err(InferFromPhotoBlocked)`
- [ ] `infer_actn3()` → `Err(GeneticInference)`
- [ ] `dna_required == false`
- [ ] `Vault::write(unsigned)` → `Err(UnsignedVaultWrite)`
- [ ] `Vault::wipe()` returns deletion-receipt fixture
- [ ] `gaia_prescribes()` → `false`
- [ ] `hspd_v1_tagged()` → `false`
- [ ] `cargo test -p gaia-hspd` green

---

## 7. Cross-References

- Phase 0: `gaia-spec/hspd/PHASE-0.md` (#133)
- Phase 1: `gaia-spec/hspd/PHASE-1.md` (#134)
- Profile spec: `gaia-spec/hspd/PROFILE.md`
- Prohibited: `gaia-spec/hspd/PROHIBITED.md`
- Ethics: `gaia-spec/hspd/ETHICS.md`
- Issues: #133 (Phase 0), #134 (Phase 1), #135 (epic), #136 (Phase 3 epic), #142 (Phase 3 slice)
- Next: HSPD Phase 3 (#136, #142) — practice-only assessment, opt-in collective sessions
