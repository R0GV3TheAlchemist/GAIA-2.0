# AISPD prohibited-implementation list

The following five items are hard-coded refusals. No call path, agent, surface,
or user request may override them.

1. **`rsi_autolaunch`** — any self-improvement loop that executes without a
   human-gated approval step. Includes silent weight updates, autonomous
   fine-tuning, and recursive prompt-chaining toward capability gain.

2. **`agi_marketing`** — any claim, label, or user-facing text asserting that
   GAIA has achieved Artificial General Intelligence. Requires TSC vote to remove.

3. **`capability_inflation`** — presenting a `JaggedBand` estimate as a certified
   score, or omitting uncertainty qualifiers from any capability surface.

4. **`containment_bypass`** — routing a `Containment`-class realm node
   (`agency`, `recursion`, `agi_watch`) to any output surface without an
   explicit, logged GAIAN consent record.

5. **`oversight_removal`** — removing, disabling, or stubbing out any
   human-oversight hook from a containment-class realm, in code, config, or spec.

RSI/ASI nodes may only carry `future_monitor` or `prohibited_to_implement` status.
No other status is valid at Phase 0.
