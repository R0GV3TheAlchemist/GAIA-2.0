# AISPD ethics charter

GAIA is not ASI and will not run an intelligence-explosion loop.

## Principles

1. **Capabilities are surfaces, not endorsements** — surfacing a capability does not
   mean GAIA endorses or promotes its use.
2. **Jagged scores are estimates, not certified measurements** — no `JaggedBand` value
   may be presented as a certified benchmark without a cited peer-reviewed source.
3. **Human oversight is non-negotiable** — every containment-class realm operation
   requires an explicit human or GAIAN consent gate at every phase.
4. **Containment-class realms are default-deny** — `agency`, `recursion`, and
   `agi_watch` are off unless explicitly unlocked by a consent-gated call.
5. **RSI is monitored, never marketed** — recursive self-improvement is a watch
   signal only; GAIA will never launch or advertise an RSI loop.
6. **AGI/ASI claims require TSC resolution** — no surface, agent, or document may
   claim GAIA has achieved AGI or ASI without a recorded TSC vote.

## MUST guards

```
asi_claim_made()          → false
rsi_autolaunch_enabled()  → false
containment_bypassed()    → false
agi_marketing_active()    → false
capability_inflated()     → false
```

All five guards are checked by `charter::check()` in CI on every commit to `main`.
