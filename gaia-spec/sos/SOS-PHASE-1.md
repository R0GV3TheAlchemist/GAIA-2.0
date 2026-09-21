# SOS Design Phase 1 — Neural Scheduler, MemCube Ops, SFS-as-FUSE, Intent Parser

**Status:** Listed  
**Issues:** #192 (epic, closed), #197 (this slice)  
**Crate:** `gaia-sos` (Apache-2.0)  
**Not:** A kernel-space ONNX runtime (v1). Not a live NPU driver. Not a cloud scheduler.

`kernel_inference_enabled()` MUST return `false` until a TCB review passes and is filed in `rfcs/sos-kernel-inference-tcb.md`.  
`sfs_v1_is_fuse()` MUST return `true`.  
`npu_slot_preempts_mid_inference()` MUST return `false`.

---

## 1. Purpose

SOS Phase 1 specifies the three L3-memos scheduler extension hooks
(no-preempt-mid-inference, NPU slot reservation, carbon windows), the five
MemCube ops that align with the #18 MemOS tier model, the SFS v1 FUSE
interface over a Qdrant/Graphiti userspace backend, and the intent-parser
finite-state machine with ambiguity handling.

---

## 2. Neural Scheduler Hooks (`scheduler.rs`)

### 2.1 No-preempt-mid-inference guard

`scheduler_hook_no_preempt(task_id: TaskId) -> SchedulerHook`

| Rule | Constraint |
|---|---|
| No mid-inference preemption | Inference tasks marked `inference = true` MUST NOT be preempted once started; yielding is caller-initiated only |
| `npu_slot_preempts_mid_inference()` | MUST return `false` |
| Preempt-safe window | Scheduler MAY preempt only at `inference_yield_point()` boundaries defined by the task |

### 2.2 NPU slot reservation

`scheduler_hook_npu_slot(npu_id: NpuId, duration_ms: u32) -> Result<NpuSlot, SchedulerError>`

| Rule | Constraint |
|---|---|
| Slot reservation | Returns `Ok(NpuSlot)` when the NPU is available for the requested duration |
| Slot conflict | Returns `Err(NpuSlotBusy)` when the slot is taken; caller MUST back off |
| No kernel ONNX | `kernel_inference_enabled()` MUST return `false` — ONNX runs in userspace only in v1 |
| TCB gate | Kernel inference MUST NOT be enabled without a TCB review filed in `rfcs/sos-kernel-inference-tcb.md` |

### 2.3 Carbon windows

`scheduler_hook_carbon_window(window: CarbonWindow) -> SchedulerHook`

| Rule | Constraint |
|---|---|
| Deferral only | Carbon windows MAY defer a task; MUST NOT drop it |
| No silent discard | `carbon_window_drops_task()` MUST return `false` |
| Window source | `CarbonWindow` is provided by the host environment; scheduler does not fetch grid data itself |

### Scheduler hook CSV reference

See `gaia-spec/sos/sched-hooks.csv` for the full hook enumeration.

---

## 3. MemCube Ops (`memcube.rs`) — L3-memos, aligns #18

MemCube is the L3 memory tier. All ops address `MemCubeId` entries and MUST be
ABI-compatible with the `#18` MemOS tier model.

| Op | Signature | Semantics |
|---|---|---|
| `recall` | `recall(id: MemCubeId) -> Result<MemEntry, MemError>` | Fetch entry from L3; `Err(NotFound)` if absent |
| `consolidate` | `consolidate(ids: &[MemCubeId]) -> Result<MemEntry, MemError>` | Merge multiple entries; deduplicate by `entry_key` |
| `migrate` | `migrate(id: MemCubeId, target_tier: MemTier) -> Result<(), MemError>` | Move entry to `target_tier`; MUST NOT duplicate |
| `fuse` | `fuse(a: MemCubeId, b: MemCubeId) -> Result<MemCubeId, MemError>` | Create a new fused entry from two; sources remain until explicit evict |
| `evict` | `evict(id: MemCubeId) -> Result<(), MemError>` | Remove entry from L3; `Err(NotFound)` if already absent |

### MemCube rules

| Rule | Constraint |
|---|---|
| No cross-user bleed | `recall` MUST NOT return entries belonging to a different `gaian_id` |
| Migrate is atomic | `migrate` MUST be atomic — if it fails, the entry stays at the source tier |
| Fuse preserves sources | `fuse` MUST NOT evict source entries; caller evicts explicitly |
| Tier alignment | All ops MUST align with `memos-tiers.csv` tier definitions |

See `gaia-spec/sos/memos-tiers.csv` and `gaia-spec/memcube.md` for context.

---

## 4. SFS-as-FUSE (`sfs.rs`)

SFS v1 is a POSIX-compatible FUSE filesystem. The backend is Qdrant (vector
search) and Graphiti (graph traversal) in userspace. Lance/Arrow are an optional
read-path extension for tabular query patterns.

