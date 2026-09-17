-- GAIA 2.0 CT-002: atomic memory access, relevance, and promotion runtime
-- Live migration: gaia20_memory_access_runtime
-- Proof: proofs/PROOF-GAIA20-MEMORY-001.md

CREATE TABLE IF NOT EXISTS public.memory_access_events (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  memory_id uuid NOT NULL REFERENCES public.memories(memory_id) ON DELETE CASCADE,
  actor_id uuid,
  accessed_at timestamptz NOT NULL DEFAULT now(),
  tier_before memory_tier NOT NULL,
  tier_after memory_tier NOT NULL,
  relevance_before double precision NOT NULL,
  relevance_after double precision NOT NULL,
  recency_weight double precision NOT NULL CHECK (recency_weight IN (0.2, 0.5, 1.0))
);

CREATE INDEX IF NOT EXISTS idx_memory_access_events_memory_time
  ON public.memory_access_events (memory_id, accessed_at DESC);

ALTER TABLE public.memory_access_events ENABLE ROW LEVEL SECURITY;

CREATE POLICY memory_access_events_owner_read ON public.memory_access_events
  FOR SELECT TO authenticated
  USING (
    EXISTS (
      SELECT 1 FROM public.memories m
      WHERE m.memory_id = memory_access_events.memory_id
        AND (m.owner_id IS NULL OR m.owner_id = auth.uid())
    )
  );

CREATE OR REPLACE FUNCTION public.access_memory(target_memory_id uuid)
RETURNS TABLE (
  memory_id uuid,
  tier memory_tier,
  access_count integer,
  relevance_score double precision,
  decay_rate double precision,
  last_accessed timestamptz
)
LANGUAGE plpgsql
SECURITY INVOKER
SET search_path = public
AS $$
DECLARE
  m public.memories%ROWTYPE;
  previous_access timestamptz;
  weight double precision;
  new_score double precision;
  new_tier memory_tier;
  new_decay double precision;
  accesses_last_7d integer;
  now_ts timestamptz := now();
BEGIN
  SELECT * INTO m
  FROM public.memories
  WHERE memories.memory_id = target_memory_id
  FOR UPDATE;

  IF NOT FOUND THEN
    RAISE EXCEPTION 'memory % not found', target_memory_id USING ERRCODE = 'P0002';
  END IF;

  IF m.owner_id IS NOT NULL AND m.owner_id <> auth.uid() THEN
    RAISE EXCEPTION 'not authorized to access memory %', target_memory_id USING ERRCODE = '42501';
  END IF;

  previous_access := m.last_accessed;
  weight := CASE
    WHEN previous_access IS NULL OR now_ts - previous_access <= interval '24 hours' THEN 1.0
    WHEN now_ts - previous_access <= interval '7 days' THEN 0.5
    ELSE 0.2
  END;

  new_score := LEAST(1.0, m.relevance_score + 0.05 * weight);

  SELECT count(*) + 1 INTO accesses_last_7d
  FROM public.memory_access_events e
  WHERE e.memory_id = target_memory_id
    AND e.accessed_at >= now_ts - interval '7 days';

  new_tier := m.tier;
  IF m.tier = 'COLD' AND accesses_last_7d >= 1 THEN
    new_tier := 'WARM';
  ELSIF m.tier = 'WARM' AND accesses_last_7d >= 5 THEN
    new_tier := 'HOT';
  END IF;

  new_decay := CASE new_tier
    WHEN 'HOT' THEN 0.01
    WHEN 'WARM' THEN 0.03
    WHEN 'COLD' THEN 0.07
  END;

  UPDATE public.memories
  SET access_count = m.access_count + 1,
      last_accessed = now_ts,
      relevance_score = new_score,
      tier = new_tier,
      decay_rate = new_decay,
      tier_upgraded_at = CASE WHEN new_tier <> m.tier THEN now_ts ELSE m.tier_upgraded_at END,
      updated_at = now_ts
  WHERE memories.memory_id = target_memory_id;

  INSERT INTO public.memory_access_events (
    memory_id, actor_id, accessed_at, tier_before, tier_after,
    relevance_before, relevance_after, recency_weight
  ) VALUES (
    target_memory_id, auth.uid(), now_ts, m.tier, new_tier,
    m.relevance_score, new_score, weight
  );

  RETURN QUERY
  SELECT target_memory_id, new_tier, m.access_count + 1, new_score, new_decay, now_ts;
END;
$$;

REVOKE ALL ON FUNCTION public.access_memory(uuid) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION public.access_memory(uuid) TO authenticated;
