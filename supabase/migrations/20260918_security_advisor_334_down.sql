-- Reversal for #334. Drops the deny policies. Does not disable RLS.
-- View security_invoker is left on (safer default).

BEGIN;

DROP POLICY IF EXISTS child_protections_deny_anon ON public.child_protections;
DROP POLICY IF EXISTS child_protections_deny_authenticated ON public.child_protections;
DROP POLICY IF EXISTS constitution_articles_deny_anon ON public.constitution_articles;
DROP POLICY IF EXISTS constitution_articles_deny_authenticated ON public.constitution_articles;
DROP POLICY IF EXISTS legacy_instruments_deny_anon ON public.legacy_instruments;
DROP POLICY IF EXISTS legacy_instruments_deny_authenticated ON public.legacy_instruments;

COMMIT;
