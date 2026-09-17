-- GAIA 2.0 CT-002: memory lifecycle maintenance
-- Live migration: gaia20_memory_tier_maintenance
-- Proof: proofs/PROOF-GAIA20-MEMORY-002.md

CREATE OR REPLACE FUNCTION public.maintain_memory_tiers(run_at timestamptz DEFAULT now())
RETURNS TABLE (
  transition text,
  changed_count bigint
)
LANGUAGE plpgsql
SECURITY INVOKER
SET search_path = public
AS $$
BEGIN
  RETURN QUERY
  WITH access_counts AS (
    SELECT
      m.memory_id,
      count(e.id) FILTER (WHERE e.accessed_at >= run_at - interval '7 days') AS accesses_7d
    FROM public.memories m
    LEFT JOIN public.memory_access_events e ON e.memory_id = m.memory_id
    GROUP BY m.memory_id
  ), classified AS (
    SELECT
      m.memory_id,
      m.tier AS old_tier,
      CASE
        WHEN m.tier = 'COLD' AND a.accesses_7d >= 1 THEN 'WARM'::memory_tier
        WHEN m.tier = 'WARM' AND a.accesses_7d >= 5 THEN 'HOT'::memory_tier
        WHEN m.tier = 'HOT' AND COALESCE(m.last_accessed, m.created_at) < run_at - interval '14 days' THEN 'WARM'::memory_tier
        WHEN m.tier = 'WARM' AND COALESCE(m.last_accessed, m.created_at) < run_at - interval '14 days' THEN 'COLD'::memory_tier
        ELSE m.tier
      END AS new_tier
    FROM public.memories m
    JOIN access_counts a ON a.memory_id = m.memory_id
  ), updated AS (
    UPDATE public.memories m
    SET tier = c.new_tier,
        decay_rate = CASE c.new_tier
          WHEN 'HOT' THEN 0.01
          WHEN 'WARM' THEN 0.03
          WHEN 'COLD' THEN 0.07
        END,
        tier_upgraded_at = CASE
          WHEN c.new_tier IN ('WARM','HOT') AND c.new_tier <> c.old_tier THEN run_at
          ELSE m.tier_upgraded_at
        END,
        updated_at = run_at
    FROM classified c
    WHERE m.memory_id = c.memory_id
      AND c.new_tier <> c.old_tier
    RETURNING c.old_tier::text || '→' || c.new_tier::text AS transition
  )
  SELECT u.transition, count(*)::bigint
  FROM updated u
  GROUP BY u.transition
  ORDER BY u.transition;
END;
$$;

REVOKE ALL ON FUNCTION public.maintain_memory_tiers(timestamptz) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION public.maintain_memory_tiers(timestamptz) TO authenticated;
