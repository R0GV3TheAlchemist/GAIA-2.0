# SOS Design Phase 2 — EACN + ColonyOS Mapping and Service Registries

**Status:** Listed  
**Issues:** #193 (epic, closed), #198 (this slice)  
**Crate:** `gaia-sos` / `gaia-agent-bus` (Apache-2.0)  
**Not:** A central always-on god coordinator. Not a push-based control plane. Not a CP-only system.

`god_coordinator_required()` MUST return `false`.  
`eacn_push_model()` MUST return `false` — EACN is pull-only.  
`registry_is_centralised()` MUST return `false`.

---

## 1. Purpose

SOS Phase 2 specifies the EACN discovery layer, the ColonyOS executor
mapping, the four service registries, broker/reconcile/channel alignment
with `gaia-agent-bus`, and the AP-vs-CP split-brain policy. No central
always-on coordinator is required.

---

## 2. EACN Discovery (`eacn.rs`)

EACN (Earth Agent Coordination Network) is the Science Earth peer-discovery
fabric. GAIA nodes participate as pull consumers — they query EACN; EACN
does not push to them.

`eacn_push_model()` MUST return `false`.

### Discovery flow

```
Intent
  │
  │  1. parse_intent()
  ▼
IntentResolved
  │
  │  2. eacn_discover(capability)
  ▼
CapabilityManifest []
  │
  │  3. negotiate_ownership(manifests)
  ▼
OwnedCapability
  │
  │  4. colonyos_dispatch(capability, capsule)
  ▼
ExecutionCapsule
  │
  │  5. await_result() / collect_result()
  ▼
Result
```

| Step | API | Notes |
|---|---|---|
| Discover | `eacn_discover(cap: &str) -> Vec<CapabilityManifest>` | Returns empty vec if none found; never blocks indefinitely |
| Negotiate | `negotiate_ownership(manifests: Vec<CapabilityManifest>) -> OwnedCapability` | Picks highest-trust manifest; logs evidence-standard clash if >1 |
| Dispatch | `colonyos_dispatch(cap: OwnedCapability, capsule: ExecCapsule) -> ExecHandle` | Fire-and-collect; no blocking RPC |
| Collect | `collect_result(handle: ExecHandle, timeout_ms: u32) -> Result<ExecResult, SosError>` | `Err(Timeout)` after `timeout_ms`; caller retries or escalates |

### Evidence-standard clash

When `negotiate_ownership` encounters manifests with conflicting evidence
standards (e.g., one requires peer review, one accepts grey literature), it
MUST log a `EvidenceClash` event and select the strictest standard.

`evidence_clash_silenced()` MUST return `false`.

---

## 3. ColonyOS Executor Mapping (`colony.rs`)

ColonyOS executors are remote compute capsule runners. GAIA maps
`ExecCapsule` payloads to ColonyOS job formats and collects results.

| Rule | Constraint |
|---|---|
| Pull model | GAIA pulls job results; ColonyOS does not push callbacks to GAIA kernel |
| No kernel boundary | ColonyOS dispatch MUST run in `gaia-agent-bus` userspace; no kernel call |
| Capsule integrity | `ExecCapsule` MUST be signed with the GAIAN Ed25519 key before dispatch |
| Timeout mandatory | `collect_result` MUST enforce `timeout_ms`; zero timeout is `Err(InvalidTimeout)` |
| No god coordinator | `god_coordinator_required()` MUST return `false` — no single node orchestrates all capsules |

### `ExecCapsule` schema

```
capsule_id:    String   — unique; signed
gaian_id:      String   — issuing GAIAN
capability:    String   — EACN capability string
payload:       Bytes    — WASM or JSON task body
signature:     Bytes    — Ed25519 over (capsule_id + gaian_id + capability + payload)
created_at:    String   — ISO 8601 UTC
```

---

## 4. Broker / Reconcile / Channels vs `gaia-agent-bus` (#105 / #131)

| Concept | EACN/ColonyOS term | `gaia-agent-bus` equivalent |
|---|---|---|
| Broker | `EacnBroker` — capability directory | `AgentBus::registry` |
| Reconcile | `negotiate_ownership` | `AgentBus::resolve_conflict` |
| Channel | `ExecHandle` + `collect_result` | `AgentBus::channel` (async) |

