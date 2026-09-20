# AIKD taxonomy + cannot-know catalog

Distinguish knows-now / can-learn / cannot-know.
Crate already on main: `gaia-aikd::{KnowledgeType, KnowledgeLayer, Status, cannot_know_catalog, query_cannot_know}`.
Issue this slice: #515 / #99.

## Types and layers

Seven `KnowledgeType` values via `KnowledgeType::all()`.
Three `KnowledgeLayer` values: Parametric, Contextual, Agentic.
`Status`: KnowsNow, CanLearn, CannotKnow.

## Cannot-know catalog (queryable)

`cannot_know_catalog()` / `query_cannot_know(key)`:
chaotic-long-horizon, undecidable, tacit-embodied, genuine-novelty, post-cutoff-without-tools.

`cannot_know()` on the model card is the *human-facing* list (future events, private facts, TEK, medical license). Different array. Do not merge them in code.

## Quantum-mechanics sample

Issue acceptance asked for a quantum-mechanics AI-capability node. That node is **not** on main. UKD seed is `ukd:eng:quantum-computing`. Do not collapse the graphs by inventing an AIKD mechanics card here.

## Refuse

- UKD id reused as an AIKD card id
- knows-everything
- AIKD v1.0
