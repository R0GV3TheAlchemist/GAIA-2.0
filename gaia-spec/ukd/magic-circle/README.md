# GAIA Magic Circle of Knowledge v0.1

**Status:** Draft specification  
**Issue:** #312  
**Scope:** Local, paper-first knowledge-governance contract  
**Related:** #311 Golden Compass; `gaia-spec/ukd/`; `gaia-spec/governance/golden-compass/`

## Purpose

The Magic Circle of Knowledge is GAIA's learning and knowledge-governance companion to the Golden Compass and AI Advisory Integrity Ring. It defines how GAIA represents movement among observations, data, information, evidence, knowledge, understanding, wisdom, and stewardship—while retaining uncertainty, provenance, plural knowledge traditions, consent, correction, and the Unfinished Horizon.

It is not a complete database of human knowledge, not an autonomous epistemic authority, and not a claim that data becomes truth, knowledge, wisdom, or authority by pipeline.

## First Law — Symbolic Accountability

Every operational symbol MUST map to a defined process, inputs, outputs, accountable steward, constraints, evidence or metric basis, and feedback/correction path. A symbol without this mapping may be artistic, educational, historical, cultural, or user-declared personal expression, but MUST NOT be presented as an operational or empirical claim.

## Core model

```text
Center: Epistemic Humility / Responsible Attention
  I do not know everything; therefore I must keep learning responsibly.

Learning paths:
  Discovery · Memory · Understanding · Application

Transformation chain:
  Data → Information → Evidence → Knowledge → Understanding → Wisdom → Stewardship

Diamond of Knowledge:
  Truth · Memory · Discovery · Understanding
  Wisdom integrates under context and uncertainty.

Library Ring:
  History · Science · Art · Nature · Ethics · Philosophy
  · Engineering · Human Experience · Place · Language/oral tradition

Round Table of Knowledge:
  Peer review · dialogue · debate · attribution
  · challenge · correction · withdrawal
```

The transformation chain is a set of distinguishable epistemic statuses. It MUST NOT be treated as a rigid one-way ladder. Evidence is required as its own status. Later statuses MAY return to earlier ones through correction, dissent, or new observation.

## Relationship to UKD

A Magic Circle record MAY reference an existing UKD knowledge node (`gaia-spec/ukd/knowledge-node.schema.json`) by identifier. It MUST NOT replace node identity, source/provenance fields, TEK seals, or consent/capability controls. Retrieval for GAIAN, Earth Twin, or Super OS remains advisory in this issue. This specification adds no runtime agent authority, remote service, credentials, or network access.

## Normative terms

- **MUST:** required for a conforming knowledge record.
- **SHOULD:** expected unless a documented rationale states otherwise.
- **MAY:** optional.

## Non-negotiable boundaries

- No claim of a complete or final database of human knowledge.
- No claim that data automatically becomes truth, knowledge, wisdom, or authority.
- No automatic ingestion of restricted, sacred, Indigenous, or community-held knowledge.
- No personality inference, astrology-based prediction, supernatural-causation claim, or operational authority derived from symbols.
- No runtime agent authority or change to existing consent/capability controls.
- Restricted, sacred, Indigenous, or community-held knowledge requires applicable authority, collective consent, attribution, benefit-sharing, and withdrawal support.

## Related inquiry

Information-science uses of a data–information–knowledge–wisdom stack are historically common and also widely treated as theoretically unsatisfactory when taken as a rigid pyramid. This specification therefore keeps Evidence explicit, separates sources from interpretation, and forbids automatic promotion of status. *Viriditas* is documented only as a tradition-contextual term (see `viriditas-and-unfinished-horizon.md`); it has no universal scientific status here.

## Documents

- `epistemic-categories.md` — statuses, chain, Library Ring, learning paths
- `diamond-of-knowledge.md` — Truth, Memory, Discovery, Understanding, Wisdom
- `round-table-of-knowledge.md` — attribution, challenge, correction, retraction, withdrawal
- `universal-symbology-bridge.md` — category boundaries and First Law mapping
- `viriditas-and-unfinished-horizon.md` — generative-knowledge test and revision duty
- `publication-status.md` — public/non-public statuses compatible with #311
- `knowledge-record.schema.json` — machine-readable advisory record contract

JSON examples and validation fixtures are deferred until this specification is reviewed.
