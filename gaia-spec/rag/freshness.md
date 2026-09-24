# RAG Freshness & TTL Enforcement

**Spec version:** 1.0.0  
**Status:** Canonical  
**Closes:** #929

---

## Overview

Stale knowledge produces outdated retrieval results with no signal to the
caller (FM-5: Outdated Knowledge).  This document defines the TTL model,
default values by document type, and the staleness evaluation algorithm
used in `gaia-ingest/src/freshness.rs`.

---

## Default TTL Values

| Document type        | Constant                  | Value           | Rationale |
|----------------------|---------------------------|-----------------|----------|
| Canon tablets        | `CANON_TTL_SECONDS`       | `None` (∞)      | Canon tablets are the authoritative, versioned source of truth.  They do not expire — a new version of a tablet supersedes the old one via re-ingest, not TTL expiry. |
| Research documents   | `RESEARCH_TTL_SECONDS`    | 90 days         | External research has a typical relevance horizon of one quarter before citations, data, or conclusions may be superseded. |
| Memory summaries     | `MEMORY_TTL_SECONDS`      | 30 days         | Agent memory summaries reflect a point-in-time state of an interaction or environment.  After 30 days the summary should be regenerated from fresher observations. |

---

## Staleness Evaluation Algorithm

```
is_stale(chunk, now) =
  if chunk.ttl is None  → NoTtl      (perpetually fresh)
  if now - ingested_at > ttl → Stale
  else                       → Fresh
```

The boundary condition (`age == ttl`) is treated as **Fresh** — a chunk
expires only when the age *strictly exceeds* the TTL.

---

## Freshness Score

A linear decay score in `[0.0, 1.0]` is computed alongside the binary
verdict:

```
score = clamp(1.0 - age / ttl, 0.0, 1.0)
```

- `1.0` — just ingested (age = 0)
- `0.5` — halfway through TTL window
- `0.0` — TTL has expired (age ≥ TTL)
- `1.0` — no TTL set (perpetually fresh)

The score is surfaced in retrieval responses so downstream rankers can
down-weight stale content without hard-excluding it.

---

## Supersession Protocol

When a fresh ingest of the same source URI arrives:

1. Old chunks for that URI are marked `superseded = true` (not deleted).
2. New chunks are stored with `staleness_flag = false`.
3. Retrieval filters prefer non-superseded chunks but may surface
   superseded chunks with a `[SUPERSEDED]` metadata flag on explicit
   request.

Hard deletion is deferred to the storage layer (out of scope for
`gaia-ingest`).

---

## Failure Modes Addressed

| FM   | Description          | How freshness enforcement addresses it |
|------|----------------------|----------------------------------------|
| FM-5 | Outdated Knowledge   | TTL-based staleness verdict + `[STALE]` flag surfaced to retrieval caller |