The `gaia-agent-bus` (#105/#131) is the local in-process bus. EACN/ColonyOS
are the external federation layer. They MUST NOT be merged into one type.
`EacnBroker` MUST delegate local-only capabilities to `AgentBus` without
passing through the EACN network.

`local_capability_goes_to_eacn()` MUST return `false`.

---

## 5. Service Registries

Four registries are defined. Each is a local cache of remote discovery
results; none is a central authoritative server.

`registry_is_centralised()` MUST return `false` for all four.

| Registry | Type | Contents |
|---|---|---|
| **GAIAN registry** | Local identity store | GAIAN ids, public keys, capability declarations |
| **Earth Twin connector** | Connector cache | Earth Twin node addresses, supported capability strings |
| **Knowledge registry** | KG source cache | Graphiti / knowledge-graph endpoint metadata, evidence standards |
| **Infra twins registry** | Infrastructure cache | NPU, compute, and storage twin addresses; HAL-tier mappings |

### Registry rules

| Rule | Constraint |
|---|---|
| TTL required | All registry entries MUST carry a `ttl_seconds` field; expired entries MUST be evicted |
| No central write | No registry MUST require a central coordinator to accept writes |
| Local-first | Registry lookups MUST return local cache before network query |
| Stale-on-partition | On network partition (AP mode), registries serve stale data rather than blocking |

See `gaia-spec/sos/eacn.csv` for the EACN capability fixture.

---

## 6. Partition (AP) vs Revoke (CP) Policy

GAIA SOS Phase 2 uses AP (availability + partition-tolerance) for discovery
and CP (consistency + partition-tolerance) only for capability revocation.

| Operation | Model | Behaviour on partition |
|---|---|---|
| Discovery (`eacn_discover`) | AP | Returns stale manifest; logs `PartitionStaleDiscovery` |
| Registry lookup | AP | Returns cached entry; logs `PartitionStaleRegistry` |
| Capability revocation | CP | Blocks until quorum; returns `Err(RevocationPending)` if quorum unavailable |
| Capsule dispatch | AP | Dispatches with stale manifest; result validated on collection |
| Evidence-clash resolution | CP | Requires acknowledgement from both parties; `Err(ClashUnresolved)` on partition |

`revocation_is_ap()` MUST return `false` — revocation is always CP.

---

## 7. Prohibition Surface

| Prohibition | Error / Return |
|---|---|
| God coordinator required | `god_coordinator_required()` → `false` |
| EACN push model | `eacn_push_model()` → `false` |
| Local capability to EACN | `local_capability_goes_to_eacn()` → `false` |
| Centralised registry write | `registry_is_centralised()` → `false` |
| Evidence clash silenced | `evidence_clash_silenced()` → `false` |
| Revocation on AP | `revocation_is_ap()` → `false` |
| Unsigned capsule dispatch | `Err(UnsignedCapsule)` |
| Zero-timeout collect | `Err(InvalidTimeout)` |

---

## 8. What This Phase Does Not Do

- Does not require a central always-on god coordinator.
- Does not use a push-based EACN control plane.
- Does not merge `EacnBroker` and `AgentBus` into one type.
- Does not serve local capabilities over the EACN network.
- Does not make capability revocation available-partition-tolerant.
- Does not allow unsigned `ExecCapsule` dispatch.

---

## 9. Acceptance Gate

- [ ] `god_coordinator_required()` → `false`
- [ ] `eacn_push_model()` → `false`
- [ ] `registry_is_centralised()` → `false`
- [ ] `local_capability_goes_to_eacn()` → `false`
- [ ] `evidence_clash_silenced()` → `false`
- [ ] `revocation_is_ap()` → `false`
- [ ] `collect_result(handle, 0)` → `Err(InvalidTimeout)`
- [ ] Unsigned capsule dispatch → `Err(UnsignedCapsule)`
- [ ] `eacn_discover` returns empty vec (not error) when no capability found
- [ ] `cargo test -p gaia-sos` green

---

## 10. Cross-References

- SOS phase-2 stub: `gaia-spec/sos/phase-2.md`
- EACN fixture: `gaia-spec/sos/eacn.csv`
- Agent bus: `gaia-agent-bus` (#105, #131)
- Infra twins: `gaia-spec/system-twins.md`
- HAL tiers: `gaia-spec/sos/hal-tiers.md`
- SOS Phase 1: `gaia-spec/sos/SOS-PHASE-1.md` (#197)
- Issues: #193 (epic), #198 (this slice)
