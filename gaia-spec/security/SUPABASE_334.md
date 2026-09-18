# #334 Supabase advisor remediation

Status: spec + reversible SQL in `supabase/migrations/20260918_security_advisor_334.sql`.
This repository does not apply the migration to a live project.

Intended model:
- `child_protections`, `constitution_articles`, `legacy_instruments`: RLS on, anon deny-all until a reviewed writer role exists.
- Control-plane views: prefer `security_invoker` so they do not run as the view owner.
- Sensitive functions: `search_path = pg_catalog, public`.

After apply: re-run the Supabase security advisor and attach the result to #334.
