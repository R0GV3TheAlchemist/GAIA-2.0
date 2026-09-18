-- #334 advisor remediation for project gaia-2-0 (yylqoiqobydrdsnnulip).
-- Apply in the Supabase SQL editor. This repo cannot reach the live project.
-- Intended access: service_role / postgres only. anon and authenticated deny-all.
-- child_protections / constitution_articles / legacy_instruments are governance
-- tables, not client-writable.

BEGIN;

ALTER TABLE IF EXISTS public.child_protections ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS public.constitution_articles ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS public.legacy_instruments ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS child_protections_deny_anon ON public.child_protections;
DROP POLICY IF EXISTS child_protections_deny_authenticated ON public.child_protections;
CREATE POLICY child_protections_deny_anon ON public.child_protections
  FOR ALL TO anon USING (false) WITH CHECK (false);
CREATE POLICY child_protections_deny_authenticated ON public.child_protections
  FOR ALL TO authenticated USING (false) WITH CHECK (false);

DROP POLICY IF EXISTS constitution_articles_deny_anon ON public.constitution_articles;
DROP POLICY IF EXISTS constitution_articles_deny_authenticated ON public.constitution_articles;
CREATE POLICY constitution_articles_deny_anon ON public.constitution_articles
  FOR ALL TO anon USING (false) WITH CHECK (false);
CREATE POLICY constitution_articles_deny_authenticated ON public.constitution_articles
  FOR ALL TO authenticated USING (false) WITH CHECK (false);

DROP POLICY IF EXISTS legacy_instruments_deny_anon ON public.legacy_instruments;
DROP POLICY IF EXISTS legacy_instruments_deny_authenticated ON public.legacy_instruments;
CREATE POLICY legacy_instruments_deny_anon ON public.legacy_instruments
  FOR ALL TO anon USING (false) WITH CHECK (false);
CREATE POLICY legacy_instruments_deny_authenticated ON public.legacy_instruments
  FOR ALL TO authenticated USING (false) WITH CHECK (false);

-- Views: run as invoker so underlying RLS applies.
DO $$ BEGIN
  EXECUTE 'ALTER VIEW public.v_chaos_regime SET (security_invoker = true)';
EXCEPTION WHEN undefined_object THEN NULL;
END $$;
DO $$ BEGIN
  EXECUTE 'ALTER VIEW public.v_work_avalanche SET (security_invoker = true)';
EXCEPTION WHEN undefined_object THEN NULL;
END $$;
DO $$ BEGIN
  EXECUTE 'ALTER VIEW public.v_execution_gate SET (security_invoker = true)';
EXCEPTION WHEN undefined_object THEN NULL;
END $$;

-- Pin search_path on named public functions regardless of signature.
DO $$
DECLARE
  r record;
BEGIN
  FOR r IN
    SELECT p.oid::regprocedure AS sig
    FROM pg_proc p
    JOIN pg_namespace n ON n.oid = p.pronamespace
    WHERE n.nspname = 'public'
      AND p.proname IN (
        'detect_system_gaps',
        'acquire_gap_lock',
        'execution_blocked',
        'prevent_work_item_avalanche',
        'classify_branching_ratio',
        'classify_qrc_phase',
        'compute_overall_phi',
        'refuse_posthumous_without_consent',
        'reject_engagement_maximizing_prompt'
      )
  LOOP
    EXECUTE format('ALTER FUNCTION %s SET search_path = pg_catalog, public', r.sig);
  END LOOP;
END $$;

COMMIT;
