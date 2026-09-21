# GAIA 2.0 — Universal AI Skills Database

**Research Date:** September 2026  
**Classification:** Foundational AI Capabilities Architecture Document  
**Status:** Living Document — Version 0.1  
**Scope:** AI Capabilities Applied to Earth Problems; AI-Human Collaboration Patterns  
**Issue:** #688

---

## What Is an AI Skill?

An **AI Skill** in GAIA 2.0 is a reliable, deployable capability that AI systems can perform to address Earth systems challenges.

```
AI SKILL     = A capability AI has developed and can reliably perform
AI SUPERPOWER = A capability where AI dramatically exceeds human limits
                or exhibits emergent, non-human cognition
```

## AI Skill Node Schema

```json
{
  "id": "uuid-v4",
  "name": "Weather Forecasting",
  "category": "earth_science",
  "reliability": "high | medium | low | emerging",
  "human_parity": "exceeds | matches | below",
  "model_examples": ["GraphCast", "Pangu-Weather", "Aurora"],
  "gaia_applications": ["earth-twin", "climate-adaptation"],
  "description": "10-day global weather forecast in 60 seconds, outperforming supercomputer-based models",
  "evidence": "Lam et al., Science 2023; Stanford AI Index 2026",
  "human_complement": "Meteorologists interpret AI output, communicate to communities, make high-stakes decisions",
  "limitations": "Short-range physical world failures; analog environment reading"
}
```

## Skill Realm 1: Earth Science Skills

```
1.1 WEATHER & CLIMATE FORECASTING
    ├── 10-day global weather (GraphCast — 60 seconds vs. ECMWF hours)
    ├── Seasonal climate projection (ClimaX, Aurora)
    ├── Extreme weather detection (hurricane, flood, drought)
    └── Climate change scenario modelling

1.2 EARTH OBSERVATION & REMOTE SENSING
    ├── Satellite image analysis (TerraMind — 9 modalities)
    ├── Land cover change detection
    ├── Deforestation monitoring
    ├── Ice sheet and glacier tracking
    └── Ocean surface temperature mapping

1.3 BIODIVERSITY MONITORING
    ├── Species identification from image (BioCLIP, iNaturalist)
    ├── Species identification from audio (NatureLM-Audio)
    ├── Population trend analysis from GBIF data
    ├── Ecosystem health scoring
    └── Invasive species detection

1.4 GEOSPATIAL ANALYSIS
    ├── Land use and land cover classification
    ├── Terrain and hydrology modelling
    ├── Disaster risk mapping
    ├── Urban growth monitoring
    └── Agricultural productivity mapping (PRITHVI)
```

## Skill Realm 2: Scientific & Research Skills

```
2.1 LITERATURE SYNTHESIS
    ├── Systematic review across 250M+ papers (OpenAlex, Semantic Scholar)
    ├── Evidence grading and confidence assessment
    ├── Contradiction detection across sources
    └── Emerging research trend identification

2.2 HYPOTHESIS GENERATION
    ├── Generating testable hypotheses from large datasets
    ├── Cross-domain hypothesis transfer
    └── AI co-scientist collaboration (approaching Nobel Prize-worthy research)

2.3 DATA ANALYSIS
    ├── Time-series analysis (climate, biodiversity, ecosystem)
    ├── Multivariate pattern detection
    ├── Anomaly detection in Earth observation data
    └── Causal inference from observational data

2.4 MATERIALS & MOLECULAR SCIENCE
    ├── Protein structure prediction (AlphaFold 3.2 — 30 seconds)
    ├── Drug-protein binding prediction
    ├── Novel crystal structure discovery (GNoME — 2.2M structures)
    └── Carbon capture material design
```

## Skill Realm 3: Language & Communication Skills

```
3.1 MULTILINGUAL COMMUNICATION
    ├── Translation across 100+ language pairs
    ├── Real-time interpretation
    ├── Multilingual document generation
    └── Cross-cultural communication support

3.2 KNOWLEDGE SYNTHESIS & EXPLANATION
    ├── Summarising complex scientific content for any audience
    ├── Generating accessible explanations at any level
    ├── Creating educational content from technical material
    └── Citation-grounded response generation (RAG)

3.3 DOCUMENT GENERATION
    ├── Policy document drafting
    ├── Scientific report writing
    ├── Grant proposal support
    └── Community communication materials
```

## Skill Realm 4: Coding & Technical Skills

```
4.1 SOFTWARE DEVELOPMENT
    ├── Code generation in 120+ languages (HumanEval 96.7%)
    ├── Bug detection and repair (SWE-bench Verified 95%)
    ├── Code review and documentation
    └── Architecture design support

4.2 DATA PIPELINE DEVELOPMENT
    ├── ETL pipeline construction
    ├── API integration
    ├── Database schema design
    └── ML model deployment

4.3 INFRASTRUCTURE & DEVOPS
    ├── Infrastructure-as-code generation
    ├── CI/CD pipeline configuration
    └── Monitoring and alerting setup
```

## Skill Realm 5: Reasoning & Decision Support

```
5.1 COMPLEX REASONING
    ├── Multi-step logical reasoning (chain-of-thought)
    ├── Mathematical problem-solving (MATH-500: 97.3%)
    ├── Formal proof verification (Lean, Coq)
    └── Scenario planning and consequence mapping

5.2 DECISION SUPPORT
    ├── Options analysis with trade-off mapping
    ├── Risk assessment and uncertainty quantification
    ├── Evidence synthesis for policy decisions
    └── Ethical impact assessment

5.3 PLANNING & COORDINATION
    ├── Multi-step task planning
    ├── Resource allocation optimisation
    └── Scheduling and logistics
```

## AI-Human Skill Routing

GAIAN routes tasks based on relative capability:

| Task Type | Route To | Rationale |
|---|---|---|
| Large-scale data analysis | AI | Scale superpower |
| Community trust-building | Human | Relational superpower |
| Literature synthesis | AI | Speed + breadth |
| Physical restoration work | Human | Embodiment |
| Policy drafting (first draft) | AI | Speed |
| Moral judgment on policy | Human | Moral agency |
| Species ID from image/audio | AI | Pattern recognition |
| Place-based ecological reading | Human | Lifetime observation |
| Multilingual translation | AI-Human | AI for accuracy; human for nuance |

See `AI-HUMAN-COMPLEMENTARITY-MAP.md` for full routing logic.

## References

- Stanford AI Index 2026
- Papers with Code State of the Art (2026)
- Lam et al. — GraphCast (Science, 2023)
- DeepMind — AlphaFold 3.2 (2024)
- Google DeepMind — GNoME (Nature, 2023)
