# Skills Phase 0 — Taxonomy, Schema, ESCO/O*NET, Graph

**Status:** Listed  
**Issues:** #108 (this epic)  
**Crate:** `gaia-skills` (Apache-2.0)  
**Not:** Skills v1.0. Not a live ESCO/O*NET ingest. Not a live graph store. Not a UKD remap.

---

## 1. Purpose

Phase 0 publishes the 12-realm human skill taxonomy, the `SkillNode` schema,
the Dreyfus 5-level mastery model, ESCO/O*NET alignment rules (stub
references only — no live ingest), graph relation type definitions, and the
ethics charter. No catalog population, no learning links — those are Phase 1 (#109).

---

## 2. 12-Realm Taxonomy

Defined in crate as `gaia_skills::REALMS` with 12 names.
`realm_stubs()` returns one domain stub per realm.
Skill ids MUST NOT reuse UKD concept ids.

| Realm | Description | AISD cross-link |
|---|---|---|
| `cognitive` | Reasoning, analysis, critical thinking, problem-solving | `reasoning`, `meta` |
| `communication` | Speaking, writing, listening, negotiation | `language` |
| `creative` | Ideation, design, artistic expression, improvisation | `multimodal` |
| `digital` | Computing, data, cybersecurity, software use | `coding`, `tool-use` |
| `emotional` | Empathy, self-regulation, resilience, social awareness | `social` |
| `financial` | Budgeting, investment, economic reasoning | `reasoning` |
| `leadership` | Facilitation, strategy, team coordination, decision-making | `planning` |
| `physical` | Motor skills, coordination, fitness, manual dexterity | `embodiment` |
| `scientific` | Observation, hypothesis, experimental design, data analysis | `science` |
| `sensory` | Perceptual discrimination, aesthetic judgment, craft | `vision`, `audio` |
| `spatial` | Navigation, mapping, 3D reasoning, architecture | `vision` |
| `spiritual` | Contemplative practice, values clarification, meaning-making | HMGD cross-link |

`skills_v1_tagged()` MUST return `false` at Phase 0.

---

## 3. `SkillNode` Schema

Fields confirmed in crate (`gaia_skills::SkillNode`):

```
id:      String          — "skill:<slug>" — MUST NOT be a UKD concept id
realm:   Realm           — one of the 12 above
dreyfus: DreyfusLevel    — novice | advanced_beginner | competent | proficient | expert
esco:    Option<String>  — ESCO occupation/skill URI stub; None if unmapped
onet:    Option<String>  — O*NET element code stub; None if unmapped
license: String          — SPDX license id; default CC0-1.0
```

**MUST rules**
- `id` MUST NOT duplicate a UKD concept id.
- Unmapped skills MUST set `esco = None` and `onet = None`; they still exist.
- `license` MUST be a valid SPDX expression.
- `dreyfus` MUST be one of the 5 defined levels.

---

## 4. Dreyfus 5-Level Mastery Model

| Level | Label | Description |
|---|---|---|
| 1 | `novice` | Follows rules; no contextual judgment |
| 2 | `advanced_beginner` | Recognises patterns in familiar situations |
| 3 | `competent` | Plans and adapts; takes responsibility |
| 4 | `proficient` | Sees situations holistically; intuitive grasp |
| 5 | `expert` | Fluid, effortless performance; transcends rules |

Source: Dreyfus & Dreyfus 1980 — *A Five-Stage Model of the Mental Activities Involved in Directed Skill Acquisition*.

---

## 5. ESCO / O*NET Alignment Rules

| Rule | Constraint |
|---|---|
| ESCO URIs | Stub references only; format `esco:skill/<uuid>` |
| O*NET codes | Stub references only; format `onet:<element-code>` |
| No live ingest | Live ESCO/O*NET API ingest deferred (separate issue) |
| None is valid | Unmapped skills set `esco = None`, `onet = None` |
| No UKD reuse | Skill ids MUST NOT reuse UNESCO Knowledge Domain concept ids |

Source anchors:
- ESCO v1.2 — European Commission, 2023 — *European Skills, Competences, Qualifications and Occupations*
- O*NET 28.0 — U.S. DOL/ETA, 2024 — *Occupational Information Network*

---

## 6. Graph Relation Types

Relation type names are defined at Phase 0. No live graph store at Phase 0.

| Relation | Meaning |
|---|---|
| `prerequisite` | Skill A must be at `competent`+ before acquiring Skill B |
| `enables` | Acquiring Skill A accelerates Skill B |
| `related` | Thematic overlap; no ordering constraint |
| `requires_knowledge` | Skill A needs a knowledge node from AIKD |

All relation types are names only. Live graph traversal deferred to Phase 2 (#110).

---

## 7. Ethics Charter

| Rule | Constraint |
|---|---|
| No surveillance | Skill data MUST NOT be used for covert profiling |
| Consent-first | No skill assessment without explicit opt-in |
| TEK sovereignty | `spiritual` realm nodes with TEK links MUST respect TEK gate |
| No genetic inference | MUST NOT infer genetic traits from skill profiles |
| CC0 default | Stub nodes default to CC0-1.0 unless sourced otherwise |

---

## 8. What This Phase Does Not Do

- Does not populate realm catalogs.
- Does not create learning links.
- Does not run live ESCO/O*NET ingest.
- Does not build GAIAN skill profiles — Phase 2 (#110).
- Does not tag Skills v1.0.

---

## 9. Acceptance Gate

- [ ] `REALMS` contains exactly 12 realm names including `digital`
- [ ] `realm_stubs()` returns 12 entries
- [ ] `active_listening()` node has `realm = communication`, `dreyfus = novice`, `license = CC0-1.0`
- [ ] `SkillNode` has fields: `id, realm, dreyfus, esco, onet, license`
- [ ] No `SkillNode.id` is a UKD concept id
- [ ] Unmapped nodes have `esco = None`, `onet = None`
- [ ] `skills_v1_tagged()` → `false`
- [ ] No live ESCO/O*NET API call
- [ ] `cargo test -p gaia-skills` green

---

## 10. Cross-References

- Schema: `gaia-spec/skills/SCHEMA.md`
- Graph: `gaia-spec/skills/GRAPH.md`
- Realms: `gaia-spec/skills/realms.csv`, `realms.json`
- HMGD cross-walk: `gaia-spec/hmgd/CROSSWALK.md` (#163 — merged)
- AISD cross-link: `gaia-spec/aisd/CROSSWALK.md` (stub)
- Issues: #108 (this), #109 (Phase 1), #110 (Phase 2), #111 (Phase 3), #112 (Phase 4)
- Next: Skills Phase 1 (#109) — populate realms and learning links
