-- GAIA 2.0 security remediation
-- Aligns repository migration history with live Supabase migration
-- `memory_retention_metric` must execute with invoker permissions so RLS
-- on public.memories is applied for every caller.

DROP VIEW IF EXISTS public.memory_retention_metric;

CREATE VIEW public.memory_retention_metric
WITH (security_invoker = true)
AS
SELECT
  count(*) FILTER (
    WHERE tier IN ('HOT','WARM')
      AND relevance_score >= 0.50
      AND created_at >= now() - interval '30 days'
  )::double precision
  / NULLIF(
    count(*) FILTER (
      WHERE tier IN ('HOT','WARM')
        AND created_at >= now() - interval '30 days'
    ),
    0
  ) AS retention_rate_30d
FROM public.memories;