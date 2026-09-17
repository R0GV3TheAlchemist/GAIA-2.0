-- Slice B: memory truth. HOT/WARM/COLD remains storage temperature.
-- cognitive_type is the MemoryHierarchy / Research 002 class.
-- Applied on gaia-2-0 as migration gaia20_memory_truth_slice_b.

DO $$
BEGIN
  CREATE TYPE public.cognitive_type AS ENUM (
    'working',
    'short_term',
    'episodic',
    'semantic',
    'procedural',
    'long_term'
  );
EXCEPTION
  WHEN duplicate_object THEN NULL;
END $$;

ALTER TABLE public.memories
  ADD COLUMN IF NOT EXISTS cognitive_type public.cognitive_type NOT NULL DEFAULT 'episodic',
  ADD COLUMN IF NOT EXISTS gaian_id text,
  ADD COLUMN IF NOT EXISTS provenance_source text NOT NULL DEFAULT 'unspecified',
  ADD COLUMN IF NOT EXISTS confidence double precision NOT NULL DEFAULT 0.5;

ALTER TABLE public.memories
  DROP CONSTRAINT IF EXISTS memories_confidence_range;

ALTER TABLE public.memories
  ADD CONSTRAINT memories_confidence_range
  CHECK (confidence >= 0::double precision AND confidence <= 1::double precision);

COMMENT ON COLUMN public.memories.tier IS 'Storage temperature: HOT, WARM, or COLD. Not the cognitive class.';
COMMENT ON COLUMN public.memories.cognitive_type IS 'Cognitive class: working, short_term, episodic, semantic, procedural, long_term.';
COMMENT ON COLUMN public.memories.gaian_id IS 'Optional GAIAN identity key. RLS still keys off owner_id.';
COMMENT ON COLUMN public.memories.provenance_source IS 'Origin of the memory. Metric 10 counts rows where this is not unspecified.';
COMMENT ON COLUMN public.memories.confidence IS 'Provenance confidence in [0, 1].';

CREATE INDEX IF NOT EXISTS memories_gaian_id_idx ON public.memories (gaian_id);
CREATE INDEX IF NOT EXISTS memories_cognitive_type_idx ON public.memories (cognitive_type);
CREATE INDEX IF NOT EXISTS memories_tier_created_at_idx ON public.memories (tier, created_at);

CREATE OR REPLACE VIEW public.memory_metric_6_retention_30d
WITH (security_invoker = true) AS
SELECT
  now() AS observed_at,
  count(*) FILTER (WHERE created_at <= now() - interval '30 days') AS cohort_30d,
  count(*) FILTER (
    WHERE created_at <= now() - interval '30 days'
      AND tier IN ('HOT', 'WARM')
  ) AS retained_hot_warm,
  count(*) FILTER (
    WHERE created_at <= now() - interval '30 days'
      AND tier = 'COLD'
  ) AS cold_excluded_as_forgotten,
  CASE
    WHEN count(*) FILTER (WHERE created_at <= now() - interval '30 days') = 0 THEN NULL
    ELSE (
      count(*) FILTER (
        WHERE created_at <= now() - interval '30 days'
          AND tier IN ('HOT', 'WARM')
      )::double precision
      / count(*) FILTER (WHERE created_at <= now() - interval '30 days')::double precision
    )
  END AS retention_ratio,
  0.85::double precision AS target_ratio,
  CASE
    WHEN count(*) FILTER (WHERE created_at <= now() - interval '30 days') = 0 THEN NULL
    ELSE (
      count(*) FILTER (
        WHERE created_at <= now() - interval '30 days'
          AND tier IN ('HOT', 'WARM')
      )::double precision
      / count(*) FILTER (WHERE created_at <= now() - interval '30 days')::double precision
    ) >= 0.85::double precision
  END AS metric_met
FROM public.memories;

COMMENT ON VIEW public.memory_metric_6_retention_30d IS
  'Metric 6: 30-day retention among memories aged >= 30 days. HOT+WARM count as retained. COLD counts as not retained (Metric 8 forgetting). security_invoker so RLS of the caller applies.';

GRANT SELECT ON public.memory_metric_6_retention_30d TO authenticated, service_role;
