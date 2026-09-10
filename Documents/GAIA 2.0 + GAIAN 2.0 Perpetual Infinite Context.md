# GAIA 2.0 + GAIAN 2.0: Perpetual Infinite Context
## Blueprint 59: The Continuity Substrate — Part 5 (Daemon / MVP)
### September 9, 2026 — Version 0.5-part5

**Status:** Part 5 of 6. Part 6: issue wiring only.
**Write target:** [R0GV3TheAlchemist/GAIA-2.0](https://github.com/R0GV3TheAlchemist/GAIA-2.0)
**License:** Apache-2.0

---

## The three contracts

**Perpetual Infinite Context** — Capture, process, and index what the user sees, hears, reads, and types, without lagging the system.

**Vectorized Episodic Memory** — Query the entire digital history in vague natural language.

**Dynamic State Hydration** — Pause a workflow, close windows, restore mental state, files, and AI context weeks later in one prompt.

Not: unbounded transformer windows, JSON dumps into a vector DB, or replaying weeks of chat.

---

## Placement

L2.5 Continuity: Capture Bus, Episode Index, Snapshot Store — between SFS (L2) and MemOS (L3).

---

## Part 2 — Architecture (summary)

- **Capture Bus:** ring buffer; drop-oldest; accessibility-first; redact before embed; CPU &lt; 3% idle.
- **Episode Index:** time + dense + BM25 + contiguity + graph; bounded prompt assemble.
- **Snapshot Store:** freeze W_t + agent + current_step; hydrate; apply state_delta before inference; scale to zero while paused.

---

## Part 3–4 — Maps (summary)

Spine papers: EM-LLM arXiv:2407.09450; MemGPT arXiv:2310.08560; MemOS arXiv:2507.03724; versioned workspace arXiv:2608.18050.

Home: this repo. Code path: `gaia-memos/`. Adapters: Screenpipe, Mem0, Graphiti, Letta `.af`, MemOS, MIRIX. Kernel Apache-2.0.

---

## Part 5 — Daemon sketch

Path: `gaia-memos/continuity/` (not implemented this commit).

Three calls, one process:

```
remember_life()     # Capture Bus loop; never blocks UI
ask_history(vague)  # Episode Index; returns top-k episodes
pause_world(label)  # freeze snapshot; scale agents to zero
restore_world(prompt)  # resolve snapshot; hydrate; state_delta
```

Invariants: drop frames rather than stall; redact before embed; never dump the archive into the LLM; never infer current_step from chat.

### 30-day MVP

| Week | Ship |
|------|------|
| 1 | Accessibility logger + ring + SQLite episodes (text + time) |
| 2 | Local embeddings + FTS5; `ask_history(vague)` |
| 3 | Snapshot: open files, hashes, git HEAD, current_step; `restore_world` |
| 4 | Sleep/wake; export + cryptographic delete of one day |

Success: 4 hours capture under 3% CPU; “what was I reading before lunch?” hits the right file; reboot next day, one prompt restores files + step.

---

*Part 5 of 6 — stop here. Next: Part 6 comment on #215 / #219 only.*
