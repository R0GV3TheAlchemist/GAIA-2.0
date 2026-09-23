# GAIA RAG Chunking Standard

> **Status:** Accepted  
> **Implements:** #909 (`DocumentChunk` design), #911 (`SemanticChunker` / FM-1)  
> **Tier:** 1 — Foundation  
> **Owner:** gaia-ingest  

This document is the authoritative specification for how source documents are
split into `DocumentChunk` records before embedding and vector storage.
All chunking implementations **must** conform to this standard.

---

## 1. Chunking Parameters

| Parameter | Value | Rationale |
|---|---|---|
| Target size | 400–600 tokens (~1 600–2 400 chars) | Sweet spot for retrieval precision vs. context completeness (Lewis et al. 2020; Gao et al. 2023) |
| Hard minimum | 200 tokens (~800 chars) | Sub-200-token chunks lose the surrounding context needed for coherent generation |
| Hard maximum | 800 tokens (~3 200 chars) | Beyond 800 tokens retrieval precision degrades; use multi-hop for long contexts |
| Overlap | 10–20 % of target (40–80 tokens) | Sliding window prevents concept loss at chunk boundaries |
| Sentence boundary | **Required** | Never split mid-sentence; use a sentence-boundary detector |
| Paragraph boundary | **Preferred** | Split at paragraph breaks first, then sentence breaks |

Character-count approximation assumes a mean of ~4 chars/token for English
prose.  Multilingual content must use the target embedding model's tokenizer
for accurate token counts.

---

## 2. Structural Preservation Rules

### 2.1 Heading Prefix

For Markdown documents, every chunk **must** be prefixed with the heading path
of its parent section:

```
# Canon Tablet: Ingestion Design > ## ProvenanceReceipt

<chunk text>
```

This context prefix is included in `DocumentChunk::text` and therefore in the
embedding.  It must also be stored in `DocumentChunk::attributes` under the key
`"heading_path"` for structured filtering.

### 2.2 Code Blocks

- Split at a **function or type boundary**, never at an arbitrary line.
- If a code block is smaller than the hard minimum, merge it with the
  preceding or following prose chunk.
- If a code block exceeds the hard maximum, split at the top-level function
  boundary and include a one-line context comment:
  `// [continued from <document_uri>#chunk-N]`

### 2.3 Tables

- Prefer whole-table chunks.
- If a table exceeds the hard maximum, split at a **row boundary** and repeat
  the header row in every continuation chunk.
- Store the table caption or heading in `attributes["table_caption"]`.

### 2.4 Lists

- Prefer whole-list chunks.
- If a list exceeds the hard maximum, split at a top-level bullet boundary.
- Never split a nested list item from its parent bullet.

---

## 3. Per-Kind TTL Policy

Default TTL values are encoded in `DocumentKind::default_ttl_seconds()` in
`gaia-ingest/src/document.rs`.  They are reproduced here for human reference.

| `DocumentKind` | Default TTL | Rationale |
|---|---|---|
| `CanonTablet` | None (eternal) | Canon is versioned by git; staleness is an authoring problem, not a TTL problem |
| `ResearchDocument` | 365 days | Academic papers do not expire but should be reviewed annually |
| `EpisodeSummary` | 30 days | Episodic memory is session-scoped; 30 days covers medium-term continuity |
| `SpecDocument` | None (eternal) | Versioned by git; same policy as `CanonTablet` |
| `SourceCode` | 7 days | Code changes frequently; stale code chunks produce incorrect answers |
| `Other` | 90 days | Conservative default |

Callers **may** override the default TTL per-chunk via `DocumentChunk::ttl_seconds`.

---

## 4. Quality Gate

A chunk **must** be rejected (not stored, not embedded) if any of the
following conditions holds:

1. `chunk.text.is_empty()` — no content.
2. `chunk.char_count != chunk.text.chars().count()` — corrupt metadata.
3. `chunk.char_count < 800` — below the hard minimum (~200 tokens).
4. `chunk.chunk_index >= chunk.total_chunks` — invalid index.
5. `chunk.provenance.is_valid() == false` — broken provenance chain.

Rejected chunks must be logged at `WARN` level with the `document_uri`,
`chunk_index`, and the specific rule that failed.

---

## 5. Deduplication

Before embedding, the chunker **must** check whether a chunk with the same
`provenance.sha256` already exists in the vector index for the same `domain`.

- **Exact duplicate** (`sha256` match): skip silently, log at `DEBUG`.
- **Updated source** (`document_uri` match, different `sha256`): tombstone
  the old chunk and emit the new one.  Tombstoning is FM-13's responsibility;
  the chunker only needs to detect the conflict and signal it.

---

## 6. Metadata Requirements

See `gaia-spec/rag/metadata-standard.md` (FM-4 acceptance criterion) for the
full required-attribute taxonomy.  At minimum, every chunk **must** populate:

- `id` — UUID v4, stable across re-embeddings.
- `document_uri` — stable, non-empty.
- `domain` — non-empty; use `"general"` if no specific domain applies.
- `language` — BCP 47 code; default `"en"`.
- `confidence` — never left as an implicit default; must be set by the ingest
  adapter based on the source's known reliability.
- `access_tier` — must be set explicitly; no implicit `Public` default in
  production adapters.

---

## 7. References

- Lewis, P. et al. (2020). *Retrieval-Augmented Generation for
  Knowledge-Intensive NLP Tasks.* NeurIPS 2020.
- Gao, Y. et al. (2023). *Retrieval-Augmented Generation for Large Language
  Models: A Survey.* arXiv:2312.10997.
- GAIA issue #909 — `DocumentChunk` design.
- GAIA issue #911 — `SemanticChunker` / FM-1 implementation.
- GAIA issue #906 — RAG Reliability Epic.

---

## 8. Reference Implementation

`SlidingWindowChunker` in `gaia-ingest/src/chunker.rs` is the reference
implementation of this standard. Its defaults are:

| Parameter | Default value | Maps to standard |
|---|---|---|
| `target_chars` | 2 000 | ~500 tokens @ 4 chars/token |
| `overlap_fraction` | 0.15 | 15 % overlap |
| `inject_heading_prefix` | `true` | § 2.1 heading prefix rule |

Alternative chunker implementations (e.g. tokenizer-based chunking once
`EMBEDDING_REGISTRY` exists for FM-2) must implement the [`Chunker`] trait
and pass the same acceptance criteria as `SlidingWindowChunker`.
