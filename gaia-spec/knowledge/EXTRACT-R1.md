# Extract R#1.1–R#1.15 — refuse list + local contract

Source: `Documents-2/GAIA 2.0 Gap Research Report R#1.1–R#1.15.md` (Blueprint 63, 2026-09-09).
Issue: #405 under META #404.

This file is the piece-together of the *beginning* of Documents-2. It is not a kernel. It is not a grant.

## Per-gap bind

| Gap | Report asked | Bind | Class |
| --- | --- | --- | --- |
| R#1.1 Kernel | Asterinas + AI syscalls + io_uring batch | research note only; Asterinas is **not a crate** | experimental |
| R#1.2 SFS | Qdrant / Weaviate / Lance bake-off | vendor bake-off is not a merge; Earth Twin search stays local contract | experimental |
| R#1.3 Memory | Letta / MemOS / Mem0 numbers | local trace sink; #219 memory-tier still design | experimental |
| R#1.4 Agents | LangGraph DAG internally | DAG already in orchestrator tests; LangGraph is not a dependency | experimental |
| R#1.5 Intent | Intent ABI / say it all | `gaia intent` local plan + accept; #220 still open | experimental |
| R#1.6 Sovereignty | network identity / DID product | refuse until local pause works | prohibited-as-product |
| R#1.7 DID / keys | cross-net identity | same; Ed25519 inventory only (#389); dual-sign #392 design-only | prohibited-as-product |
| R#1.8 Marketplace | economics as grant path | refuse | prohibited-as-product |
| R#1.9 Continuum | federated edge/fog/cloud | listed later node policy; no federated cloud this slice | experimental |
| R#1.10 Security | deep dive | control plane deny/kill/pause; #341 stays open | experimental |
| R#1.11 TLA+ | TLA-Prover runtime | formal spec allowed as a *file*; no prover runtime | experimental |
| R#1.12 Scheduling | carbon-aware MILP / federated nodes | listed node policy; no fleet scheduler product | experimental |
| R#1.13 Governance stress | live planetary council | refuse | prohibited |
| R#1.14 Protocol | MCP 97M + A2A | MCP is zero-trust **unsigned refuse** (#341 open). No live MCP. A2A not a product | prohibited-as-product |
| R#1.15 (remainder) | any unlisted “must adopt vendor” | default refuse until a local contract + test (#378) | experimental |

## Already in the plane

- Intent ABI → #220
- MCP unsigned refuse → #341
- Memory-tier design → #219
- Trace / gap-lock → #335

## Missing local contracts (not new doctrine)

- Kernel batching numbers as a bench file, not a syscall product.
- Search backend as an interface name, not a vendor crate.
- Memory tier as a local sink contract (#219).
- Intent accept/refuse receipt (#220).

## Refuse (do not promote from R#1)

- Live MCP because “97M downloads”.
- Marketplace economics (R#1.8) as a grant path.
- Sovereignty / DID (R#1.6–R#1.7) as a network identity product before local pause works.
- Governance stress (R#1.13) as a live planetary council.
- Custom Asterinas / AgenticOS syscall product.
- LangGraph, Letta, Weaviate, Qdrant, Lance as merges.
- TLA-Prover as a runtime.
- Carbon federated cloud scheduler as this slice.

## Next piece

R#1 is the validation of *assumptions*. Sibling extracts: #406–#411, index #412. Four before five still holds.
