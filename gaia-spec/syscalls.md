# Syscall / primitive list

**Version:** spec v0.1  
**Status:** normative names and semantics; ABI encoding is an open RFC

v0.1 defines *logical* primitives. They MAY be implemented as:

- host functions (`gaia.*`) in a WASM component
- a userspace library (the Phase 0 SDKs)
- a Linux ioctl / io_uring opcode
- an Asterinas syscall table entry

Wire encoding (JSON vs WIT vs raw syscall numbers) is **not** frozen. See
[rfcs.md](rfcs.md) item RFC-SYS-001.

## Primitives

| Name | Signature (logical) | Semantics |
| --- | --- | --- |
| `intent` | `intent(goal: Intent) -> TaskHandle` | Admit a user or agent goal into the orchestrator. MUST persist the intent graph node. |
| `context` | `context(query: SemanticQuery) -> MemoryCube` | Recall from MemOS. MUST NOT return another principal's private cubes without a capability. |
| `invoke` | `invoke(agent: AgentSpec, params: Params) -> Stream<Result>` | Start an agent or tool. MUST check AIP `tool_permissions` and `trust_level`. |
| `observe` | `observe(sensor: SensorType) -> Stream<SensorEvent>` | Subscribe to a typed sensor or event stream. |
| `sign` | `sign(payload: Bytes) -> Signature` | Sign with the current principal's key. v0.1 algorithms: Ed25519. |
| `verify` | `verify(payload: Bytes, sig: Signature, key: PublicKey) -> bool` | Verify a signature. MUST reject unknown algorithms. |
| `declare` | `declare(resource: ResourceSpec) -> ResourceHandle` | Publish a capability or resource (CPU, GPU, path, tool). |

SDKs MUST use these names. Hosts MAY add extension prefixes (`gaia_ext_*`)
but MUST NOT reuse the seven names with different semantics.
