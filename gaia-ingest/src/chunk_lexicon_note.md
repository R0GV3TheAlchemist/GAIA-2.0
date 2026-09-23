# DocumentChunk — Lexicon Plane Integration Note

**Status:** Pending — follow-on PR after `feat/lexicon-order-chaos` merges.

## What needs to change in `chunk.rs`

The `DocumentChunk` struct needs two new fields so every emitted chunk carries
its plane classification through the full ingestion pipeline:

```rust
use crate::lexicon::{LexiconPlane, LexiconVoice};

pub struct DocumentChunk {
    // ... existing fields ...

    /// Which ontological plane this chunk's vocabulary belongs to.
    /// Default: `LexiconPlane::Bridge` (safe; unknown provenance).
    /// Promoted to Order or Chaos by the pipeline classify step.
    pub lexicon_plane: LexiconPlane,

    /// Who speaks the vocabulary in this chunk.
    /// `None` until resolved by the pipeline classify step.
    pub lexicon_voice: Option<LexiconVoice>,
}
```

## Where to wire it

1. `DocumentChunk::default()` — set `lexicon_plane: LexiconPlane::Bridge`, `lexicon_voice: None`.
2. `Chunker::emit_chunk()` — pass through from `LexiconSignals` already resolved at
   document ingestion time (author type, source registry, language code, domain, sacred flag).
3. `gaia-ingest` pipeline step (new) — call `lexicon::classify_chunk(&signals)` and
   set the fields before the chunk is embedded and stored.
4. Supabase `document_chunks` table (new migration, separate PR) — add
   `lexicon_plane lexicon_plane NOT NULL DEFAULT 'Bridge'` and
   `lexicon_voice lexicon_voice` columns.

## RAG retrieval guard

The retrieval layer must check `lexicon_plane` before cross-plane lookup:

```rust
// Never silently serve an Order definition in response to a Chaos query.
if query_plane == LexiconPlane::Chaos && chunk.lexicon_plane == LexiconPlane::Order {
    // Require explicit Bridge crossing — do not auto-promote.
    return Err(LexiconPlaneMismatch { query: query_plane, chunk: chunk.lexicon_plane });
}
```

## Test to add

```rust
#[test]
fn chunk_default_plane_is_bridge() {
    let chunk = DocumentChunk::default();
    assert_eq!(chunk.lexicon_plane, LexiconPlane::Bridge);
    assert_eq!(chunk.lexicon_voice, None);
}
```

## Canon refs

- C156 — KG + memory taxonomy
- C30  — no silent failures
- `gaia-aikd/src/card.rs` `cannot_know()` — sacred TEK gate
