# AIKD Phase 3 — agentic stubs

Signed tool results. Sandbox cannot take the host.
Crate already on main: `gaia-aikd::{ToolResult, computer_use, world_model_claim, sandbox_breakout, embodied_enabled}`.
Issue this slice: #541 / #97. Child #105 stays open.

## What exists

`ToolResult::run(_, false)` → `NeedVerify`.
`computer_use(true)` → `NeedVerify`; false → `sandboxed`.
`world_model_claim` prefixes `simulated:`.
`sandbox_breakout(true)` → `NeedVerify`.
`embodied_enabled()` → false.

No live computer-use product. No host FS.

## Refuse

- unbounded host control
- unsigned tool as knowledge
- embodied on by default
