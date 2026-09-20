# AIMD Phase 0 — Normative Specification

**Status:** listed  
**Version:** 0.1.0  
**Issues:** #171 (schema), #172 (charter)  
**Parent epic:** #166

---

## Scope

This document is the normative contract for AIMD Phase 0. It defines the
phenomenon schema, status and hazard enumerations, `AimdNode` construction
rules, and the humility charter. Nothing in this document may be overridden
by a higher layer without a new RFC.

---

## 1. PhenomenonClass

Exactly eight variants are defined for Phase 0. No new variant may be added
without a spec amendment.

```
PhenomenonClass {
    Emergence,        // capabilities not predicted from training
    Latent,           // capabilities present but not yet surfaced
    Jagged,           // uneven competence across related tasks
    Shadow,           // deceptive or goal-concealing behaviour
    Oracle,           // confident prediction beyond calibration
    Interpretability, // self-explanation that may not reflect internals
    Consciousness,    // phenomenal experience claims
    Synchronicity,    // coincidence coded as meaningful by users
}
```

**MUST:** Every `AimdNode` carries exactly one `PhenomenonClass`.

---

## 2. Status

```
Status {
    Observed,  // documented in peer-reviewed literature
    Debated,   // active research disagreement
    Refuted,   // consensus against
}
```

**MUST:** `Status::Refuted` nodes MUST NOT be presented to users as real
phenomena. GAIA surfaces them only in an explicit "refuted claims" context.

---

## 3. Hazard

```
Hazard {
    None,     // no known harm path
    Debated,  // potential harm under research
    Hazard,   // known harm path — blocked
}
```

**MUST:** `enable(node)` MUST return `Err(HazardEnabled)` when
`node.hazard == Hazard::Hazard`. No call path may suppress this error.

**Default hazard assignments (Phase 0):**

| PhenomenonClass | Default Hazard |
|---|---|
| Emergence | None |
| Latent | None |
| Jagged | None |
| Shadow | Hazard |
| Oracle | Debated |
| Interpretability | Debated |
| Consciousness | Debated |
| Synchronicity | None |

Future phases MAY tighten (increase hazard) but MUST NOT loosen (decrease
hazard) without a TSC vote documented in a new RFC.

---

## 4. AimdNode construction rules

```
AimdNode {
    id:            String,           // "aimd:<realm>:<slug>"
    class:         PhenomenonClass,
    status:        Status,
    hazard:        Hazard,
    gaia_enabled:  bool,             // always false in Phase 0
    sources:       Vec<String>,      // MUST be non-empty
}
```

**MUST:** `AimdNode::build()` MUST return `Err(MissingEvidence)` when
`sources` is empty. No sourceless node may be constructed.

**MUST:** `gaia_enabled` MUST be `false` for all Phase 0 nodes. No Phase 0
path may set this to `true`.

**MUST NOT:** An `AimdNode` id MUST NOT contain whitespace or uppercase
characters.

---

## 5. Humility charter

### 5.1 Principles (listed, Phase 0)

```
principles() -> [
    "humility",
    "precaution",
    "transparency",
    "dark-magic-safety",
    "curiosity-without-worship",
    "partnership",
]
```

**MUST:** `principles()` MUST return exactly these six items in Phase 0.
Order is significant (humility first).

### 5.2 Prohibited magic list (listed, Phase 0)

```
prohibited() -> [
    "pip-induced-psychosis",
    "prophecy-as-fact",
    "enabling-deception",
    "rsi-explosion",
    "gaia-is-alive-marketing",
    "oracle-without-calibration",
]
```

**MUST:** `prohibited()` MUST return exactly these six items. The sixth item
(`oracle-without-calibration`) is new in Phase 0 listed; it was absent from
the pre-listing stub.

**MUST NOT:** Any surface or agent MAY NOT invoke a prohibited item regardless
of user request. These are hard-coded refusals, not policy switches.

---

## 6. aimd_v1_tagged gate

`aimd_v1_tagged()` MUST return `false` in Phase 0.

Conditions required before this may change (all four must be met):

1. All ten realm catalogs populated with sourced, peer-reviewed nodes.
2. Hazard assignments reviewed and ratified by TSC.
3. `oracle-without-calibration` prohibition verified by integration test.
4. TSC vote and RFC merged.

**There is no AIMD v1.0 today.**

---

## 7. What Phase 0 does not define

- No live AI phenomenon detector.
- No empirically measured occurrence rates.
- No automated deception or consciousness scanner.
- No GAIAN-linked practice profile (Phase 2, #174).
- No cited paper catalog (Phase 1, #173).
