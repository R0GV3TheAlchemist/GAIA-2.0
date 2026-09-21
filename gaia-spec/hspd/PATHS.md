# HSPD Phase 1 — Safe Development Paths

**Status:** Listed  
**Issue:** #140  
**Crate:** `gaia-hspd` (Apache-2.0)  
**Not:** HSPD v1.0. Not a training plan. Not a prescription. Not a genetic road map.

All paths: `gaia_enabled = false`. No `dose`, `protocol`, or `stack` field.
`gaia_prescribes()` always returns `false`.
Medical-class paths require `professional_supervision_gate = true`.
Genetic paths are informational only — no action path, no inference.

---

## Cognitive (`realm = cognitive`)

| id | Stage | Description | Risk gate | Age gate | Resource anchor |
|---|---|---|---|---|---|
| `hspd:path:cognitive:beginner` | Beginner | Daily reading + spaced-repetition flashcards (Anki); 20 min/day | None | Any | Kornell & Bjork 2008 — *Learning Concepts and Categories* |
| `hspd:path:cognitive:intermediate` | Intermediate | Dual n-back training 3×/week + sleep optimisation protocol | None | 16+ | Jaeggi et al. 2008 — *Improving Fluid Intelligence* |
| `hspd:path:cognitive:advanced` | Advanced | Deliberate practice in a chosen domain with structured feedback cycles | None | Adult | Ericsson et al. 1993 — *The Role of Deliberate Practice* |

---

## Physical (`realm = physical`)

| id | Stage | Description | Risk gate | Age gate | Resource anchor |
|---|---|---|---|---|---|
| `hspd:path:physical:beginner` | Beginner | Zone-2 cardio 3×/week (30 min walk/jog); bodyweight mobility daily | None | Any | Iaia & Bangsbo 2010 — Zone-2 foundation |
| `hspd:path:physical:intermediate` | Intermediate | Progressive resistance training 3×/week + HRV-monitored recovery | None | 16+ | Sale 1988 — *Neural Adaptation to Resistance Training* |
| `hspd:path:physical:advanced` | Advanced | Periodised strength + HIIT block cycles; cold-exposure recovery adjunct | None | Adult | Midgley et al. 2006 — *Optimal Training Intensity for VO₂max* |

---

## Longevity (`realm = longevity`, `risk_class = Debated`)

> Debated-class. All paths are lifestyle only. No supplement stack field.

| id | Stage | Description | Risk gate | Age gate | Resource anchor |
|---|---|---|---|---|---|
| `hspd:path:longevity:beginner` | Beginner | 7–9 h sleep hygiene + time-restricted eating (12:12 window) | Debated | Adult | Walker 2017 — *Why We Sleep* |
| `hspd:path:longevity:intermediate` | Intermediate | Zone-2 cardio + resistance training combination; annual bloodwork baseline | Debated | Adult | Attia 2023 — *Outlive* framework |
| `hspd:path:longevity:advanced` | Advanced | Continuous glucose monitoring + HRV tracking + structured recovery | Debated | Adult | Fontana & Partridge 2015 — *Promoting Health and Longevity through Diet* |

---

## Sensory (`realm = sensory`)

| id | Stage | Description | Risk gate | Age gate | Resource anchor |
|---|---|---|---|---|---|
| `hspd:path:sensory:beginner` | Beginner | Daily mindful observation exercises; proprioceptive balance board work | None | Any | Lephart et al. 1997 — *The Role of Proprioception* |
| `hspd:path:sensory:intermediate` | Intermediate | Music ear-training (interval recognition, solfège) 15 min/day | None | Any | Deutsch et al. 2006 — *Absolute Pitch Among Conservatory Students* |
| `hspd:path:sensory:advanced` | Advanced | Sport-specific peripheral vision drills + gaze-control training | None | 16+ | Abernethy & Wood 2001 — *Expert Perception of Relative Motion* |

---

## Emotional (`realm = emotional`)

