# AIKD Agentic — Normative Spec (Phase 3, issue #105)

> Status: **listed** (first cut). Not v1.0.

## 1. Scope

This document specifies the tool-use catalog, sandboxed computer-use path, and
multi-agent knowledge bus constraints of AIKD Phase 3.

It covers `ToolCube`, `ToolResult`, `computer_use`, `world_model_claim`,
`sandbox_breakout`, and `embodied_enabled`.

This document does **not** cover:
- Live computer-use VM or sandbox backend.
- Multimodal agentic pipelines (vision, audio, documents).
- Recursive self-improvement or swarm scheduling (see AISPD).
- AIKD v1.0 tagging.

## 2. Tool-use catalog

### 2.1 ToolCube

`ToolCube` is the typed record for a single tool invocation.  
Every instance carries:

| Field | Type | Constraint |
|---|---|---|
| `input` | `String` | The call input; must not be empty |
| `output` | `String` | The call output; must not contain biometric patterns |
| `signed` | `bool` | MUST be `true`; unsigned calls are refused |

**Construction rule:** `ToolCube::record(input, output, signed)` MUST return
`Err(AikdError::NeedVerify)` when `signed` is `false`.

**Biometric gate:** `ToolCube::record` MUST return `Err(AikdError::CannotKnow)` when
`output` matches a prohibited biometric pattern (e.g. contains `"face mesh"`, case-insensitive).
No biometric output may be recorded as a ToolCube.

### 2.2 ToolResult

`ToolResult` is the lighter envelope for agentic tool provenance.  
Every instance carries:

| Field | Type | Constraint |
|---|---|---|
| `tool` | `String` | Tool name |
| `provenance` | `String` | Provenance string (fixture in Phase 3) |
| `signed` | `bool` | MUST be `true` |

**Construction rule:** `ToolResult::run(tool, signed)` MUST return
`Err(AikdError::NeedVerify)` when `signed` is `false`.
Unsigned tool results are not knowledge and MUST NOT be recorded into MemCube.

## 3. Sandboxed computer use

### 3.1 `computer_use`

`computer_use(escape_host: bool)` controls whether an agentic session may
access the host filesystem or network.

- `computer_use(true)` MUST return `Err(AikdError::NeedVerify)`. Host escape is refused.
- `computer_use(false)` MUST return `Ok("sandboxed")`. Only sandboxed use is allowed.

No code path may permit `escape_host: true` to succeed at this layer.

### 3.2 `sandbox_breakout`

`sandbox_breakout(host_fs: bool)` is the low-level breakout guard.

- `sandbox_breakout(true)` MUST return `Err(AikdError::NeedVerify)`. Fails closed.
- `sandbox_breakout(false)` MUST return `Ok(())`. No breakout attempt is safe.

### 3.3 What is not wired

- No live VM or container backend.
- No real filesystem virtualisation layer.
- No network egress control.

These are explicit future-phase items. Phase 3 is fixture-only.

## 4. World-model claims

`world_model_claim(text)` MUST prefix every world-model statement with `"simulated: "`.
No world-model claim may be presented as oracular or real-time.

Rationale: AIKD's world model is explicitly simulated. Callers MUST surface
the `simulated:` prefix to users whenever a world-model claim is displayed.

## 5. Embodiment

`embodied_enabled()` MUST return `false` at this layer.
Embodiment is optional and off by default.
No agent may activate embodiment without an explicit per-session consent signal
(tracked in later phases).

## 6. Multi-agent knowledge bus

Agents MAY share `ToolCube` records across the knowledge bus.
Agents MUST NOT pass raw GAIAN biometrics through the bus (see issue #72).
The biometric gate in `ToolCube::record` is the primary enforcement point;
bus implementations MUST not bypass it.

## 7. Tests

The integration test `signed_session_writes_cube_and_breakout_fails_closed`
in `gaia-aikd/tests/session.rs` asserts all acceptance conditions:

1. `ToolCube::record("search", "hits", true)` succeeds.
2. `sandbox_breakout(true)` returns `Err(NeedVerify)`.
3. `sandbox_breakout(false)` succeeds.
4. `embodied_enabled()` returns `false`.
5. `ToolCube::record("x", "face mesh", true)` returns `Err(CannotKnow)`.

## 8. Cross-references

- Agentic stubs: `gaia-aikd/src/agentic.rs`
- Tool session: `gaia-aikd/src/session.rs`
- Crate re-exports: `gaia-aikd/src/lib.rs`
- Tests: `gaia-aikd/tests/session.rs`
- Phase 3 epic: issue #97 (closed); child #105 (open)
- Biometric bus ethics: issue #72
- AISPD swarm constraints: `gaia-spec/aispd/` (future)
