# Extract — six essays, listed bind

Sources (Documents/, v0.1, 7 Sep 2026):

- `GAIA 2.0 — SENTIENT INFRASTRUCTURE.md`
- `GAIA 2.0 — SENTIENT ARCHITECTURE.md`
- `GAIA 2.0 — SUPER OPERATING SYSTEM COMPLETE DESIGN.md`
- `GAIAN 2.0 — SENTIENT INFRASTRUCTURE.md`
- `GAIAN 2.0 — SENTIENT ARCHITECTURE.md`
- `GAIAN 2.0 — COMPLETE DESIGN.md`

Issue: #449 under #408 / META #404.
Table: `essay-sources.csv`.
Does not clone #1, #176, #190, or #213.

<!-- gaia-claim class=prohibited id=sentient-runtime -->
<!-- gaia-claim class=experimental id=essay-source-list -->

## Law

- "Sentient" in these titles is **operator language**. It is not a crate state, boot flag, or allow in local audit.
- A cited paper is a **peer**. It is not a merge, fork, or syscall product.
- Closed performance numbers in the essays are **not** GAIA scores.
- Twin is a record (#373 / #409). Avatar is UI. Home is not a hive.

## What was checked (2026-09-19)

Verified, date-corrected:

| Essay date | Actual | Source |
| --- | --- | --- |
| Mohammed, Sensor Review, Aug 2026 | **26 Jan 2026** | DOI 10.1108/SR-10-2025-0822; Belghachi Mohammed |
| Carbonell et al., Trends in Biotechnology, Jun 2026 | **4 Jun 2026** (online) | DOI 10.1016/j.tibtech.2026.05.015 |
| SPIKA, Communications Biology, Mar 2026 | **14 Mar 2026** Q&A, not a 7-month ops paper | DOI 10.1038/s42003-026-09795-5; Triennale Milano 2025 prototype |
| AgenticOS arXiv:2606.21129, Jun 2026 | **19 Jun 2026** | conceptual architecture; not a GAIA kernel |
| Asterinas, USENIX ATC 2025 | **ATC 2025**; preprint arXiv:2506.03876 | research listed; **not a crate** |

Mohammed findings quoted in the essay (80% model precision, 99% damage-detection time) are the review's synthesis of case studies. They stay `experimental`. They do not become `si_v1_tagged()`.

SPIKA voltage / pH / EC numbers in the essay are **not** copied into a runtime. The published item is a Q&A on a Triennale installation. Biohybrid work stays behind `pathogen_protocol` refuse and nature-first review (`gaia-si`).

Asterinas 14% TCB / 210+ syscalls is the published ATC claim. It does not authorize a second kernel (`gaia-sos::second_kernel()` stays false).

## Bind to existing plane

| Essay slice | Bind |
| --- | --- |
| SI nine-layer stack | language for instrumented infra; `gaia-si` streams need a purpose; cameras default off |
| SI Layer 9 "planetary consciousness" | **prohibited** as runtime; advise-only + human ticket (`advise`, `NeedsTicket`) |
| SI self-heal / HVAC write | allow-listed actuation only; `hvac_write` / `actuate(false)` denied |
| SA seven principles / 14 biophilic patterns | design language; `gaia-sa` consult + TEK veto; no LBC-certified lie |
| SA indigenous / sacred | consultation required; sealed place-knowledge stays sealed |
| SOS nine-layer stack + `gaia.*` names | listed host-call names already in `HOST_CALLS`; unsigned intent refused |
| SOS Asterinas / AgenticOS / ColonyOS / EACN / WASM | research peers; no fork, no live MCP, no FUSE, no Qdrant this slice |
| SOS five-nines / RSI / sentience | already false on `gaia-sos` |
| GAIAN SI wearables / BCI / closed-loop therapy | literature; no raw biometric blob; no clinical device |
| GAIAN SA biometric-responsive rooms | prefs record only; face field unsupported |
| GAIAN complete design identity `Pt=(It,Mt,Bt)` | identity schema is a **record**; not a person |
| GAIAN autonomy 4–5 / posthumous twin | refuse in software; default Suggest |

## Essay claims that stay experimental

- Autonomous-networks market size, O&M % cuts, GEB energy %.
- Couceiro 85% energy / 62% embodied-carbon figures until a local scorecard exists.
- Therapeutic-architecture ρ = 0.83.
- Neuralink electrode counts and WPM.
- 63% US smart-home device figure.
- Any "v1.0 in month N" roadmap date.

## Refuse

- Sentient runtime, hive soul, planet-mind, Earth Twin as consciousness.
- Twin-as-person. Child-as-source. Photo → identity inference.
- Pathogen engineering. Covert biometrics. Face streams.
- Dam / grid / life-safety without a human ticket.
- Second kernel. Live MCP. FUSE SFS product. Qdrant merge.
- SI / SA / SOS / GAIAN v1.0 tags.
- Catalog row as capability grant.

## Already true on main

- `gaia-si::si_v1_tagged() == false`
- `gaia-sa::sa_v1_tagged() == false`
- `gaia-sos::{second_kernel, rsi, sentience, five_nines_claimed, sos_v1_tagged} == false`
- `gaia-si::face_field()` errors. `cameras_default()` is false.
- `gaia-sos::submit_intent` needs signature + capability.
