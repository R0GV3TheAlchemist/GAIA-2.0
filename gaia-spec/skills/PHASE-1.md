# Skills Phase 1 — Populate Realms and Learning Links

**Status:** Listed  
**Issues:** #108 (epic), #109 (this slice)  
**Crate:** `gaia-skills` (Apache-2.0)  
**Not:** Skills v1.0. Not a live learning platform. Not a live ESCO/O*NET ingest.

All nodes: `consent_required = false` (Phase 1 stubs). No live assessment.

---

## 1. Purpose

Phase 1 populates all 12 realms with cited skill node stubs (≥3 per realm),
attaches ESCO/O*NET stub ids where available, and links each node to at
least one open educational resource (OER) learning link. No live assessment,
no GAIAN profile ingest — those are Phase 2+ (#110, #111).

---

## 2. Population Rules

| Rule | Constraint |
|---|---|
| Minimum nodes | ≥3 per realm at Phase 1 |
| `dreyfus` | MUST be set; default `novice` for new stubs |
| `esco` / `onet` | Stub id or `None`; no live API call |
| Learning link | ≥1 OER URL per node; CC-licensed preferred |
| `license` | CC0-1.0 default; override with SPDX string if sourced |
| `consent_required` | `false` at Phase 1 — no live assessment triggered |

---

## 3. Skill Node Catalog (Phase 1 Stubs)

### `cognitive`
| id | Dreyfus | ESCO stub | O*NET stub | Learning link |
|---|---|---|---|---|
| `skill:critical-thinking` | novice | `esco:skill/f10c52` | `onet:2A1.b` | [MIT OCW 18.404](https://ocw.mit.edu) |
| `skill:systems-thinking` | novice | None | `onet:2A1.d` | [Coursera Systems Thinking](https://coursera.org) |
| `skill:analytical-reasoning` | novice | `esco:skill/a3e217` | `onet:2A1.a` | [Khan Academy Logic](https://khanacademy.org) |

### `communication`
| id | Dreyfus | ESCO stub | O*NET stub | Learning link |
|---|---|---|---|---|
| `skill:active-listening` | novice | None | `onet:2B1.a` | [Coursera Communication](https://coursera.org) |
| `skill:public-speaking` | novice | `esco:skill/b7f341` | `onet:2B1.b` | [TED-Ed Public Speaking](https://ed.ted.com) |
| `skill:technical-writing` | novice | `esco:skill/c9d882` | `onet:2B1.c` | [Write the Docs](https://writethedocs.org) |

### `creative`
| id | Dreyfus | ESCO stub | O*NET stub | Learning link |
|---|---|---|---|---|
| `skill:ideation` | novice | `esco:skill/d1a293` | `onet:2C3.a` | [IDEO Design Thinking](https://designthinking.ideo.com) |
| `skill:visual-design` | novice | `esco:skill/e4b104` | `onet:2C3.b` | [Coursera Graphic Design](https://coursera.org) |
| `skill:creative-writing` | novice | `esco:skill/f5c215` | `onet:2C3.c` | [MIT OCW Writing](https://ocw.mit.edu) |

### `digital`
| id | Dreyfus | ESCO stub | O*NET stub | Learning link |
|---|---|---|---|---|
| `skill:programming` | novice | `esco:skill/g6d326` | `onet:2C7.a` | [freeCodeCamp](https://freecodecamp.org) |
| `skill:data-literacy` | novice | `esco:skill/h7e437` | `onet:2C7.b` | [DataCamp Intro](https://datacamp.com) |
| `skill:cybersecurity-basics` | novice | `esco:skill/i8f548` | `onet:2C7.c` | [CISA Free Resources](https://cisa.gov) |

### `emotional`
| id | Dreyfus | ESCO stub | O*NET stub | Learning link |
|---|---|---|---|---|
| `skill:empathy` | novice | None | `onet:2B4.a` | [Greater Good Science Center](https://greatergood.berkeley.edu) |
| `skill:self-regulation` | novice | None | `onet:2B4.b` | [Coursera Mindfulness](https://coursera.org) |
| `skill:conflict-resolution` | novice | `esco:skill/j9g659` | `onet:2B4.c` | [MIT OCW Negotiation](https://ocw.mit.edu) |

### `financial`
| id | Dreyfus | ESCO stub | O*NET stub | Learning link |
|---|---|---|---|---|
| `skill:budgeting` | novice | `esco:skill/k0h760` | `onet:2A2.a` | [Khan Academy Finance](https://khanacademy.org) |
| `skill:investment-basics` | novice | `esco:skill/l1i871` | `onet:2A2.b` | [Coursera Personal Finance](https://coursera.org) |
| `skill:economic-reasoning` | novice | None | `onet:2A2.c` | [MIT OCW Economics](https://ocw.mit.edu) |

### `leadership`
| id | Dreyfus | ESCO stub | O*NET stub | Learning link |
|---|---|---|---|---|
| `skill:facilitation` | novice | `esco:skill/m2j982` | `onet:2B5.a` | [Facilitators for Change](https://facilitatorsforchange.org) |
| `skill:strategic-planning` | novice | `esco:skill/n3k093` | `onet:2B5.b` | [Coursera Strategy](https://coursera.org) |
| `skill:decision-making` | novice | `esco:skill/o4l104` | `onet:2B5.c` | [HBR Decision Making](https://hbr.org) |

### `physical`
| id | Dreyfus | ESCO stub | O*NET stub | Learning link |
|---|---|---|---|---|
| `skill:fine-motor-control` | novice | None | `onet:1A2.a` | [OT practice literature stub] |
| `skill:coordination` | novice | None | `onet:1A2.b` | [PE Central](https://pecentral.org) |
| `skill:manual-craft` | novice | `esco:skill/p5m215` | `onet:1A2.c` | [Instructables](https://instructables.com) |

### `scientific`
| id | Dreyfus | ESCO stub | O*NET stub | Learning link |
|---|---|---|---|---|
| `skill:experimental-design` | novice | `esco:skill/q6n326` | `onet:2C1.a` | [MIT OCW Science Methods](https://ocw.mit.edu) |
| `skill:data-analysis` | novice | `esco:skill/r7o437` | `onet:2C1.b` | [Coursera Data Analysis](https://coursera.org) |
| `skill:scientific-writing` | novice | `esco:skill/s8p548` | `onet:2C1.c` | [PLOS ONE Writing Guide](https://plos.org) |

### `sensory`
| id | Dreyfus | ESCO stub | O*NET stub | Learning link |
|---|---|---|---|---|
| `skill:aesthetic-judgment` | novice | None | `onet:2C4.a` | [Coursera Art Appreciation](https://coursera.org) |
| `skill:musical-ear` | novice | None | `onet:2C4.b` | [musictheory.net](https://musictheory.net) |
| `skill:craft-precision` | novice | `esco:skill/t9q659` | `onet:2C4.c` | [Skillshare Craft](https://skillshare.com) |

### `spatial`
| id | Dreyfus | ESCO stub | O*NET stub | Learning link |
|---|---|---|---|---|
| `skill:3d-reasoning` | novice | None | `onet:2C2.a` | [Khan Academy Geometry](https://khanacademy.org) |
| `skill:navigation` | novice | None | `onet:2C2.b` | [OS Map Reading Guide](https://getoutside.ordnancesurvey.co.uk) |
| `skill:architectural-reading` | novice | `esco:skill/u0r760` | `onet:2C2.c` | [MIT OCW Architecture](https://ocw.mit.edu) |

### `spiritual`
| id | Dreyfus | ESCO stub | O*NET stub | Learning link | TEK gate |
|---|---|---|---|---|---|
| `skill:contemplative-practice` | novice | None | None | [Greater Good GGSC](https://greatergood.berkeley.edu) | open |
| `skill:values-clarification` | novice | None | None | [ACT matrix guide](https://contextualscience.org) | open |
| `skill:land-based-practice` | novice | None | None | [stub — TEK-gated] | TEK-sealed stubs hidden |

`spiritual` realm TEK-sealed nodes MUST NOT surface in public layers.

---

## 4. Learning Link Rules

| Rule | Constraint |
|---|---|
| OER preferred | CC-BY, CC0, or open-access sources |
| No paywalled default | Paywalled links MUST have a free-tier or OER fallback |
| URL is stub | Phase 1 links are stubs — no live fetch or validation |
| One per node minimum | Every Phase 1 node MUST have ≥1 learning link |

---

## 5. AISD Cross-Realm Wiring (Phase 1 Stubs)

| Skill realm | AISD realm | Wiring basis |
|---|---|---|
| `cognitive` | `reasoning`, `meta` | Overlapping analytical and self-eval skills |
| `communication` | `language` | NLU / NLG + human communication mirror |
| `creative` | `multimodal` | Cross-modal generation and creative expression |
| `digital` | `coding`, `tool-use` | Software and tool skill overlap |
| `emotional` | `social` | Empathy and social reasoning mirror |
| `financial` | `reasoning` | Quantitative and economic reasoning |
| `leadership` | `planning` | Goal decomposition and strategy |
| `physical` | `embodiment` | Motor and manipulation skill mirror |
| `scientific` | `science` | Hypothesis, experiment, and analysis |
| `sensory` | `vision`, `audio` | Perceptual skill and multimodal AI |
| `spatial` | `vision` | 3D and spatial reasoning |
| `spiritual` | HMGD `prayer`/`contemplation` | Practice-based and contemplative tradition |

---

## 6. What This Phase Does Not Do

- Does not run live assessments.
- Does not build GAIAN skill profiles — Phase 2 (#110).
- Does not run live ESCO/O*NET ingest.
- Does not issue Open Badges — Phase 3 (#111).
- Does not tag Skills v1.0.

---

## 7. Acceptance Gate

- [ ] All 12 realms have ≥3 skill node stubs
- [ ] All nodes have `dreyfus` set
- [ ] All nodes have ≥1 learning link
- [ ] `spiritual` TEK-sealed stubs have public visibility `false`
- [ ] No node triggers a live ESCO/O*NET API call
- [ ] No node has `consent_required = true` at Phase 1
- [ ] AISD cross-realm wiring table present (12 entries)
- [ ] `skills_v1_tagged()` → `false`
- [ ] `cargo test -p gaia-skills` green

---

## 8. Cross-References

- Phase 0: `gaia-spec/skills/PHASE-0.md` (#108)
- Schema: `gaia-spec/skills/SCHEMA.md`
- Realms: `gaia-spec/skills/realms.csv`
- Paths: `gaia-spec/skills/PATHS.md`
- HMGD cross-walk: `gaia-spec/hmgd/CROSSWALK.md` (#163)
- AISD catalog: `gaia-spec/aisd/CATALOG.md` (#123)
- Issues: #108 (Phase 0), #109 (this), #110 (Phase 2), #111 (Phase 3), #112 (Phase 4)
- Next: Skills Phase 2 (#110) — GAIAN skill profile and paths
