# LLM application frameworks — listed (#860)

**Points at:** `gaia-kernel`, `gaia-runtime`, `gaia-orchestrator`.

Named field: LangChain, LlamaIndex, Haystack, DSPy, Semantic Kernel, LangGraph.

| External pattern | GAIA surface today |
| --- | --- |
| Chains / graphs | `gaia-orchestrator` crate exists; not LangGraph |
| Grounded generation | `gaia-runtime` `enforce_grounding` |
| Kernel loop | `gaia-kernel` listed, not a replacement for DSPy |

Worth a later RFC only if a gap is measured. None opened here.

## Refuse

- Do not vendor LangChain. Do not add a graph runtime.
