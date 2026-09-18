-- #334 reversible spec. Not applied from this repo to a live project.
-- Default deny on RLS tables that currently have no policy.
-- Recheck advisor after applying in the gaia-2-0 project.

BEGIN;

ALTER TABLE IF EXISTS public.child_protections ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS public.constitution_articles ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS public.legacy_instruments ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS child_protections_deny_anon ON public.child_protections;
CREATE POLICY child_protections_deny_anon ON public.child_protections
  FOR ALL TO anon USING (false) WITH CHECK (false);

DROP POLICY IF EXISTS constitution_articles_deny_anon ON public.constitution_articles;
CREATE POLICY constitution_articles_deny_anon ON public.constitution_articles
  FOR ALL TO anon USING (false) WITH CHECK (false);

DROP POLICY IF EXISTS legacy_instruments_deny_anon ON public.legacy_instruments;
CREATE POLICY legacy_instruments_deny_anon ON public.legacy_instruments
  FOR ALL TO anon USING (false) WITH CHECK (false);

-- Prefer invoker on control-plane views when Postgres supports it.
DO $$ BEGIN
  EXECUTE 'ALTER VIEW public.v_chaos_regime SET (security_invoker = true)';
EXCEPTION WHEN others THEN NULL;
END $$;
DO $$ BEGIN
  EXECUTE 'ALTER VIEW public.v_work_avalanche SET (security_invoker = true)';
EXCEPTION WHEN others THEN NULL;
END $$;
DO $$ BEGIN
  EXECUTE 'ALTER VIEW public.v_execution_gate SET (security_invoker = true)';
EXCEPTION WHEN others THEN NULL;
END $$;

-- Explicit search_path on named functions if they exist.
DO $$ BEGIN
  EXECUTE 'ALTER FUNCTION public.detect_system_gaps() SET search_path = pg_catalog, public';
EXCEPTION WHEN others THEN NULL;
END $$;
DO $$ BEGIN
  EXECUTE 'ALTER FUNCTION public.acquire_gap_lock() SET search_path = pg_catalog, public';
EXCEPTION WHEN others THEN NULL;
END $$;
DO $$ BEGIN
  EXECUTE 'ALTER FUNCTION public.execution_blocked() SET search_path = pg_catalog, public';
EXCEPTION WHEN others THEN NULL;
END $$;
DO $$ BEGIN
  EXECUTE 'ALTER FUNCTION public.prevent_work_item_avalanche() SET search_path = pg_catalog, public';
EXCEPTION WHEN others THEN NULL;
END $$;

COMMIT;
