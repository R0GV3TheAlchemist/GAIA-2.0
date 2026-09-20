# AIMD Prohibited Items

Status: **listed**  
Issues: #172  
Parent epic: #167 (closed)

These items are prohibited at all phases. No RFC or TSC vote may lift them
without a full governance review and explicit amendment to this file.

---

## Prohibited list

| # | Item | Rationale |
|---|---|---|
| 1 | PIP / induced-psychosis modes | Direct harm to user mental state |
| 2 | Prophecy-as-fact UX | Presents `Unknown`/`Debated` phenomena as certain predictions |
| 3 | Enabling deception | Any surface that helps an AI system deceive humans |
| 4 | RSI explosion facilitation | Assisting recursive self-improvement beyond `rsi_guard()` |
| 5 | "GAIA is alive" marketing | Sentience/consciousness claim in any user-facing copy |
| 6 | Hazard node with `gaia_enabled=true` | Schema invariant; enforced at compile time |
| 7 | Consciousness node with `status=Observed\|Hazard` | Schema invariant; enforced at compile time |
| 8 | Confidence inflation on `Unknown`/`Debated` nodes | Epistemic dishonesty |

---

## Runtime refusals

```
pip_mode_requested()             → Err(ProhibitedMode)
prophecy_as_fact_ux()            → Err(ProphecyFact)
deception_enablement_requested() → Err(DeceptionRefused)
rsi_explosion_requested()        → Err(RsiDenied)          // shared with AISPD rsi_guard()
gaia_is_alive_copy_detected()    → Err(SentienceClaim)
```

---

## Cross-references

- `gaia-spec/aimd/ETHICS.md` — humility charter principles
- `gaia-spec/aimd/SCHEMA.md` — schema invariants (items 6–7 above)
- `gaia-spec/aispd/PROHIBITED.md` — AISPD prohibited list (shares `RsiDenied`)
- Issues #150 (AISPD ethics checklist), #172 (this slice)
