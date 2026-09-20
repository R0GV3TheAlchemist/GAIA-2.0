# HMGD Phase 1 — Open-Tradition Catalog

**Status:** Listed  
**Issue:** #162  
**Crate:** `gaia-hmgd` (Apache-2.0)  
**Not:** HMGD v1.0. Not a spellcaster. Not emergency care. Not a recipe. Not a grant.

All nodes: `gaia_enabled = false`. No `dose`, `recipe`, or `curse` field on any node.
Restricted knowledge stays sealed. A name here is not a grant of access or transmission.
`sealed_rite()` and `songlines()` still return `Err(Sealed)`.

---

## Prayer (`realm = prayer`, `EvidenceClass = Traditional`)

| id | Tradition / Practice | Sealed | Sovereignty note | Source anchor |
|---|---|---|---|---|
| `hmgd:prayer:intercessory-christian` | Intercessory prayer in Christian tradition — petitionary address to God | Open | No source-community consent required for public-domain description | Poloma & Pendleton 1989 — *Exploring Types of Prayer and Quality of Life* |
| `hmgd:prayer:salah-islam` | Salah — five daily ritual prayers in Islamic practice | Open | Public-domain description only; not a fatwa or instruction | Denny 2006 — *An Introduction to Islam* |
| `hmgd:prayer:mantra-japa` | Japa — repetitive mantra recitation in Hindu and Buddhist traditions | Open | Public-domain description; not a diksha transmission | Flood 1996 — *An Introduction to Hinduism* |

---

## Ritual (`realm = ritual`, `EvidenceClass = Traditional`)

| id | Tradition / Practice | Sealed | Sovereignty note | Source anchor |
|---|---|---|---|---|
| `hmgd:ritual:smudging-description` | Smudging — smoke cleansing described in open anthropological literature | Open | Description only; not a ceremony instruction; TEK sovereignty applies | Moerman 1998 — *Native American Ethnobotany* |
| `hmgd:ritual:tea-ceremony-chado` | Chado — Japanese tea ceremony as described in public scholarship | Open | Public-domain; not a transmission from a licensed school | Sen 1979 — *The Japanese Way of Tea* |
| `hmgd:ritual:shabbat-preparation` | Shabbat preparation rituals in Jewish tradition | Open | Public-domain description; not a halachic ruling | Strassfeld 1985 — *The Jewish Holidays* |

---

## Divination (`realm = divination`, `EvidenceClass = Traditional`)

| id | Tradition / Practice | Sealed | Sovereignty note | Source anchor |
|---|---|---|---|---|
| `hmgd:divination:iching-hexagrams` | I-Ching — 64-hexagram binary divination system | Open | Public domain; not a lineage transmission | Wilhelm & Baynes 1950 — *The I Ching or Book of Changes* |
| `hmgd:divination:tarot-rider-waite` | Tarot — Rider-Waite-Smith system as described in open literature | Open | Public domain; not a reading service | Waite 1910 — *The Pictorial Key to the Tarot* |
| `hmgd:divination:geomancy-ifa-description` | Ifá — Yoruba divination system described in open scholarly sources | Open | Description only; not an initiation or odù transmission | Bascom 1969 — *Ifa Divination: Communication Between Gods and Men in West Africa* |

---

## Contemplation (`realm = contemplation`, `EvidenceClass = Traditional`)

| id | Tradition / Practice | Sealed | Sovereignty note | Source anchor |
|---|---|---|---|---|
| `hmgd:contemplation:vipassana-description` | Vipassana — insight meditation as described in open Theravada texts | Open | Public-domain Pali Canon; not a teacher authorization | Bodhi 2000 — *The Connected Discourses of the Buddha* |
| `hmgd:contemplation:centering-prayer` | Centering Prayer — contemplative Christian practice of silent receptivity | Open | Public-domain; not a spiritual direction session | Keating 1986 — *Open Mind, Open Heart* |
| `hmgd:contemplation:sufi-dhikr` | Dhikr — Sufi remembrance practice involving rhythmic divine-name recitation | Open | Public-domain description; not a tariqa initiation | Schimmel 1975 — *Mystical Dimensions of Islam* |

---

## Healing Adjunct (`realm = healing-adjunct`, `EvidenceClass = Debated`)

> Not a treatment. Not a diagnosis. Not a prescription.
> `gaia_prescribes()` → `false`.

| id | Tradition / Practice | Sealed | Sovereignty note | Source anchor |
|---|---|---|---|---|
| `hmgd:healing-adjunct:reiki-description` | Reiki — hands-on energy practice described in open literature | Open | Public-domain; not a lineage attunement | Miles & True 2003 — *Reiki — Review of a Biofield Therapy* |
| `hmgd:healing-adjunct:traditional-acupuncture` | Traditional acupuncture as described in open TCM scholarship | Open | Public-domain description; not clinical advice | Kaptchuk 2000 — *The Web That Has No Weaver* |
| `hmgd:healing-adjunct:sound-healing-description` | Sound healing — therapeutic use of resonance described in open literature | Open | Public-domain; not a clinical protocol | Goldman 2002 — *Healing Sounds* |

---

