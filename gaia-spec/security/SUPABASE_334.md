# #334 Supabase advisor remediation

Project: `gaia-2-0` (`yylqoiqobydrdsnnulip`).
This repository cannot apply SQL to the live project. Apply `supabase/migrations/20260918_security_advisor_334.sql` in the Supabase SQL editor, then re-run Database Linter / Security Advisor.

## Intended readers/writers

| Table | anon | authenticated | service_role / postgres |
| --- | --- | --- | --- |
| child_protections | deny | deny | bypass RLS |
| constitution_articles | deny | deny | bypass RLS |
| legacy_instruments | deny | deny | bypass RLS |

These are governance instruments (child agency cap, constitution text, posthumous consent). They are not client APIs.

## Views

`v_chaos_regime`, `v_work_avalanche`, `v_execution_gate` set `security_invoker = true` so they do not run as the view owner.

## Functions

Named anti-chaos and constitution functions get `search_path = pg_catalog, public` by oid lookup (signature-safe).

## After apply

1. SQL editor: run the up migration.
2. Dashboard: Reports → Security Advisor / Database Linter.
3. Paste the new finding counts on issue #334.
4. If needed, run `20260918_security_advisor_334_down.sql`.
