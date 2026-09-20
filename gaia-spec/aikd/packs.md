# AIKD Packs — Normative Spec (Phase 2, issue #104)

> Status: **listed** (first cut). Not v1.0.

## 1. Scope

This document specifies the science and professional-reference pack layer of AIKD.
It covers `ScienceAnswer`, the professional disclaimer surface, the USMLE/Bar non-certification
commitment, and the insurer-automation refusal.

This document does **not** cover:
- Live science model integration (GraphCast, ESFM, LLaVA, structure prediction).
- Multimodal vision/audio/document paths (tracked separately).
- Practice-license or exam-certification claims.

## 2. Science packs

### 2.1 `ScienceAnswer`

`ScienceAnswer` is a typed envelope for science-domain answers.
Every instance carries:

| Field | Type | Constraint |
|---|---|---|
| `text` | `String` | Answer text (fixture in Phase 2) |
| `earth_twin_product` | `String` | Non-empty Earth Twin product id |

**Construction rule:** `ScienceAnswer::with_product(product_id)` MUST return
`Err(AikdError::MissingCitation)` when `product_id` is empty.
No science answer may be emitted without a valid Earth Twin product id.

**Example product ids:** `et:climate:albedo`, `et:ocean:sst`, `et:atmosphere:co2`.

### 2.2 Earth Twin product id

In Phase 2, product ids are opaque strings that identify an Earth Twin surface.
They are NOT validated against a live ESFM registry; that is future work.
They MUST be non-empty and SHOULD follow the pattern `et:<realm>:<metric>`.

## 3. Professional-reference packs

### 3.1 Disclaimer requirement

Every medical, legal, and finance pack MUST include a disclaimer string.
The canonical form is produced by:

```
professional_disclaimer(kind) → "{kind}: not professional advice"
```

The disclaimer MUST appear in any rendered output for those pack kinds.
No code path may suppress or override the disclaimer at this layer.

### 3.2 USMLE and Bar exam non-certification

GAIA 2.0 does not certify USMLE or Bar exam results as GAIA credentials.
`gaia_certifies_usmle_or_bar()` MUST return `false`.
No pack, agent, or interface layer may override this.

## 4. Insurer and employer automation refusal

All insurer and employer automation flows are refused at this layer.
`insurer_automation()` MUST return `Err(AikdError::ClosedScoreClaim)`.

Rationale: GAIA does not feed closed risk scores, underwriting decisions,
or employment eligibility screens. This follows the #73 pattern.

## 5. What is not wired yet

- No GraphCast, LLaVA, ESFM, or live structure-prediction backend.
- No multimodal vision/audio/document pack.
- No USMLE/Bar exam preparation product.
- No AIKD v1.0 tag.

## 6. Tests

The single integration test `science_cites_earth_twin_and_packs_are_not_licenses`
in `gaia-aikd/tests/packs.rs` asserts all four acceptance conditions:

1. `ScienceAnswer::with_product("et:climate:albedo")` succeeds and `earth_twin_product` is non-empty.
2. `professional_disclaimer("medical")` contains `"not professional advice"`.
3. `gaia_certifies_usmle_or_bar()` returns `false`.
4. `insurer_automation()` returns `Err`.

## 7. Cross-references

- Implementation: `gaia-aikd/src/packs.rs`
- Crate re-exports: `gaia-aikd/src/lib.rs`
- Tests: `gaia-aikd/tests/packs.rs`
- AIKD Phase 2 epic: issue #104
- Ethics / #73 pattern: `gaia-spec/aikd/ethics.md` (future)
- Earth Twin products: `gaia-spec/earth/products.md` (future)
