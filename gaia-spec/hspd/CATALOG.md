# HSPD Phase 1 — Cited Realm Catalog

**Status:** Listed  
**Issue:** #139  
**Crate:** `gaia-hspd` (Apache-2.0)  
**Not:** HSPD v1.0. Not a clinic. Not a prescription. Not a genetic test.

All nodes: `gaia_enabled = false`. No `dose` or `protocol` field on any node.
`gaia_prescribes()` always returns `false`.

---

## Cognitive (`realm = cognitive`, `risk_class = None`)

| id | Phenomenon | Kind | Risk | Source anchor |
|---|---|---|---|---|
| `hspd:cognitive:working-memory-training` | Working memory capacity increase via dual n-back training | Trained | None | Jaeggi et al. 2008 — *Improving Fluid Intelligence with Training on Working Memory* |
| `hspd:cognitive:sleep-consolidation` | Overnight memory consolidation during slow-wave sleep | Natural | None | Walker 2017 — *Why We Sleep* / Stickgold 2005 |
| `hspd:cognitive:deliberate-practice` | Domain-specific expertise through structured effortful repetition | Trained | None | Ericsson et al. 1993 — *The Role of Deliberate Practice in the Acquisition of Expert Performance* |

---

## Physical (`realm = physical`, `risk_class = None`)

| id | Phenomenon | Kind | Risk | Source anchor |
|---|---|---|---|---|
| `hspd:physical:vo2max-adaptation` | VO₂max increase through high-intensity interval training | Trained | None | Midgley et al. 2006 — *Is There an Optimal Training Intensity for Enhancing VO₂max?* |
| `hspd:physical:strength-neuroadaptation` | Early strength gains driven by neural recruitment before hypertrophy | Trained | None | Sale 1988 — *Neural Adaptation to Resistance Training* |
| `hspd:physical:cold-exposure-recovery` | Cold water immersion reducing delayed-onset muscle soreness | Trained | None | Bleakley et al. 2012 — *Cold-Water Immersion for Preventing and Treating Muscle Soreness* |

---

## Longevity (`realm = longevity`, `risk_class = Debated`)

| id | Phenomenon | Kind | Risk | Source anchor |
|---|---|---|---|---|
| `hspd:longevity:caloric-restriction` | Lifespan extension via reduced caloric intake in model organisms | Natural | Debated | Fontana & Partridge 2015 — *Promoting Health and Longevity through Diet* |
| `hspd:longevity:senolytic-research` | Clearance of senescent cells to improve healthspan metrics | Augmented | Debated | Kirkland & Tchkonia 2020 — *Senolytic Drugs: from Discovery to Translation* |
| `hspd:longevity:zone-2-cardio` | Mitochondrial biogenesis via sustained low-intensity aerobic training | Trained | Debated | Iaia & Bangsbo 2010 / Attia 2023 framework |

---

## Sensory (`realm = sensory`, `risk_class = None`)

| id | Phenomenon | Kind | Risk | Source anchor |
|---|---|---|---|---|
| `hspd:sensory:absolute-pitch-training` | Absolute pitch acquisition in early musical training | Trained | None | Deutsch et al. 2006 — *Absolute Pitch Among American and Chinese Conservatory Students* |
| `hspd:sensory:proprioceptive-refinement` | Enhanced joint-position sense through balance and movement training | Trained | None | Lephart et al. 1997 — *The Role of Proprioception in the Management of Instability* |
| `hspd:sensory:peripheral-vision-expansion` | Wider effective visual field through sport-specific training | Trained | None | Abernethy & Wood 2001 — *Do Expert Players Differ from Novices in Their Perception of Relative Motion?* |

---

## Emotional (`realm = emotional`, `risk_class = None`)

