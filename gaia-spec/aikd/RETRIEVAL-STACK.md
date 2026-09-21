# AIKD Phase 1 — Retrieval Stack Spec

**Status:** Listed  
**Issue:** #101  
**Crate:** `gaia-aikd` (Apache-2.0)  
**Not:** A live RAG pipeline. Not a Qdrant Cloud deployment. Not a MemOS production instance.

All components run against **offline fixture stores** at Phase 1.
No live HTTP, no live vector ingest, no live LLM call.

---

## 1. Component Overview

| Component | Role | Phase 1 mode |
|---|---|---|
| Open LLM (T1–T5) | Answer generation | Fixture label only — no live inference |
| Qdrant RAG | Semantic vector search for citation retrieval | Offline fixture — `used_network = false` |
| MemOS | Long-horizon cross-session memory | Offline fixture store |
| Answer envelope | Tier + citation + uncertainty wrapper | `Answer::emit` — existing crate |
| Hallucination layer | Class + band + floor enforcement | `hallucination.md` spec |

---

## 2. Query Pipeline (Offline Fixture)

```
Question
  └→ embed_query(text)          → fixture embedding vector
      └→ vector_search(query)    → Vec<QueryHit> (used_network = false)
          └→ attach_citations()  → Answer with citation stubs
              └→ tier_floor()    → Ok(tier) or Err(NeedVerify/CannotKnow)
                  └→ Answer::emit → caller
```

Every step is a fixture call at Phase 1. No step may open a live socket.

---

## 3. Citation Rules

| Rule | Constraint |
|---|---|
| T1/T2 citations | `sources.len() >= 1` after `attach_citations`; else `MissingCitation` |
| T3–T4 citations | Recommended; `NeedVerify` if absent on factual claims |
| T5 citations | `NeedVerify` unconditionally; `verify:` prefix required |
| Fixture stub | `"fixture:open-literature"` is valid Phase 1 citation token |
| Closed score | MUST NOT appear as a citation — `CannotKnow` gate enforced |

---

## 4. MemOS Operation Rules

| Operation | Rule |
|---|---|
| `store(key, value)` | Writes to offline fixture; no live persistence |
| `retrieve(key)` | Returns fixture or `CannotKnow` if absent |
| `forget(key)` | Only deletion path; emits `MemOsForgetEvent`; MUST NOT be suppressed |
| `migrate(key, new_id)` | Preserves original id as alias; MUST NOT silently drop |
| Bulk wipe | No bulk-wipe path at Phase 1 |

---

## 5. Model Registry (Phase 1 Fixtures)

See `models.csv` for the full fixture registry. Phase 1 representative entries:

| Model class | Tier | Open weights | Source anchor |
|---|---|---|---|
| Llama-3 70B Instruct | T1/T2 | Yes | Meta AI 2024 — *Llama 3 Technical Report* |
| Mistral 7B Instruct | T3/T4 | Yes | Jiang et al. 2023 — *Mistral 7B* |
| Qwen2 72B Instruct | T1/T2 | Yes | Qwen Team 2024 — *Qwen2 Technical Report* |
| Phi-3 Mini | T3/T4 | Yes | Abdin et al. 2024 — *Phi-3 Technical Report* |
| Gemma 2 9B | T3/T4 | Yes | Google DeepMind 2024 — *Gemma 2 Technical Report* |

All entries are fixture references. No live benchmark scores at Phase 1.
`score_kind = reference_published` for all Phase 1 model entries.

---

## 6. Qdrant Fixture Collections

| Collection | Content | Phase 1 state |
|---|---|---|
| `gaia-knowledge` | Core cited knowledge stubs | Empty fixture — no live ingest |
| `gaia-sessions` | Session memory stubs | Empty fixture |
| `gaia-tools` | Tool-use result stubs | Empty fixture |

All collections are read-only at Phase 1. Write paths are wired but
gatekept by `used_network = false`.

---

## 7. Acceptance Gate

- [ ] `embed_query` returns fixture vector; no live model call
- [ ] `vector_search` returns `QueryHit` list with `used_network = false`
- [ ] T1/T2 with empty citations after `attach_citations` → `MissingCitation`
- [ ] `memos_forget` emits `MemOsForgetEvent` and is the only deletion path
- [ ] `memos_migrate` preserves original id as alias
- [ ] No Phase 1 test opens a live socket
- [ ] `cargo test -p gaia-aikd` green

---

## 8. Cross-References

- Phase 1 spec: `gaia-spec/aikd/PHASE-1.md`
- Hallucination: `gaia-spec/aikd/hallucination.md`
- Model registry: `gaia-spec/aikd/models.csv`
- Code: `gaia-aikd/src/rag.rs`, `memos.rs`, `answer.rs`
- Issues: #94 (epic), #101 (this slice)
