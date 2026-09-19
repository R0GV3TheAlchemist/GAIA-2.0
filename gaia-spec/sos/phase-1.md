# SOS Design Phase 1 — listed contracts

Specify AI-kernel *extensions* without pretending they are already in Asterinas.
Asterinas remains a research citation, not a crate.
Crate already on main: `gaia-sos::{sfs_v1, kernel_inference, submit_intent, live_slurm}`.
Issue this slice: #467 / #192. Parent META #190 stays open.

## Userspace first

Specs are implementable against userspace. Kernel-space inference is optional TCB growth:

- `sfs_v1()` → `"fuse"` (deployment *label*, not a FUSE product in this tree)
- `kernel_inference()` → `"optional-tcb-risk"`
- `live_slurm()` → `false`

AGENTS.md: do not add FUSE, Qdrant, or Tantivy product stacks in this slice.
Do not invent cube, lake, or bleach APIs that are not exported on main.

## Neural scheduler (names only)

CFS + VRAM / NPU / carbon are **hooks to specify**, not a running scheduler.
No live carbon actuator. No live NPU driver.

## MemOS 5-tier (names only)

Listed in `memos-tiers.csv`. Aligned with #18 as a *name map*, not a live cube store.

## SFS

Semantic layer on POSIX. FUSE-first means the v1 *target* is userspace. Not in-kernel v1.

## Intent parser

Structured path already exists: `submit_intent(signed, cap)` plus `gaia-spec/sos/intent.schema.json`.
Natural-language → Intent is **not** implemented. Do not add an NL parser here.

## Refuse

- second kernel / Asterinas crate / SOS v1.0
- in-kernel inference as default
- live slurm / FUSE daemon / MemOS product
