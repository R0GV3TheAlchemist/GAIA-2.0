# AIKD execute verify — fixture

Passing tests can be T1. Failed runs stay T4. Not a sandbox product.
Crate already on main: `gaia-aikd::{Executed, Adapter, Tier}`.
Issue this slice: #571 / #103. Phase 2 already named the same adapters.

## What exists

`Executed::code(true, None)` → `Tier::T1`, no error.
`Executed::code(false, Some("boom"))` → `Tier::T4` with that error.
`Executed::proof(false)` → T4 `unverified proof`. Never T1 without verify.
`Adapter::Math` / `Code` `eval_slice` are fixture names. Not a runner.

## Refuse

- live symbolic/numeric sandbox
- live unit-test runner
- unverified proof as Tier 1
