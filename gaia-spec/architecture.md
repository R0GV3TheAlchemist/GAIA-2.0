# Architecture overview (L0–L6)

**Version:** spec v0.1  
**Status:** normative intent, informative deployment

GAIA 2.0 is a Super Operating System: a meta-layer that MUST run *with* a
host OS (Linux, a framekernel, or an embedded kernel). It MUST NOT be
specified as a mandatory Linux replacement in v0.1.

```
L6  Sovereign interface     voice, text, vision, haptic, AR/VR, BCI
L5  Agent ecosystem         agents, marketplace, A2A
L4  Cognitive orchestration intent engine, planner, coordinator, broker
L3  MemOS                   MemCube + five-tier memory
L2  Semantic File System    POSIX + vector + provenance
L1  GAIA kernel path        framekernel *or* Linux module + syscalls
L0  Hardware continuum      IoT → edge → mobile → desktop → cloud → HPC
```

## Normative requirements (v0.1)

1. Implementations MUST expose the primitives in [syscalls.md](syscalls.md),
   even if they are userspace stubs.
2. Implementations MUST identify principals per [identity.md](identity.md).
3. Agents that are packaged for distribution MUST carry an AIP Manifest
   that validates against [aip-manifest.md](aip-manifest.md).
4. Memory objects exchanged across process or node boundaries MUST be
   representable as MemCubes per [memcube.md](memcube.md).
5. User-level goals MUST be representable as an Intent Graph per
   [intent-graph.md](intent-graph.md).
6. Implementations SHOULD prefer pull-based executors on L0 (no required
   inbound port).
7. Implementations MUST treat host choice (Asterinas fork vs Linux module)
   as an implementation detail. The syscall contract is what is portable.

## Informative deployment (not required to claim v0.1 spec compliance)

- Executor daemon in Rust/Tokio
- SFS via FUSE
- MemOS tiers backed by process memory, a KV store, a vector index, a graph, and model weights
- WASM 3.0 + WASI agent runtime
- MCP as the tool bus

These land in Phases 1–3. Spec compliance in Phase 0 means: schemas + SDK types exist.
