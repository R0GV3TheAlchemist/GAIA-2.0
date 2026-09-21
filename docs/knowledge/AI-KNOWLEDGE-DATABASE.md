# GAIA 2.0 — AI Knowledge Database

**Research Date:** September 2026  
**Classification:** Foundational AI Intelligence Architecture Document  
**Status:** Living Document — Version 0.1  
**Scope:** Catalogue of All Major AI Models, Tools, Research Papers, and Benchmarks Relevant to GAIA 2.0  
**Issue:** #688

---

## Purpose

The AI Knowledge Database (AIKD) is GAIA 2.0's living catalogue of the AI landscape — every model, tool, dataset, paper, and benchmark relevant to Earth systems science, planetary intelligence, and human flourishing. It enables GAIAN to:

- Recommend the right AI tool for any Earth science task
- Surface relevant AI research for any GAIAN query
- Track the state of the art across all AI domains
- Provide citations and provenance for AI-generated insights

---

## Database Schema

### AI Model Record

```json
{
  "id": "uuid-v4",
  "name": "GraphCast",
  "type": "foundation_model | specialist_model | tool | dataset | paper | benchmark",
  "modality": "weather_forecasting",
  "organisation": "Google DeepMind",
  "release_date": "2023-11",
  "paper": "arXiv:2212.12794",
  "license": "Apache-2.0",
  "description": "10-day global weather forecast in 60 seconds. Outperforms ECMWF IFS on most metrics.",
  "performance": {
    "benchmark": "ECMWF comparison",
    "result": "Outperforms on 90%+ of metrics",
    "source": "Lam et al., Science 2023"
  },
  "gaia_applications": ["earth-twin", "planetary-boundary-monitor", "climate-adaptation"],
  "huggingface_url": "https://huggingface.co/google/graphcast",
  "papers_with_code": "https://paperswithcode.com/paper/graphcast",
  "tags": ["weather", "climate", "forecasting", "earth-science"]
}
```

---

## Catalogue Domains

### Domain 1: Earth Systems AI

| Model/Tool | Organisation | Capability | GAIA 2.0 Use |
|---|---|---|---|
| GraphCast | Google DeepMind | 10-day weather forecast, 60 seconds | Earth Twin weather layer |
| ESFM | EU Copernicus / Destination Earth | Earth System Foundation Model | Planetary boundary monitoring |
| TerraMind | ESA | Any-to-any Earth observation, 9 modalities | Satellite data fusion |
| Pangu-Weather | Huawei | Global weather at 1-hour resolution | Short-range forecasting |
| ClimaX | Microsoft Research | Climate + weather foundation model | Climate projection |
| Aurora | Microsoft | Atmospheric foundation model | Atmospheric science |
| PRITHVI | NASA + IBM | Geospatial foundation model | Land cover, disaster response |

### Domain 2: Biodiversity AI

| Model/Tool | Organisation | Capability | GAIA 2.0 Use |
|---|---|---|---|
| NatureLM-Audio | Earth Species Project | Bioacoustic species identification | Species monitoring |
| iNaturalist CV | iNaturalist | Visual species ID (millions of species) | Citizen science integration |
| BioCLIP | University of Wisconsin | Vision model for biodiversity | Species classification |
| GBIF Name Matching | GBIF | Taxonomic name resolution | Knowledge graph backbone |
| DeepBird | Various | Bird species ID from audio | Avian biodiversity monitoring |

### Domain 3: Scientific Discovery AI

| Model/Tool | Organisation | Capability | GAIA 2.0 Use |
|---|---|---|---|
| AlphaFold 3.2 | Google DeepMind | Protein structure + drug binding | Drug discovery, biochemistry |
| GNoME | Google DeepMind | 2.2M new crystal structures | Clean energy materials |
| OpenAlex | OurResearch | 250M+ open research papers | Literature search |
| Semantic Scholar | Allen AI | AI-powered research graph | Citation intelligence |
| AIMATRY | Various | AI materials design | Climate solutions |

### Domain 4: Foundation Models (General)

| Model | Organisation | Context Window | License | Best For |
|---|---|---|---|---|
| GPT-5 | OpenAI | 1M tokens | Proprietary | General reasoning, MMLU 92.1% |
| Claude Opus 4.8 | Anthropic | 200K tokens | Proprietary | Long-document, GPQA 93.6% |
| Gemini 2.5 Ultra | Google DeepMind | 2M tokens | Proprietary | Multimodal, coding |
| Llama 4 | Meta | 128K tokens | Open weights | On-premise deployment |
| Mistral Large 3 | Mistral AI | 128K tokens | Open weights | European sovereignty |
| DeepSeek R2 | DeepSeek | 128K tokens | Open weights | Math, reasoning |
| Qwen 3 | Alibaba | 128K tokens | Open weights | Multilingual |

### Domain 5: AI Research Papers (Key 2025–2026)

| Paper | Venue | Key Finding | Relevance |
|---|---|---|---|
| arXiv:2606.12683 | DeepMind | 4 paths from AGI to ASI | AI trajectory planning |
| arXiv:2607.07663 | Survey | Recursive self-improvement: 1,250 papers | AI R&D acceleration |
| arXiv:2509.20328 | ICML 2026 | Veo 3: zero-shot physics simulation | Earth simulation |
| arXiv:2503.05788 | Survey | Emergent AI abilities: 1,250+ papers | Capability monitoring |
| arXiv:2606.12835 | Various | Internet of Agentic AI | Multi-agent architecture |
| arXiv:2607.12125 | Various | Jagged Intelligence map | AI-Human routing |
| Stanford AI Index 2026 | HAI Stanford | Annual AI state-of-the-art | Benchmark tracking |

### Domain 6: AI Benchmarks

| Benchmark | Scope | SOTA Result (Sep 2026) | Human Baseline |
|---|---|---|---|
| MMLU | 57 academic subjects | 92.1% (GPT-5) | 89.8% experts |
| GPQA Diamond | PhD-level science | 93.6% (Claude Opus 4.8) | ~65% PhDs |
| HumanEval | Code generation | 96.7% | — |
| SWE-bench Verified | Real GitHub bugs | 95.0% (Claude Fable 5) | — |
| MATH-500 | Competition math | 97.3% (DeepSeek R1) | ~5% AIME solve |
| IMO | Math olympiad | Gold Medal (Gemini Deep Think, 2025) | Human champions |
| Humanity's Last Exam | Hard cross-domain | +30pp in 1 year (Stanford AI Index) | Expert humans |

---

## Seeding Strategy

- **Primary sources:** Hugging Face model cards, Papers with Code, arXiv, OpenAlex
- **Curation cadence:** Monthly review of new models and papers
- **Quality standard:** Each entry requires paper citation or official documentation
- **Target at launch:** 500+ models and tools catalogued
- **Community contributions:** Researchers can submit new entries via PR

## References

- Stanford AI Index 2026 (HAI, Stanford University)
- Hugging Face Open LLM Leaderboard (2026)
- Papers with Code State of the Art (2026)
- Google DeepMind — "From AGI to ASI" (arXiv:2606.12683, Jun 2026)
- MIT Technology Review — "10 Things That Matter in AI" (Apr 2026)
