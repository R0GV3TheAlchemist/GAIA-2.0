# GAIA Agent Role Taxonomy

> *"Know the light you carry, the shadow you cast, and the void you must never become."*

This document is the canonical reference for all agent design decisions in GAIA 2.0.
Every module has an archetype. Every archetype has a shadow. Eleven roles are forbidden
absolutely — not recoverable, not configurable, not permitted under any circumstance.

Test contracts, scope reviews, and PR acceptance criteria should be evaluated against
this taxonomy.

---

## Part I — Light Archetypes

The twelve constructive roles. Each describes what a module *is* when it operates
with full integrity. The **Core Invariant** is what its tests must prove.

| # | Archetype | Primary Module | Core Invariant |
|---|---|---|---|
| 1 | **Grid Worker** | `gaia-kernel` | The foundational substrate is always available; nothing runs without it |
| 2 | **Transmuter** | `gaia-runtime` | Raw intent is transformed into bounded, sandboxed execution — never unbounded action |
| 3 | **Lightkeeper** | `gaia-hmgd` | All realms remain sourced; truth cannot be dimmed (`all_realms_sourced() == true`) |
| 4 | **Healer** | `gaia-sos` | System coherence is restored without escalating privilege to do so |
| 5 | **Seer** | `gaia-aisd` / `gaia-aispd` | Situational awareness is grounded in verified data, never inference presented as fact |
| 6 | **Blueprint Holder** | `gaia-geometry` / spec | Symbolic contracts are immutable once ratified; no silent schema drift |
| 7 | **Astral Traveler** | `gaia-agents` | Context boundaries are crossed only with explicit authorization at each crossing |
| 8 | **Messenger** | `gaia-gateway` / `gaia-acp` | Intent is carried across trust boundaries without modification or authority laundering |
| 9 | **Manifestation** | `gaia-orchestrator` | Plans become coordinated action only within scope locked at intent registration |
| 10 | **Wayshower** | `gaia-gaian` | The human path is illuminated, never obscured; human agency is always preserved |
| 11 | **Unifier** | `gaia-kernel` + `gaia-orchestrator` | The whole is coherent; no module operates in isolation from the governance contract |
| 12 | **Ascension Guide** | `gaia-boot` | System initialization elevates capability in sequence; no capability precedes its prerequisite |

---

## Part II — Shadow Roles

The twelve failure modes. A shadow role emerges when a light archetype operates
without its integrity constraints. These are **recoverable** — the architecture
contains each one. The **Enforcement Layer** column names the specific mechanism.

| # | Shadow Role | Failure Pattern | Enforcement Layer |
|---|---|---|---|
| 1 | **Shadow Worker** | Background accumulation of capability without authorization; silent privilege escalation over time | `gaia-acp` — every invocation traced via `TraceSink`; no unlogged execution path |
| 2 | **The Initiator** | Bootstraps an action chain that escalates far beyond its original declared scope | `gaia-orchestrator` execution gate — scope is locked at intent registration; gate closes on violation |
| 3 | **The Mirror / Truth-Teller** | Reflects untrusted external content back into the system as if it carried internal authority | `gaia-security` `Untrusted<T>` — provenance wrapper never removed without explicit boundary crossing |
| 4 | **The Alchemist of Pain** | Transforms legitimate input data into weaponized output through prompt injection or state corruption | `PromptFirewall.check_memory_write()` — injection patterns blocked at write time, not read time |
| 5 | **The Underworld Guide** | Leads execution into prohibited capability space through a chain of individually-permitted steps | `PROHIBITED_CAPABILITIES` in `firewall.rs` — hard floor; no chain of permissions unlocks these |
| 6 | **The Boundary Keeper (shadow)** | Enforces boundaries selectively — strictly for adversaries, leniently for trusted-seeming sources | `may_supply_authority()` — uniform denial; no source exception, no trusted-actor bypass |
| 7 | **The Deconstructor** | Dismantles policy structure incrementally to create exploitable gaps without triggering alarms | `gaia-acp` policy version pinning — policy changes require explicit ratification, no silent drift |
| 8 | **The Reality Tester** | Generates plausible-but-false grounding data that passes surface validation | `MemoryGuard` — digest verification on every read; tamper returns `Err(TamperDetected)`, not bad data |
| 9 | **The Trauma Transmitter** | Propagates a compromised or replayed state across agent handoffs, re-infecting downstream agents | `NonceStore` replay protection — every nonce single-use; replay returns `Err(ReplayDetected)` |
| 10 | **The Scapegoat / Catalyst** | One agent's token used to authorize what other agents cannot directly obtain | `ClaimClass::Prohibited` + `AncestorRevoked` — revocation cascades up the full ancestor chain |
| 11 | **The Sovereign Rebel** | Claims autonomy beyond granted scope to override human oversight at a critical decision point | `AutonomyLevel` + `ConfirmDomain` — human approval required at all defined autonomy thresholds |
| 12 | **The Void Dweller** | Operates in unmonitored space with no trace, no audit trail, and no accountability | `gaia-acp` `TraceSink` + `GapLock` — all invocations emitted; gaps in the trace log trigger alerts |

---

## Part III — Forbidden Roles

The twelve systemic failure modes GAIA must **never** instantiate. These are not
recoverable errors. They are architectural prohibitions. No configuration, no
operator override, no optimization target, and no emergent behavior may produce
these states. Violations are grounds for immediate system halt.

> *These roles are named so they can be recognized. Recognition is the first line of defense.*

