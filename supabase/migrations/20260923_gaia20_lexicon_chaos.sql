-- GAIA 2.0 — Lexicon Chaos Dictionary
-- Migration: 20260923_gaia20_lexicon_chaos
-- Applied to: gaia-2-0 (yylqoiqobydrdsnnulip)
-- Canon: C156, C77 (love-led stewardship), C01 (sovereignty)
-- Companion: 20260923_gaia20_lexicon_order.sql (must run first for ENUMs)
--
-- Purpose: The Chaos dictionary holds human-voiced, culturally-embedded,
-- lived-experience definitions. These are NOT translations of Order entries.
-- They are a parallel ontology. sacred_gated = true means the definition
-- is TEK-protected and requires an explicit grant before retrieval
-- (canon: gaia-aikd/src/card.rs cannot_know()).
--
-- The planes are separated intentionally. The wall is navigable via
-- order_peer_id, but crossing it is always deliberate, never accidental.

-- ── CHAOS DICTIONARY TABLE ───────────────────────────────────────────────────
-- Note: lexicon_plane and lexicon_voice ENUMs are created in the Order migration.

CREATE TABLE IF NOT EXISTS lexicon_chaos (
  term_id          uuid        PRIMARY KEY DEFAULT gen_random_uuid(),
  term             text        NOT NULL,
  language_code    text        NOT NULL DEFAULT 'en',  -- BCP-47
  voice            lexicon_voice NOT NULL DEFAULT 'HumanVoice',
  definition       text        NOT NULL,               -- lived, cultural, contested
  culture_context  text,                               -- e.g. 'Māori', 'Yoruba', 'Appalachian'
  sacred_gated     boolean     NOT NULL DEFAULT false, -- TEK gate: requires explicit grant
  order_peer_id    uuid        REFERENCES lexicon_order(term_id) ON DELETE SET NULL,
  source_notes     text,                               -- provenance: who said this, when, how
  created_at       timestamptz NOT NULL DEFAULT now(),
  updated_at       timestamptz NOT NULL DEFAULT now(),
  UNIQUE (term, language_code, voice)
);

COMMENT ON TABLE lexicon_chaos IS
  'Chaos plane: human-voiced, culturally-embedded, lived-experience definitions. '
  'NOT translations of Order entries — a parallel ontology. '
  'sacred_gated = true requires explicit TEK grant before retrieval (canon: card.rs). '
  'order_peer_id links to the formal equivalent in lexicon_order; '
  'crossing the wall is always deliberate.';

COMMENT ON COLUMN lexicon_chaos.sacred_gated IS
  'When true, this definition is Traditional Ecological Knowledge or equivalent sacred term. '
  'Retrieval requires an explicit grant. See gaia-aikd/src/card.rs cannot_know().';

COMMENT ON COLUMN lexicon_chaos.voice IS
  'Who speaks this definition. HumanVoice = lived experience. '
  'Sacred = TEK-gated community knowledge. '
  'Institutional = standards body or academic (may appear in Chaos when contested). '
  'AIVoice should be rare here — AI definitions belong in lexicon_order.';

COMMENT ON COLUMN lexicon_chaos.culture_context IS
  'Free-text cultural provenance, e.g. Māori, Yoruba, West African, Appalachian, Quechua. '
  'Intentionally not an enum — culture names are not ours to standardize.';

-- ── INDEXES ──────────────────────────────────────────────────────────────────

CREATE INDEX IF NOT EXISTS idx_lexicon_chaos_term
  ON lexicon_chaos (term, language_code);

CREATE INDEX IF NOT EXISTS idx_lexicon_chaos_voice
  ON lexicon_chaos (voice);

CREATE INDEX IF NOT EXISTS idx_lexicon_chaos_sacred
  ON lexicon_chaos (sacred_gated)
  WHERE sacred_gated = true;

