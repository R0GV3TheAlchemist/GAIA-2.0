# RAG Chunking Strategy — Size Constants & Rationale

**Spec version:** 1.0.0  
**Status:** Canonical  
**Closes:** #931

---

## Overview

This document defines the size constants and strategy rationale for
structure-preserving document chunking in `gaia-ingest`.  All chunking
logic lives in `gaia-ingest/src/chunking.rs`.

---

## Size Constants

| Constant          | Value        | Unit       | Rationale |
|-------------------|--------------|------------|-----------|
| `MIN_CHUNK_CHARS` | 256          | characters | Below this a chunk lacks enough context to be retrievable on its own.  Merging with the following sibling preserves semantic continuity. |
| `MAX_CHUNK_CHARS` | 2 048        | characters | Keeps chunks well within typical embedding model token windows (512–1 024 tokens at ~3–4 chars/token). Code blocks and tables are exempt — they are never split. |
| `OVERLAP_CHARS`   | 128          | characters | One to two sentences of trailing context prepended to the next chunk prevents hard retrieval boundaries at split points. |

### Token approximation

At an average of 4 characters per token (conservative for English prose):

- `MIN_CHUNK_CHARS` 256 ≈ 64 tokens
- `MAX_CHUNK_CHARS` 2 048 ≈ 512 tokens
- `OVERLAP_CHARS` 128 ≈ 32 tokens

These match the token-level constants described in the issue specification
(MIN 64 tokens, MAX 512 tokens, overlap 32 tokens).

---

## Chunking Strategies

### `MarkdownHeading` (default)

Split on Markdown heading boundaries (`#`, `##`, `###`).  Each heading
section becomes at minimum one chunk.  The heading text is prepended as
a context prefix on every child chunk so that retrieval queries can match
the section topic even when the body text alone is ambiguous.

**Atomic units** (never split regardless of size):
- Fenced code blocks (` ``` ` … ` ``` `)  
- Markdown tables (contiguous lines beginning with `|`)

**Heading breadcrumb** (`heading_path`): each chunk carries the ordered
list of ancestor heading titles so the retrieval layer can filter by
document section.

### `FixedSizeWithOverlap`

Sliding window of configurable `size` and `overlap` (both in characters).
Recommended for plain-text sources that carry no Markdown structure.

### `Sentence` (stub)

Sentence-boundary splitting using a future NLP tokeniser.  Not yet
implemented — stub present in the enum for forward compatibility.

---

## Failure Modes Addressed

| FM   | Description                        | How chunking addresses it |
|------|------------------------------------|---------------------------|
| FM-1 | Fragmented chunks                  | `MIN_CHUNK_CHARS` merge prevents orphan slivers |
| FM-12| Ignoring document structure        | `MarkdownHeading` strategy + `heading_path` breadcrumb |
