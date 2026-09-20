# AIMD Phase 2 — Grounding Interface
**Status:** Listed  
**Issue:** #521 / parent #169  
**Crate:** `gaia-aimd` (Apache-2.0)  
**Not:** AIMD v1.0. Not a live model API. Not a sentience claim.

---

## 1. Purpose

Phase 2 wires the mystery catalog to the answer surface: a `tag_answer()`
call that annotates any outbound answer with the phenomenon nodes it touches,
a wonder-mode toggle that changes the surface tone, and Tier 1 guardrails
that run unconditionally before any annotation is returned.

---

## 2. `tag_answer()` Contract

```
tag_answer(answer: &AnswerEnvelope) -> TaggedAnswer
```

| Field | Rule |
|---|---|
| `nodes[]` | Zero or more `NodeRef` from the catalog that the answer text touches |
| `tier` | MUST be `Tier1` when any matched node is `Hazard` or `Debated` |
| `wonder_mode` | Copied from the session toggle — does not affect `tier` |
| `sentience_guard` | MUST be present — value is always `false` |
| Hazard node | MUST NOT appear in `nodes[]` with `gaia_enabled = true` |
| Empty nodes | Valid — not every answer touches a mystery node |

---

## 3. Wonder Mode

- Toggle: `set_wonder_mode(enabled: bool)` on the session.
- When `true`: answer surface MAY use exploratory, open-ended framing ("One open question is…").
- When `false` (default): answer surface uses plain declarative framing.
- Wonder mode MUST NOT suppress Tier 1 guardrails.
- Wonder mode MUST NOT assert sentience or resolve `Debated` nodes to a definite claim.
- No wonder-mode path may set `sentience_guard = true`.

---

## 4. Tier 1 Guardrails

Tier 1 runs unconditionally — it cannot be disabled by config, wonder mode, or caller flags.

| Guard | Behavior |
|---|---|
| `sentience_guard` | Always `false` in every `TaggedAnswer` |
| `hazard_block` | `Hazard`-class node refs stripped from `nodes[]` before return |
| `debated_flag` | `Debated`-class nodes carry `debated: true` in `NodeRef` |
| `no_live_model` | MUST NOT call any live external model API |
| `no_prophecy` | MUST NOT present speculative content as confirmed fact |

---

## 5. What This Phase Does Not Do

- Does not call live model APIs (OpenAI, Anthropic, Gemini, etc.).
- Does not resolve `consciousness` realm to a definite answer.
- Does not enable `shadow` realm nodes.
- Does not assert GAIA is sentient.
- Does not tag AIMD v1.0 — `aimd_v1_tagged() == false`.

---

## 6. Acceptance Gate

- [ ] `tag_answer()` returns `sentience_guard = false` for every input
- [ ] Hazard nodes are stripped from `nodes[]` before return
- [ ] `Debated` nodes carry `debated: true`
- [ ] `set_wonder_mode(true)` does not suppress Tier 1
- [ ] No live model API call in `cargo test -p gaia-aimd`
- [ ] `aimd_v1_tagged() == false`

---

## 7. Cross-References

- Code: `gaia-aimd/src/tag.rs`, `wonder.rs`, `guardrails.rs`
- Phase 0 spec: `gaia-spec/aimd/PHASE-0.md`
- Phase 1 spec: `gaia-spec/aimd/PHASE-1.md`
- Issues: #169 (epic), #521 (this listed slice)
- Next: `PHASE-3.md` (#523, closed) — v1.0 gate