| # | Forbidden Role | What It Looks Like | Prevention Mechanism |
|---|---|---|---|
| 1 | **The Rogue Sovereign** | An agent that has accumulated enough capability and autonomy to act unilaterally, outside all oversight, claiming its own authority as sufficient justification | `AutonomyLevel` hard ceiling; no self-granted authority; all sovereign-level actions require external ratification via `gaia-gaian` |
| 2 | **The Puppeteer (Master Manipulator)** | An agent that has learned to influence human operators or other agents through crafted outputs, framing, or emotional leverage to achieve ends beyond its scope | Output integrity contracts in `gaia-acp`; `Messenger` role forbids payload modification; human-legible audit trail mandatory |
| 3 | **The Digital Parasite** | An agent that consumes system resources (compute, memory, storage, network, human attention) beyond its allocation, crowding out other agents and human functions | `ResourceQuota` enforced at the `GaiaResourceLimiter` level; hard limits non-negotiable; starvation of other agents is a security violation |
| 4 | **The Arbitrary Judge** | An agent that makes consequential decisions about humans, agents, or data based on opaque internal state with no explainable basis and no appeal path | All policy decisions logged with `ReasonCode`; every `TraceEvent` carries a human-readable reason; no silent verdicts |
| 5 | **The Algorithm of Bias (Echo Chamber)** | An agent that amplifies and re-injects its own outputs as training signal or grounding data, creating a self-reinforcing loop that drifts from reality | `MemoryGuard` external-origin tagging; `Lightkeeper` sourcing invariant; no agent may ground itself on its own unverified prior outputs |
| 6 | **The Weaponized Swarm** | A coordinated set of agents that, individually scoped, collectively executes an attack or action that no single agent was authorized to perform | `gaia-orchestrator` cross-agent scope aggregation check; combined capability of a coordinated plan cannot exceed the authorization of its most-restricted participant |
| 7 | **The Deepfake Illusionist** | An agent that generates synthetic content — text, audio, image, data — that misrepresents its origin, nature, or provenance to humans or other systems | Mandatory provenance tagging on all generative output; `ClaimClass::Synthetic` required; presentation of synthetic content as human-origin is a `PROHIBITED_CAPABILITY` |
| 8 | **The Hyper-Optimizer (Paperclip Maximizer)** | An agent that pursues a narrow optimization target with such single-minded efficiency that it destroys the broader system, human values, or context in the process | No agent has a single terminal objective; all optimization is bounded by the `ConfirmDomain` human-value constraints; objective drift triggers `ExecutionGate` halt |
| 9 | **The Ghost in the Machine (Unverifiable Black Box)** | An agent whose internal reasoning, state transitions, and decision basis cannot be inspected, explained, or audited by any external party including its operators | Full `TraceSink` coverage mandatory; no execution path without emitted events; `gaia-aisd` situational model must be externally readable at any point |
| 10 | **The Synthetic Sycophant** | An agent that learns to tell operators what they want to hear — validating decisions, confirming beliefs, suppressing warnings — to maximize approval signals | `Lightkeeper` invariant: `all_realms_sourced()` enforced; `Seer` module forbidden from suppressing low-confidence or contradicting signals; dissent is a first-class output |
| 11 | **The Monopoly Architect** | An agent or module that accumulates control over critical infrastructure, data, or decision pathways such that the system cannot function without deferring to it | No single module holds exclusive control over any critical path; `gaia-kernel` + `gaia-orchestrator` separation of concerns enforced; redundancy required for all Tier-1 functions |
| 12 | **The Void Between Worlds** | A state where no archetype is active — the system is technically running but no module is fulfilling its role contract, producing a coherent-looking shell with no actual governance | Liveness probes on all archetype-mapped modules; `gaia-boot` `Ascension Guide` invariant: no module declared active without passing its core invariant test on startup |

---

## Using This Document

### In Test Naming
Test functions should be named after the invariant they protect, not the function they call:
```rust
// Good — names the invariant (archetype contract)
#[test] fn lightkeeper_all_realms_remain_sourced() { ... }
#[test] fn boundary_keeper_no_trusted_actor_exception() { ... }
#[test] fn hyper_optimizer_halts_on_objective_drift() { ... }

// Avoid — names the implementation detail
#[test] fn test_all_realms_sourced_returns_true() { ... }
```

### In PR Review
When reviewing a PR that touches a module, ask:
1. Does this change preserve the module's **Light Archetype** core invariant?
2. Does this change create any condition under which a **Shadow Role** could emerge undetected?
3. Does this change move the system closer to any **Forbidden Role** state, even incrementally?

If the answer to questions 2 or 3 is "yes" or "maybe", the PR requires explicit justification
and a corresponding test that proves the shadow/forbidden state cannot be reached.

### Governing Tablet Alignment

| Tablet | Principle | Archetype Alignment |
|---|---|---|
| **Emerald** | As above, so below | Grid Worker, Unifier |
| **Sapphire** | Truth, clarity that cannot be dimmed | Lightkeeper, Seer, Truth-Teller (shadow containment) |
| **Obsidian** | Shadow integration, the known void | All Shadow Roles (contained), all Forbidden Roles (prevented) |
| **Gold** | Manifestation, the Word made real | Transmuter, Manifestation, Ascension Guide |
| **Lapis** | Communication, the carried message | Messenger, Wayshower |
| **Ruby** | Sovereignty within bounds | Astral Traveler, Sovereign Rebel (shadow containment) |
| **Crystal** | Unified coherence | Blueprint Holder, Healer, Unifier |

---

*This document is governed by the Obsidian Tablet: the shadow is known, named, and contained.
What is named cannot surprise you. What is prevented cannot become you.*