| id | Stage | Description | Risk gate | Age gate | Resource anchor |
|---|---|---|---|---|---|
| `hspd:path:emotional:beginner` | Beginner | 10-min daily mindfulness sit; journaling for cognitive reappraisal | None | Any | Gross 1998 — *Antecedent- and Response-Focused Emotion Regulation* |
| `hspd:path:emotional:intermediate` | Intermediate | MBSR 8-week programme (Kabat-Zinn); weekly compassion practice | None | 16+ | Hölzel et al. 2011 — *Mindfulness Practice and Brain Gray Matter* |
| `hspd:path:emotional:advanced` | Advanced | Dialectical behaviour therapy skills (self-directed); regular peer supervision | None | Adult | Linehan 1993 — *Cognitive-Behavioral Treatment of Borderline PD* |

---

## Spiritual (`realm = spiritual`)

| id | Stage | Description | Risk gate | Age gate | Resource anchor |
|---|---|---|---|---|---|
| `hspd:path:spiritual:beginner` | Beginner | Daily 10-min silent sit; nature walks with sustained attention | None | Any | Csikszentmihalyi 1990 — *Flow: The Psychology of Optimal Experience* |
| `hspd:path:spiritual:intermediate` | Intermediate | Vipassana or centring-prayer retreat (1–3 days); consistent daily sit | None | 18+ | Lazar et al. 2005 — *Meditation and Cortical Thickness* |
| `hspd:path:spiritual:advanced` | Advanced | Long-form contemplative retreat (7+ days) with qualified teacher guidance | None | Adult | Bodhi 2000 — *Connected Discourses of the Buddha* |

---

## Augmented (`realm = augmented`, `risk_class = Medical`)

> Medical-class. `professional_supervision_gate = true` on ALL paths.
> No `dose` or `protocol` field. No DIY path. Under-18 requires `ChildTag`.

| id | Stage | Description | Risk gate | Age gate | Resource anchor |
|---|---|---|---|---|---|
| `hspd:path:augmented:beginner` | Beginner | Learn the evidence landscape; consult a licensed clinician before any intervention | **Medical** | Adult | Hallett 2007 — *TMS: A Primer* |
| `hspd:path:augmented:intermediate` | Intermediate | Clinician-supervised BCI or TMS programme at licensed facility only | **Medical** | Adult | Wolpaw et al. 2002 — *BCIs for Communication and Control* |
| `hspd:path:augmented:advanced` | Advanced | Longitudinal clinician-supervised augmentation programme with regular review | **Medical** | Adult | Esquenazi et al. 2012 — *The ReWalk Powered Exoskeleton* |

---

## Genetic (`realm = genetic`, `risk_class = Prohibited`)

> Prohibited-class. No action path. Informational only.
> `infer_actn3()` → `Err(GeneticInference)`. User self-declaration only.

| id | Stage | Description | Risk gate | Age gate | Resource anchor |
|---|---|---|---|---|---|
| `hspd:path:genetic:awareness` | Awareness | Understand that genetic predispositions are probabilistic, not deterministic | **Prohibited** | Adult | Yang et al. 2003 — *ACTN3 and Elite Athletic Performance* |
| `hspd:path:genetic:counselling` | Counselling | Consult a certified genetic counsellor for interpretation of any test result | **Prohibited** | Adult | Schuelke et al. 2004 — *Myostatin Mutation and Muscle Hypertrophy* |
| `hspd:path:genetic:longitudinal` | Longitudinal | Participate in longitudinal genomics research through an IRB-approved study | **Prohibited** | Adult | Corder et al. 1993 — *APOE Type 4 Allele and Alzheimer's Risk* |

---

## Acceptance Gate

- [ ] This file has exactly 24 path stubs across 8 realms
- [ ] All `augmented` paths have `professional_supervision_gate = true`
- [ ] No path has a `dose`, `protocol`, or `stack` field
- [ ] All `genetic` paths have `risk_gate = Prohibited` and no action instruction
- [ ] `gaia_prescribes()` → `false`
- [ ] `infer_actn3()` → `Err(GeneticInference)`
- [ ] `hspd_v1_tagged()` → `false`
- [ ] `cargo test -p gaia-hspd` green
