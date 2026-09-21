# GAIAN C2PA Watermarking and v1.0 Release Gate

**Status:** Listed  
**Issues:** #63 (epic, closed), #66 (C2PA), #76 (this slice — v1.0 release gate)  
**Crate:** `gaia-gaian` (Apache-2.0)  
**Not:** A central biometric warehouse. Not a live C2PA signing service. Not a bypass for consent.

`c2pa_export_has_manifest()` MUST return `true` for every generated media export.  
`biometric_warehouse_exists()` MUST return `false`.  
`gaian_v1_tagged()` MUST return `false` until the v1.0 gate below is fully met.

---

## 1. Purpose

All GAIAN image, video, and voice exports are C2PA-signed before leaving the
local boundary. This spec defines the `c2pa-rs`-backed export contract, the
manifest schema, tamper detection, the delete-my-GAIAN QA requirement, and the
GAIAN 2.0 v1.0 release gate.

---

## 2. `c2pa-rs` Export Contract

`C2paExport::sign(payload: &[u8], meta: ExportMeta) -> Result<SignedExport, C2paError>`

### `ExportMeta` required fields

```
gaian_id:    String   — non-empty GAIAN user id
timestamp:   String   — ISO 8601 UTC timestamp
consent_id:  String   — non-empty consent token from GAIAN vault (#65)
media_type:  String   — "image" | "audio" | "video" | "text"
```

| Rule | Constraint |
|---|---|
| `gaian_id` required | MUST be non-empty — `Err(MissingGaianId)` otherwise |
| `consent_id` required | MUST be non-empty — `Err(MissingConsentId)` otherwise |
| `timestamp` required | MUST be a valid ISO 8601 UTC string — `Err(InvalidTimestamp)` otherwise |
| No biometric field | Manifest MUST NOT contain a biometric, voiceprint, or face-hash field |
| No central warehouse | Signed export MUST NOT be uploaded to a central GAIAN server — `Err(WarehouseBlocked)` |

---

## 3. C2PA Manifest Schema

```json
{
  "claim_generator": "gaia-gaian/<version>",
  "assertions": [
    { "label": "ai.generated", "data": { "generated": true } },
    { "label": "c2pa.action", "data": { "action": "created", "softwareAgent": "gaia-gaian" } },
    { "label": "gaian.consent", "data": { "consent_id": "<consent_id>", "gaian_id": "<gaian_id>" } }
  ],
  "timestamp": "<ISO 8601 UTC>"
}
```

**MUST rules**
- `claim_generator` MUST be `"gaia-gaian/<version>"` with a non-empty version string.
- `assertions` MUST include `ai.generated`, `c2pa.action`, and `gaian.consent`.
- `gaian.consent.consent_id` MUST match the `consent_id` in `ExportMeta`.
- Manifest MUST NOT include any field named `biometric`, `voiceprint`, `face_hash`, or `face_id`.

---

## 4. Tamper Detection

`c2pa_verify(bytes: &[u8]) -> Result<C2paVerified, C2paError>`

| Return | Meaning |
|---|---|
| `Ok(C2paVerified)` | Manifest is present and signature is intact |
| `Err(TamperDetected)` | File has been modified since signing |
| `Err(NoManifest)` | File has no C2PA manifest |
| `Err(InvalidSignature)` | Signature does not verify against the manifest |

`c2pa_verify` MUST be called on all GAIAN exports before delivery. A file that
returns `Err(TamperDetected)` or `Err(NoManifest)` MUST NOT be delivered.

---

## 5. Delete-My-GAIAN QA Requirement

`delete_my_gaian(gaian_id: &str) -> Result<DeleteReceipt, GaianError>`

MUST appear in the release QA script. The QA step MUST:

1. Call `delete_my_gaian(id)` for a fixture GAIAN id.
2. Assert `Ok(DeleteReceipt)` is returned.
3. Assert no residual record exists for that id in any fixture store.
4. Assert the deleted id's C2PA exports are revoked or flagged `Err(RevokedConsent)` on verify.

This step MUST NOT be skippable in the GAIAN 2.0 v1.0 release QA script.

---

## 6. GAIAN 2.0 v1.0 Release Gate

`gaian_v1_tagged()` MUST return `false` until every condition below is satisfied
and a TSC resolution is recorded in `rfcs/gaian-v1.0-resolution.md`.

| Gate item | Condition |
|---|---|
| C2PA signing | `c2pa_export_has_manifest()` = `true` for all generated media export paths |
| Tamper detection | `c2pa_verify()` returns `Err(TamperDetected)` on a modified fixture file |
| No biometric warehouse | `biometric_warehouse_exists()` = `false` |
| Delete-my-GAIAN | `delete_my_gaian()` in release QA; receipt returned for fixture id |
| Release notes | Notes include: layers, known gaps, equity eval, child-safety, non-impersonation tests |
| TSC vote | Lazy-consensus resolution filed in `rfcs/gaian-v1.0-resolution.md` |

No code path may set `gaian_v1_tagged()` to `true` without the TSC resolution file present.

---

## 7. Prohibition Surface

| Prohibition | Error / Return |
|---|---|
| Export without C2PA manifest | `Err(MissingC2paManifest)` |
| Missing `gaian_id` | `Err(MissingGaianId)` |
| Missing `consent_id` | `Err(MissingConsentId)` |
| Biometric field in manifest | `Err(BiometricFieldForbidden)` |
| Upload to central warehouse | `Err(WarehouseBlocked)` |
| Modified file delivered | `Err(TamperDetected)` |
| v1.0 tag before gate | `gaian_v1_tagged()` → `false` |

---

## 8. What This Spec Does Not Do

- Does not operate a live C2PA signing service or key management infrastructure.
- Does not create a central biometric, voiceprint, or face-hash warehouse.
- Does not sign exports without explicit user consent (`consent_id` required).
- Does not allow tampered files to be delivered.
- Does not tag GAIAN 2.0 v1.0.

---

## 9. Acceptance Gate

- [ ] `C2paExport::sign` with empty `gaian_id` → `Err(MissingGaianId)`
- [ ] `C2paExport::sign` with empty `consent_id` → `Err(MissingConsentId)`
- [ ] Signed manifest includes `ai.generated`, `c2pa.action`, `gaian.consent`
- [ ] Manifest MUST NOT contain `biometric` or `face_hash` field
- [ ] `c2pa_verify(original_bytes)` → `Ok(C2paVerified)`
- [ ] `c2pa_verify(modified_bytes)` → `Err(TamperDetected)`
- [ ] `delete_my_gaian(fixture_id)` → `Ok(DeleteReceipt)` with no residual record
- [ ] `biometric_warehouse_exists()` → `false`
- [ ] `gaian_v1_tagged()` → `false`
- [ ] `cargo test -p gaia-gaian` green

---

## 10. Cross-References

- GAIAN identity: `gaia-spec/gaian/IDENTITY.md`
- GAIAN autonomy: `gaia-spec/gaian/AUTONOMY.md`
- Vault / export consent: `gaia-gaian` #65
- Wonder labels (creative output watermark): `gaia-spec/aimd/WONDER-LABELS.md` (#174)
- C2PA spec: C2PA Specification 2.1, Coalition for Content Provenance and Authenticity
- c2pa-rs crate: https://github.com/contentauth/c2pa-rs
- RFC placeholder: `rfcs/gaian-v1.0-resolution.md` (must not exist until gate is met)
- Issues: #63 (epic), #66 (C2PA), #76 (this slice)
