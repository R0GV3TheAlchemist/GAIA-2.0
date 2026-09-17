-- GAIA 2.0 CT-002: auditable scheduled memory maintenance
-- Live migration: gaia20_scheduled_memory_maintenance
-- Proof: proofs/PROOF-GAIA20-MEMORY-003.md

CREATE EXTENSION IF NOT EXISTS pg_cron;

CREATE TABLE IF NOT EXISTS public.maintenance_runs (
  run_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  maintenance_name text NOT NULL,
  started_at timestamptz NOT NULL DEFAULT now(),
  completed_at timestamptz,
  status text NOT NULL CHECK (status IN ('running', 'succeeded', 'failed')),
  transition_counts jsonb NOT NULL DEFAULT '{}'::jsonb,
  error_message text,
  triggered_by text NOT NULL DEFAULT 'manual'
);

CREATE INDEX IF NOT EXISTS idx_maintenance_runs_name_started
  ON public.maintenance_runs (maintenance_name, started_at DESC);

ALTER TABLE public.maintenance_runs ENABLE ROW LEVEL SECURITY;

CREATE POLICY maintenance_runs_read ON public.maintenance_runs
  FOR SELECT TO authenticated USING (true);

CREATE OR REPLACE FUNCTION public.run_memory_tier_maintenance(run_at timestamptz DEFAULT now())
RETURNS uuid
LANGUAGE plpgsql
SECURITY INVOKER
SET search_path = public
AS $$
DECLARE
  current_run_id uuid := gen_random_uuid();
  counts jsonb := '{}'::jsonb;
BEGIN
  INSERT INTO public.maintenance_runs (
    run_id, maintenance_name, started_at, status, triggered_by
  ) VALUES (
    current_run_id, 'memory_tier_maintenance', run_at, 'running',
    COALESCE(current_setting('application_name', true), 'manual')
  );

  SELECT COALESCE(jsonb_object_agg(transition, changed_count), '{}'::jsonb)
  INTO counts
  FROM public.maintain_memory_tiers(run_at);

  UPDATE public.maintenance_runs
  SET completed_at = now(),
      status = 'succeeded',
      transition_counts = counts
  WHERE run_id = current_run_id;

  RETURN current_run_id;
EXCEPTION WHEN OTHERS THEN
  UPDATE public.maintenance_runs
  SET completed_at = now(),
      status = 'failed',
      error_message = SQLERRM
  WHERE run_id = current_run_id;
  RAISE;
END;
$$;

REVOKE ALL ON FUNCTION public.run_memory_tier_maintenance(timestamptz) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION public.run_memory_tier_maintenance(timestamptz) TO authenticated;

DO $$
DECLARE existing_job_id bigint;
BEGIN
  SELECT jobid INTO existing_job_id
  FROM cron.job
  WHERE jobname = 'gaia-memory-tier-maintenance-daily';

  IF existing_job_id IS NOT NULL THEN
    PERFORM cron.unschedule(existing_job_id);
  END IF;

  PERFORM cron.schedule(
    'gaia-memory-tier-maintenance-daily',
    '0 5 * * *',
    'SELECT public.run_memory_tier_maintenance();'
  );
END;
$$;