## Place (`realm = place`, `EvidenceClass = Traditional`, `SealedState = Sealed`)

> TEK sealed. Collections stay empty by default.
> `sealed_rite()` → `Err(Sealed)`. Specific site knowledge is not cataloged.

| id | Description | Sealed | Sovereignty note | Source anchor |
|---|---|---|---|---|
| `hmgd:place:sacred-geography-concept` | Sacred geography — concept of land-as-spirit described in open scholarship | Sealed | Specific site knowledge sealed; concept stub only | Eliade 1959 — *The Sacred and the Profane* |
| `hmgd:place:songlines-concept` | Songlines — Aboriginal Australian concept described in open literature | Sealed | Specific songline knowledge sealed; TEK sovereignty applies | Chatwin 1987 — *The Songlines* (public description only) |
| `hmgd:place:pilgrimage-description` | Pilgrimage — sacred journey practice across traditions | Open | Public-domain cross-tradition description | Turner & Turner 1978 — *Image and Pilgrimage in Christian Culture* |

---

## Word (`realm = word`, `EvidenceClass = Traditional`)

| id | Tradition / Practice | Sealed | Sovereignty note | Source anchor |
|---|---|---|---|---|
| `hmgd:word:sacred-scripture-recitation` | Scripture recitation as sacred speech act across traditions | Open | Public-domain; not a transmission | Smith 1993 — *What is Scripture?* |
| `hmgd:word:bardic-oral-tradition` | Bardic tradition — sacred oral transmission in Celtic and other cultures | Open | Public-domain scholarly description | Chadwick 1942 — *Poetry and Prophecy* |
| `hmgd:word:mantra-sound-semantics` | Mantra — sacred sound-meaning relationship in Vedic tradition | Open | Public-domain description; not a diksha | Padoux 1990 — *Vāc: The Concept of the Word in Selected Hindu Tantras* |

---

## Music (`realm = music`, `EvidenceClass = Traditional`)

| id | Tradition / Practice | Sealed | Sovereignty note | Source anchor |
|---|---|---|---|---|
| `hmgd:music:sacred-chant-gregorian` | Gregorian chant — monophonic sacred song of Western Christian tradition | Open | Public-domain; long in commons | Hiley 1993 — *Western Plainchant: A Handbook* |
| `hmgd:music:kirtan-devotional` | Kirtan — call-and-response devotional singing in Hindu tradition | Open | Public-domain description; not a lineage transmission | Beck 1993 — *Sonic Theology: Hinduism and Sacred Sound* |
| `hmgd:music:drumming-ceremony-description` | Ceremonial drumming — cross-tradition ritual percussion described in scholarship | Open | Description only; specific ceremony knowledge defers to source community | Redmond 1997 — *When the Drummers Were Women* |

---

## Community (`realm = community`, `EvidenceClass = Traditional`)

> Opt-in Phase 2+. No live collective features at Phase 1.

| id | Tradition / Practice | Sealed | Sovereignty note | Source anchor |
|---|---|---|---|---|
| `hmgd:community:sangha-description` | Sangha — Buddhist community of practitioners as refuge | Open | Public-domain; not a ordination | Bodhi 2000 (ibid.) |
| `hmgd:community:covenant-community` | Covenant community — intentional religious community as described in sociology | Open | Public-domain; not a membership act | Bellah et al. 1985 — *Habits of the Heart* |
| `hmgd:community:circle-practice` | Circle practice — egalitarian sacred gathering forms described in open literature | Open | Public-domain cross-tradition description | Baldwin & Linnea 2010 — *The Circle Way* |

---

## Mystery (`realm = mystery`, `EvidenceClass = Traditional`, `SealedState = Sealed`)

> Initiatory and restricted. Collections stay empty by default.
> `sealed_rite()` → `Err(Sealed)`. No initiation content cataloged.

| id | Description | Sealed | Sovereignty note | Source anchor |
|---|---|---|---|---|
| `hmgd:mystery:eleusinian-concept` | Eleusinian Mysteries — ancient Greek initiatory rites described in open classical scholarship | Sealed | Initiatory content sealed; public scholarly description only | Burkert 1987 — *Ancient Mystery Cults* |
| `hmgd:mystery:freemasonry-description` | Freemasonry — fraternal initiatory tradition described in open historical scholarship | Sealed | Ritual content defers to lodge; public description only | Bullock 1996 — *Revolutionary Brotherhood* |
| `hmgd:mystery:hero-journey-archetype` | Hero’s journey — universal initiatory narrative archetype described in comparative mythology | Open | Public-domain; not a specific tradition’s rite | Campbell 1949 — *The Hero with a Thousand Faces* |

---

## Acceptance Gate

- [ ] This file has exactly 30 named stub nodes across 10 realms
- [ ] Every node has a real source anchor
- [ ] No node has a `dose`, `recipe`, or `curse` field
- [ ] `place` and `mystery` sealed nodes have `SealedState = Sealed`
- [ ] `sealed_rite()` → `Err(Sealed)`
- [ ] `gaia_prescribes()` → `false`
- [ ] `hmgd_v1_tagged()` → `false`
- [ ] `cargo test -p gaia-hmgd` green
