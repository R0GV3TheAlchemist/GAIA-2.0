# GAIA 2.0 — Universal Human Skills Database

**Research Date:** September 7, 2026  
**Classification:** Foundational Human Skills Architecture Document  
**Status:** Living Document — Version 0.1  
**Scope:** Every Skill a Human Can Develop — From First Breath to Mastery  
**Issue:** #688

---

## Core Distinction: Skills vs. Knowledge

Skills are fundamentally different from knowledge. This database is about **what humans can DO**, not just what they know.

```
KNOWLEDGE = "I know that swimming involves arm strokes and breathing"
SKILL     = "I can swim 1,000 meters without stopping"

KNOWLEDGE = "I know the theory of negotiation"
SKILL     = "I can negotiate a salary increase effectively"
```

## Skill Node Schema

```json
{
  "id": "uuid-v4",
  "name": "Active Listening",
  "category": "interpersonal",
  "domain": "communication",
  "type": "behavioral | cognitive | physical | creative | social | emotional",
  "description": "...",
  "prerequisites": ["basic-attention", "empathy-awareness"],
  "enables": ["conflict-resolution", "counseling", "negotiation", "leadership"],
  "mastery_levels": {
    "novice": "...",
    "competent": "...",
    "proficient": "...",
    "expert": "...",
    "master": "..."
  },
  "esco_id": "...",
  "onet_id": "...",
  "wef_category": "social-emotional",
  "ai_replaceability": "low | medium | high",
  "cultural_variations": "..."
}
```

## Mastery Model (Dreyfus, 6 Stages)

```
1. NOVICE          → Follows rules rigidly
2. ADV. BEGINNER   → Recognises patterns
3. COMPETENT       → Solves problems independently
4. PROFICIENT      → Holistic, experience-based decisions
5. EXPERT          → Intuitive, fluid, deep contextual understanding
6. MASTER          → Innovates, pushes boundaries, creates new approaches
```

## Classification Frameworks Synthesised

| Framework | Origin | Scope |
|---|---|---|
| ESCO | EU Commission | 13,485 skills |
| O*NET | US Dept. of Labor | 19,000+ skills |
| BESSI | Soto & Napolitano | 5 domains, 32 fine-grained skills |
| DigComp 2.2 | EU JRC | 5 areas, 21 digital competencies |
| WEF Future of Jobs 2025 | World Economic Forum | Top 10 rising skills to 2030 |
| Bloom's Psychomotor | Simpson/Dave/Harrow | Physical skills, 7 levels |
| UNESCO Skills for Future | UNESCO | Cognitive, socio-emotional, functional |

## WEF Top 10 Rising Skills (2025–2030)

1. AI and Big Data literacy
2. Networks and Cybersecurity
3. Technological literacy
4. Creative thinking
5. Resilience, flexibility and agility
6. Curiosity and lifelong learning
7. Leadership and social influence
8. Talent management
9. Analytical thinking
10. Environmental stewardship

> Key finding: 39% of key job skills will change by 2030. 170M new jobs created; 92M displaced. (WEF Future of Jobs 2025)

## The 9 Skill Realms

```
REALM 1: COGNITIVE & INTELLECTUAL
  ├── Analytical & critical thinking
  ├── Creative thinking
  ├── Learning & metacognitive skills
  ├── Language & communication (cognitive)
  └── Strategic & systems thinking

REALM 2: PHYSICAL & MOTOR
  ├── Fundamental movement skills
  ├── Fine motor skills
  ├── Sports & athletic skills
  ├── Physical fitness
  ├── Dance & movement arts
  ├── Manual & craft skills
  └── Sensory & perceptual skills

REALM 3: SOCIAL & INTERPERSONAL
  ├── Communication skills
  ├── Social engagement skills (BESSI Domain 1)
  ├── Cooperation skills (BESSI Domain 2)
  ├── Relationship skills
  └── Cultural & intercultural skills

REALM 4: EMOTIONAL & SELF-MANAGEMENT
  ├── Emotional intelligence (Goleman model)
  ├── Self-management skills (BESSI Domain 3)
  ├── Emotional resilience (BESSI Domain 4)
  ├── Mindfulness & contemplative skills
  └── Personal development skills

REALM 5: CREATIVE & ARTISTIC
  ├── Visual art skills
  ├── Music skills
  ├── Performing arts
  ├── Writing & literary skills
  ├── Design skills
  └── Culinary arts

REALM 6: DIGITAL & TECHNOLOGICAL (DigComp 2.2)
  ├── Information & data literacy
  ├── Communication & collaboration
  ├── Digital content creation
  ├── Digital safety
  ├── Digital problem solving
  └── Advanced technical skills

REALM 7: PROFESSIONAL & LEADERSHIP
  ├── Leadership skills
  ├── Management skills
  ├── Entrepreneurship
  ├── Sales & marketing
  ├── Research & analytical skills
  └── Teaching & facilitation

REALM 8: PRACTICAL LIFE SKILLS
  ├── Health & wellness
  ├── Financial life skills
  ├── Home & household
  ├── Food & nutrition
  ├── Transportation & navigation
  └── Legal & civic skills

REALM 9: SURVIVAL & OUTDOOR
  ├── Wilderness survival
  ├── Navigation & wayfinding
  ├── First aid & emergency response
  ├── Ecological skills (farming, restoration, naturalism)
  └── Traditional survival knowledge
```

## GAIA 2.0 Integration

- Skills link to knowledge domains in `catalog-v2.json` via `enables` and `prerequisites`
- AI-replaceability scores guide AI-Human complementarity routing
- CARE-tagged traditional skills have community-controlled access
- All skill nodes queryable via RAG for GAIAN recommendations
- `skills-catalog-v1.json` is the machine-readable seed for this database

## References

- ESCO v1.2 (European Commission, 2023)
- O*NET 28.0 (US Dept. of Labor, 2024)
- BESSI Framework (Soto & Napolitano, 2022)
- DigComp 2.2 (EU JRC, 2022)
- WEF Future of Jobs Report 2025
- UNESCO Skills for the Future Framework