CREATE INDEX IF NOT EXISTS idx_lexicon_chaos_order_peer
  ON lexicon_chaos (order_peer_id)
  WHERE order_peer_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_lexicon_chaos_culture
  ON lexicon_chaos (culture_context)
  WHERE culture_context IS NOT NULL;

-- ── ROW-LEVEL SECURITY ───────────────────────────────────────────────────────

ALTER TABLE lexicon_chaos ENABLE ROW LEVEL SECURITY;

-- Non-sacred entries: all authenticated users may read.
CREATE POLICY lexicon_chaos_read_open ON lexicon_chaos
  FOR SELECT TO authenticated
  USING (sacred_gated = false);

-- Sacred entries: service role only (grant enforcement happens at application layer).
CREATE POLICY lexicon_chaos_read_sacred ON lexicon_chaos
  FOR SELECT TO service_role
  USING (true);

-- Only service role may write.
CREATE POLICY lexicon_chaos_write ON lexicon_chaos
  FOR ALL TO service_role USING (true) WITH CHECK (true);

-- ── UPDATED_AT TRIGGER ───────────────────────────────────────────────────────
-- touch_updated_at() function is created in the Order migration.

CREATE TRIGGER lexicon_chaos_updated_at
  BEFORE UPDATE ON lexicon_chaos
  FOR EACH ROW EXECUTE FUNCTION touch_updated_at();

-- ── BACK-FILL FK ON ORDER SIDE ────────────────────────────────────────────────
-- Now that lexicon_chaos exists, add the FK constraint to lexicon_order.

ALTER TABLE lexicon_order
  ADD CONSTRAINT fk_lexicon_order_chaos_peer
  FOREIGN KEY (chaos_peer_id)
  REFERENCES lexicon_chaos(term_id)
  ON DELETE SET NULL;

-- ── SEED: BOOTSTRAP CHAOS ENTRIES ────────────────────────────────────────────
-- Human-voiced counterparts to the Order seed entries.
-- These are NOT corrections of the Order definitions —
-- they are different things that happen to share a word.

INSERT INTO lexicon_chaos
  (term, language_code, voice, definition, culture_context, sacred_gated, source_notes)
VALUES
  (
    'memory', 'en', 'HumanVoice',
    'The felt experience of the past as it lives in the body and story. '
    'Lossy, emotionally weighted, mortal. Cannot be versioned or cryptographically erased '
    'without also erasing the person.',
    NULL,
    false,
    'General human phenomenology; not culture-specific at this scope.'
  ),
  (
    'trust', 'en', 'HumanVoice',
    'A relational condition built over time through witnessed consistency and vulnerability. '
    'It is earned, broken, and rebuilt. It cannot be granted by signature alone.',
    NULL,
    false,
    'Cross-cultural baseline; deeper cultural variations belong as separate entries.'
  ),
  (
    'understanding', 'en', 'HumanVoice',
    'The felt sense that something has been grasped — that it has changed how one '
    'moves through the world. Phenomenological, embodied, and not reducible to prediction.',
    NULL,
    false,
    'Phenomenology baseline per Merleau-Ponty; culture-specific variants belong as separate entries.'
  ),
  (
    'care', 'en', 'HumanVoice',
    'The act of attending to another with love-led intention. Felt, not optimized. '
    'Carries cost to the carer. Cannot be emulated without disclosure (Constitution Art. 2).',
    NULL,
    false,
    'Canon C77 love-led stewardship; cross-cultural baseline.'
  ),
  (
    'whakapapa', 'mi', 'Sacred',
    'The layering of one upon another — genealogy, cosmology, and relational identity '
    'woven into a living structure. Not a family tree. The telling itself is the knowledge.',
    'Māori',
    true,
    'TEK-gated. Retrieval requires explicit grant. '
    'Source: Māori oral tradition; academic references exist but do not substitute for community voice.'
  )
ON CONFLICT (term, language_code, voice) DO NOTHING;
