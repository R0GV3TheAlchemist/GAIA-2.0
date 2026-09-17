# GAIA 2.0 Build Sequence

**Filed:** 2026-09-17  
**Rule:** finish the backbone that already exists before opening new universes.

## Already true

1. Public monorepo `GAIA-2.0` with crates, proofs, CI, supabase/, huggingface/.
2. Supabase project `gaia-2-0` healthy in us-east-2 with 10 RLS tables and 6 migrations.
3. Hugging Face account exists; Hub is empty.
4. Research corpus covers cognition, memory, consent, flourishing, education, HRV, particle geometry, and AlScN/GaN.

## Sequence

### Slice A — Records and law (this pack)

- Commit these records under `docs/academic/` in `GAIA-2.0`.
- Add front-matter to each: `status: records`, `requires: []`, `tags: [academic, github, supabase, huggingface]`.
- Link STACK.md to this pack instead of duplicating it.

### Slice B — Memory truth

- Keep HOT/WARM/COLD as storage temperature.
- Add `cognitive_type`, `gaian_id`, `provenance_source`, `confidence` to `memories`.
- Implement Metric 6 as a `security_invoker` view (already started in a fix migration).
- Export anonymized `memory_access_events` only with RESEARCH_USE consent.

### Slice C — Consent as proof, not a log

- Migrate `consent_events` toward HMAC chain + fourteen scopes.
- Block Execution paths when Safety/Consent circuit is OPEN (agent runtime already has incident types).
- Do not claim GDPR erasure until key destruction receipts exist.

### Slice D — Lithic completeness

- Finish RRUFF/IMA ingest into `elements` / `minerals` / `crystals`.
- Validate JSON against schemas in CI.
- Publish `gaia-elements` on Hugging Face only after row count is a real periodic table, not 3 demo rows.

### Slice E — Hub

- Create org `gaia-os`.
- Push `huggingface/gaia-elements` via Hub sync.
- Bind `proofs` rows to dataset SHAs.

### Slice F — Studies stay in the lab

- GFI instrument tables may be built now (items, not people).
- Coherence and Elemental Learning pipelines stay protocol-only in this project.
- No identifiable minor data. No clinical marketing copy.

### Slice G — GAIAN design issues #213–#221

Only after A–C. Identity file, Telos, child rules, and GAIAN-to-GAIAN envelopes inherit consent and memory tiers; they must not invent a second store.

## Hard no

- No silent deletion of personal memory.
- No Hub upload of raw session logs.
- No “this treats depression / replaces IEP / replaces GDP” language.
- No canon merge without Proof.
- No service-role keys in client apps.
- No training on user memory without RESEARCH_USE grant.

## Success for “no problems anymore”

Problems in this project have been: canon without proof, docs without a live store, stores without RLS, and claims without datasets. The stack now has a public repo, a healthy database with RLS, and an empty Hub waiting for honest cards.

“No problems anymore” means every claim has a row in `proofs`, every personal row has a consent scope, and every public artefact has a card that states what it is not.