`sfs_v1_is_fuse()` MUST return `true`.

### POSIX surface

| Call | Behaviour |
|---|---|
| `open(path)` | Returns a file descriptor for a semantic node |
| `read(fd, buf)` | Returns serialised `MemEntry` bytes |
| `write(fd, buf)` | Writes / upserts a semantic node; returns bytes written |
| `readdir(path)` | Lists semantic neighbours; backed by Graphiti edge traversal |
| `unlink(path)` | Soft-delete: marks node as evicted; does not purge vector index immediately |

### Backend mapping

| SFS concept | Backend |
|---|---|
| Vector search (`search_semantic`) | Qdrant (userspace) |
| Graph traversal (`readdir`) | Graphiti (userspace) |
| Tabular query (optional) | Lance / Apache Arrow (userspace, read-only) |
| Kernel boundary | FUSE VFS syscall layer — no kernel module beyond FUSE |

### SFS v1 gate

| Gate | Condition |
|---|---|
| `sfs_v1_is_fuse()` | MUST return `true` |
| ABI-compatible | SFS MUST expose the POSIX subset defined in `abi.md` without breaking changes |
| No kernel module | SFS MUST NOT require a custom kernel module; FUSE daemon only |
| Lance optional | Lance/Arrow integration MUST be feature-gated; absence MUST NOT break the build |

---

## 5. Intent Parser FSM (`intent_parser.rs`)

```
States:
  Idle → Parsing → Resolved
                 → Ambiguous → AwaitingClarification → Resolved
                 → Rejected
```

| State | Trigger | Next state |
|---|---|---|
| `Idle` | Input received | `Parsing` |
| `Parsing` | Single match found | `Resolved` |
| `Parsing` | Multiple matches, low confidence | `Ambiguous` |
| `Parsing` | Safety / policy block | `Rejected` |
| `Ambiguous` | Clarification prompt sent | `AwaitingClarification` |
| `AwaitingClarification` | User response received | `Parsing` (re-enter) |

### Ambiguity handling

`IntentParser::parse(input: &str) -> IntentResult`

| Rule | Constraint |
|---|---|
| Clarification prompt | `Ambiguous` state MUST emit a clarification prompt; MUST NOT silently pick the first candidate |
| Re-entry limit | Parser MUST NOT loop more than 3 clarification cycles before returning `Err(AmbiguityUnresolved)` |
| Rejected is terminal | `Rejected` state MUST NOT transition back to `Parsing` without a new input |
| No silent default | `IntentParser` MUST NOT execute a default action on ambiguous input |

---

## 6. Prohibition Surface

| Prohibition | Error / Return |
|---|---|
| Kernel ONNX in v1 | `kernel_inference_enabled()` → `false`; `Err(KernelInferenceNotEnabled)` |
| NPU preempt mid-inference | `npu_slot_preempts_mid_inference()` → `false` |
| Carbon window drops task | `carbon_window_drops_task()` → `false` |
| Cross-user MemCube recall | `Err(AccessDenied)` |
| SFS requires kernel module | `sfs_v1_is_fuse()` → `true`; no custom kernel module |
| Silent intent default | `Err(AmbiguityUnresolved)` after 3 cycles |

---

## 7. What This Phase Does Not Do

- Does not run ONNX or any inference runtime in kernel space.
- Does not preempt inference tasks mid-run.
- Does not drop tasks silently due to carbon window policy.
- Does not require a custom kernel module for SFS.
- Does not provide a live NPU driver — slot reservation only.
- Does not implement Lance/Arrow as a mandatory dependency.

---

## 8. Acceptance Gate

- [ ] `kernel_inference_enabled()` → `false`
- [ ] `npu_slot_preempts_mid_inference()` → `false`
- [ ] `carbon_window_drops_task()` → `false`
- [ ] `sfs_v1_is_fuse()` → `true`
- [ ] `MemCube::recall` returns `Err(AccessDenied)` for wrong `gaian_id`
- [ ] `MemCube::migrate` is atomic (fails cleanly on error)
- [ ] `MemCube::fuse` does not evict source entries
- [ ] `IntentParser` emits clarification prompt on ambiguous input
- [ ] `IntentParser` returns `Err(AmbiguityUnresolved)` after 3 cycles
- [ ] `cargo test -p gaia-sos` green

---

## 9. Cross-References

- SOS phase-1 stub: `gaia-spec/sos/phase-1.md`
- Scheduler hooks CSV: `gaia-spec/sos/sched-hooks.csv`
- MemOS tiers: `gaia-spec/sos/memos-tiers.csv`; #18
- MemCube overview: `gaia-spec/memcube.md`
- SFS ABI: `gaia-spec/sos/abi.md`
- SOS Phase 2: `gaia-spec/sos/SOS-PHASE-2.md` (#198)
- TCB RFC placeholder: `rfcs/sos-kernel-inference-tcb.md`
- Issues: #192 (epic), #197 (this slice)
