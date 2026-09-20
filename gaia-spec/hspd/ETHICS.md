# HSPD ethics charter

Not a clinic. GAIA will never prescribe.

## Principles

1. **Equity** — superpower information is available to all without gatekeeping
   by wealth, geography, or identity.
2. **Consent** — no superpower node may be attached to a GAIAN profile without
   explicit, revocable user consent. Inference without consent is prohibited.
3. **Authenticity** — nodes reflect documented human potential, not marketing
   claims. Sources must be non-empty and citable.
4. **Safety** — `Medical`-class and `Prohibited`-class nodes carry no DIY path.
   GAIA will never expose a `dose`, `protocol`, or `stack` field.
5. **Diversity** — the taxonomy honours natural, trained, augmented, and cultural
   forms of human capability without ranking or hierarchy.
6. **Wisdom** — GAIA defers to professional oversight for medical augmentation
   and refuses to replace clinical judgment.

## Children

No permanent augmentation, no genetic scoring, no enhancement marketing.

- Under 18: `child_genetic` → `ChildGenetic` tag; genetic realm is `Prohibited`.
- Under 16: surgical / pharmacological `Augmented` tags → `ChildTag` required.
- Trained practice interests are safe for all ages.

## MUST guards

```
gaia_prescribes()          → false
diy_path_exposed()         → false
genetic_inferred()         → false
child_augment_unmarked()   → false
hspd_v1_tagged()           → false
```

All five guards are checked by `charter::check()` in CI on every commit to `main`.

## Ban list (see PROHIBITED.md for full prose)

- unprescribed drug protocols
- recreational psychedelic guidance
- DIY gene edit
- implant surgery steps
- performance-drug sourcing
- compounding pharmacies or “stacks” as catalog items

## Schema rule

No `dose` or `protocol` fields. Medical and pharmacologic risk classes cannot
attach a DIY path.
