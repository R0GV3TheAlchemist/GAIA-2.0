-- =============================================================================
-- Migration : 20260923_gaia20_document_chunks_lexicon_plane
-- Purpose   : Add lexicon_plane and lexicon_voice columns to document_chunks.
--
-- Follow-on 2 of 3 from PR #913 (feat/lexicon-order-chaos) and
-- PR #914 (feat/document-chunk-lexicon-plane).
--
-- The Rust DocumentChunk type now carries:
--   lexicon_plane : LexiconPlane   -- Order | Chaos | Bridge
--   lexicon_voice : Option<LexiconVoice>  -- Sacred | Scholarly | … | None
--
-- This migration aligns the database schema with those fields.
--
-- Idempotent: safe to re-run; all mutations are guarded with
-- IF NOT EXISTS / IF EXISTS / WHERE IS NULL / DO NOTHING.
--
-- Canon refs: C156 (KG + memory taxonomy), C30 (no silent failures)
-- =============================================================================

BEGIN;

-- ---------------------------------------------------------------------------
-- 1. lexicon_plane column
--    NOT NULL with a safe default of 'Bridge' so every existing row and
--    every new row emitted before the classify step runs is valid.
-- ---------------------------------------------------------------------------
ALTER TABLE document_chunks
  ADD COLUMN IF NOT EXISTS lexicon_plane TEXT NOT NULL DEFAULT 'Bridge';

-- Enforce the closed vocabulary from LexiconPlane.
-- Drop first (idempotent) then re-add so the constraint body is authoritative.
ALTER TABLE document_chunks
  DROP CONSTRAINT IF EXISTS document_chunks_lexicon_plane_check;

ALTER TABLE document_chunks
  ADD CONSTRAINT document_chunks_lexicon_plane_check
  CHECK (lexicon_plane IN ('Order', 'Chaos', 'Bridge'));

-- ---------------------------------------------------------------------------
-- 2. lexicon_voice column
--    Nullable — None until the pipeline classify step resolves provenance.
-- ---------------------------------------------------------------------------
ALTER TABLE document_chunks
  ADD COLUMN IF NOT EXISTS lexicon_voice TEXT DEFAULT NULL;

ALTER TABLE document_chunks
  DROP CONSTRAINT IF EXISTS document_chunks_lexicon_voice_check;

ALTER TABLE document_chunks
  ADD CONSTRAINT document_chunks_lexicon_voice_check
  CHECK (
    lexicon_voice IS NULL OR
    lexicon_voice IN (
      'Sacred',
      'Scholarly',
      'Narrative',
      'Technical',
      'Colloquial',
      'Synthetic'
    )
  );

-- ---------------------------------------------------------------------------
-- 3. Backfill existing rows
--    Rows inserted before this migration will have the column default
--    ('Bridge' / NULL) applied by Postgres, so this UPDATE is a no-op on
--    fresh tables but is included for safety on live tables that may have
--    received rows via a concurrent write path.
-- ---------------------------------------------------------------------------
UPDATE document_chunks
  SET   lexicon_plane = 'Bridge'
WHERE lexicon_plane IS NULL;   -- can only be NULL if default was not applied

-- lexicon_voice backfill: NULL is already correct; nothing to do.

-- ---------------------------------------------------------------------------
-- 4. Indexes
--
--    idx_document_chunks_lexicon_plane
--      Partial index on non-Bridge rows — the retrieval guard (follow-on 3)
--      needs to quickly identify Order/Chaos chunks during query matching.
--      Bridge rows (the majority before classify runs) are excluded to keep
--      the index small.
--
--    idx_document_chunks_uri_plane
--      Composite index for per-document plane breakdown queries used by the
--      admin dashboard and the classify worker's progress scan.
-- ---------------------------------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_document_chunks_lexicon_plane
  ON document_chunks (lexicon_plane)
  WHERE lexicon_plane <> 'Bridge';

CREATE INDEX IF NOT EXISTS idx_document_chunks_uri_plane
  ON document_chunks (document_uri, lexicon_plane);

-- ---------------------------------------------------------------------------
-- 5. Comment the columns for Supabase Studio / pg_dump readability
-- ---------------------------------------------------------------------------
COMMENT ON COLUMN document_chunks.lexicon_plane IS
  'LexiconPlane classification: Order | Chaos | Bridge (default). '
  'Set by the pipeline classify step after ingestion (gaia-ingest). '
  'Bridge = unresolved provenance; safe default per C30.';

COMMENT ON COLUMN document_chunks.lexicon_voice IS
  'LexiconVoice register: Sacred | Scholarly | Narrative | Technical | '
  'Colloquial | Synthetic. NULL until classify step resolves provenance.';

COMMIT;
