# UKD Phase 2 — learner state + TEK gate

Local known/learning/frontier. TEK stays empty without a grant.
Crate already on main: `gaia-ukd::{KnowledgeState, tek_export, teach_me, plan}`.
TEK protocol already listed under #473 / #88. Issue this slice: #549 / #80.

## What exists

`KnowledgeState::local()` — `sync == false`.
`mark_learned` updates local lists.
`tek_export(false)` → `NoAgreement`.
`teach_me` reads that local state. `plan` is a fixture path, not a PubMed connector.

Literature / WikiHow connectors are **not** live.

## Refuse

- public TEK without agreement
- cloud-sync learner dossier
- UKD v1.0
