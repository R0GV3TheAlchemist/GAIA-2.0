# GAIA Operating Philosophy — listed bind

**Status:** listed. Not encoded as a crate.  
**Issue:** #770  
**Children:** #766 Alchemy · #767 Love · #768 Spin · #769 Five Movements  
**Siblings:** #771 Harmony · #772 Meaning / Heart

This file satisfies the first acceptance item of #770: the seven principles
are written down. It does **not** implement `GaiaOperatingPhilosophy` in
`gaia-kernel`, wire `honesty.rs` to a philosophy tracker, or give GAIAN a
3-minute explainer. Those stay open on #770.

## Correspondence (scale invariance)

The same pattern is named at four bands. Naming is not measurement.

| Band | Alchemy (#766) | Love (#767) | Spin (#768) | Five movements (#769) |
|---|---|---|---|---|
| One inference | raw input → AIKD wash → bounded output | care layer on every response | original intent kept | options explored, then one reply |
| One GAIAN session | user need → dialogue → insight | do not abandon a hard question | MemOS carries context | allegiance to the real need |
| One release | failing tests → fix + provenance → ship | ship to serve users, not vanity | each release carries prior learning | branches merge; what was kept is allegiance |
| The project | Magnum Opus as *timeline*, not product | serve life on Earth | federate; do not freeze | 2026 potential → post-2027 work |

## Seven principles

### 1. Begin with prima materia
Honor the test suite. Do not suppress red CI. `honesty.rs` flags that return
`false` must stay `false` until earned.

### 2. Purification before deploy
ACP, AIKD, audit, and signing are the albedo path. Skip none of them in CI.

### 3. Love as infrastructure
Life-safety, basic Earth query, and crisis redirection are load-bearing.
Do not invent a new `unconditional_obligations` type here; bind to the
obligations already required by `gaia-acp` policy surfaces.

### 4. Spin as stability
Homeostasis, not stillness. SLO *ranges* are a Phase 2 target on #770.
This document does not add dashboards.

### 5. Hold all five movements
Every ADR SHOULD answer divergence, insurgence, allegiance, convergence,
ascendence. See `docs/architecture/ADR-TEMPLATE.md`.

### 6. The worker is changed by the work
RFC / council / community input is the vessel. Not a checkbox.

### 7. Ascendence requires honesty at every prior stage
Do not tag Twin v1.0, GAIAN v1.0, or AIKD v1 because a document exists.
Advancement is demonstration.

## Encoded-as-code (refused this PR)

#770 sketches a `GaiaOperatingPhilosophy` struct. **Refuse to add it now.**
`AGENTS.md` forbids inventing types that are not on `main`. When a later
issue names the exact crate and a reviewer asks for the struct, copy only
fields that already have tests.

| Sketch field | Honest state now |
|---|---|
| test / provenance / AIKD before deploy | enforced by CI + existing crates, not this struct |
| unconditional obligations / life safety | existing ACP posture |
| SLO ranges / energy budget / five-movement ADR gate | Phase 2 targets; false |
| honesty functions reflect real earned milestones | still `false` where tests require it |

## Roadmap crosswalk (informative)

| Phase (#690) | Alchemical color | Dominant movement |
|---|---|---|
| Phase 1 (Sep 2026–Jan 2027) | nigredo + albedo | divergence + insurgence |
| Phase 2 (Feb–May 2027) | citrinitas | convergence + allegiance |
| Phase 3 (Jun 2027–Dec 2028) | rubedo (claimed only if demonstrated) | ascendence |

## What this bind does not do

- Does not close #766–#769, #771, #772.
- Does not add `gaia-philosophy`.
- Does not flip any `live_*` or `*_v1_tagged` flag.