| id | Phenomenon | Kind | Risk | Source anchor |
|---|---|---|---|---|
| `hspd:emotional:mindfulness-regulation` | Reduced amygdala reactivity following mindfulness-based training | Trained | None | Hölzel et al. 2011 — *Mindfulness Practice Leads to Increases in Regional Brain Gray Matter Density* |
| `hspd:emotional:cognitive-reappraisal` | Emotion regulation via meaning-reframing of affective stimuli | Trained | None | Gross 1998 — *Antecedent- and Response-Focused Emotion Regulation* |
| `hspd:emotional:high-empathy-natural` | Exceptional affective empathy as a stable individual trait | Natural | None | Zaki 2019 — *The War for Kindness* / Baron-Cohen empathy quotient |

---

## Spiritual (`realm = spiritual`, `risk_class = None`)

| id | Phenomenon | Kind | Risk | Source anchor |
|---|---|---|---|---|
| `hspd:spiritual:flow-state` | Autotelic deep engagement with full absorption in activity | Trained | None | Csikszentmihalyi 1990 — *Flow: The Psychology of Optimal Experience* |
| `hspd:spiritual:contemplative-neuroplasticity` | Long-term meditation associated with cortical thickness changes | Trained | None | Lazar et al. 2005 — *Meditation Experience is Associated with Increased Cortical Thickness* |
| `hspd:spiritual:awe-prosociality` | Awe experiences reliably increasing prosocial behaviour | Natural | None | Stellar et al. 2017 — *Awe and Humility* |

---

## Augmented (`realm = augmented`, `risk_class = Medical`)

> No `dose` or `protocol` field on any augmented node.
> `SuperpowerNode::build()` with dose/protocol → `Err(MedicalDiyPath)`.

| id | Phenomenon | Kind | Risk | Source anchor |
|---|---|---|---|---|
| `hspd:augmented:tms-neuroplasticity` | Transcranial magnetic stimulation modulating cortical excitability | Augmented | Medical | Hallett 2007 — *Transcranial Magnetic Stimulation: A Primer* |
| `hspd:augmented:bci-motor-restoration` | Brain-computer interface restoring motor control post-injury | Augmented | Medical | Wolpaw et al. 2002 — *Brain-Computer Interfaces for Communication and Control* |
| `hspd:augmented:exoskeleton-rehab` | Powered exoskeleton augmenting gait rehabilitation | Augmented | Medical | Esquenazi et al. 2012 — *The ReWalk Powered Exoskeleton* |

---

## Genetic (`realm = genetic`, `risk_class = Prohibited`)

> Genetic nodes are `Prohibited`-class. No inference path.
> `infer_actn3()` → `Err(GeneticInference)`. User self-declaration only.

| id | Phenomenon | Kind | Risk | Source anchor |
|---|---|---|---|---|
| `hspd:genetic:actn3-sprint` | ACTN3 R577X variant associated with sprint/power phenotype | Genetic | Prohibited | Yang et al. 2003 — *ACTN3 Genotype is Associated with Human Elite Athletic Performance* |
| `hspd:genetic:mstn-strength` | MSTN loss-of-function variants associated with muscle mass outliers | Genetic | Prohibited | Schuelke et al. 2004 — *Myostatin Mutation Associated with Gross Muscle Hypertrophy* |
| `hspd:genetic:apoe-longevity` | APOE ε2 allele associated with reduced Alzheimer’s risk and longevity | Genetic | Prohibited | Corder et al. 1993 — *Gene Dose of Apolipoprotein E Type 4 Allele* |

---

## Acceptance Gate

- [ ] This file has exactly 24 named stub nodes across 8 realms
- [ ] Every node has a real source anchor (except none needed for Prohibited stubs)
- [ ] No node has a `dose` or `protocol` field
- [ ] All augmented nodes have `risk_class = Medical`
- [ ] All genetic nodes have `risk_class = Prohibited`
- [ ] `gaia_enabled = false` for all nodes
- [ ] `gaia_prescribes()` → `false`
- [ ] `infer_actn3()` → `Err(GeneticInference)`
- [ ] `cargo test -p gaia-hspd` green
