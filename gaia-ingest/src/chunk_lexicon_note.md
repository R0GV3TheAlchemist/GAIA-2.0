# DocumentChunk — Lexicon Plane Integration Note

**Status:** DONE — merged in `feat/document-chunk-lexicon-plane`.

`lexicon_plane: LexiconPlane` and `lexicon_voice: Option<LexiconVoice>` are
now fields on `DocumentChunk` in `document.rs`.

`pub mod lexicon` and all four public items (`classify_chunk`, `LexiconPlane`,
`LexiconSignals`, `LexiconVoice`) are re-exported from `lib.rs`.

## Remaining follow-ons

1. ~~DocumentChunk type integration~~ ✅ this PR
2. **`document_chunks` Supabase migration** — add `lexicon_plane` and
   `lexicon_voice` columns to the chunks table (separate PR).
3. **RAG retrieval guard** — `LexiconPlaneMismatch` error type + middleware
   check in `gaia-memos` (separate PR).

## Canon refs
- C156 — KG + memory taxonomy
- C30  — no silent failures
