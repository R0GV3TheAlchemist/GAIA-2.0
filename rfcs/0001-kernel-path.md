# RFC 0001: GAIA Kernel path for developer v0

- Status: Accepted for Phase 1
- Layer: L1
- Breaking: no
- Issues: #15, RFC-KERN-001

## Summary

Phase 1 ships a **userspace kernel module** that implements the GAIA syscall
ABI on a stock Linux (or any POSIX) host. An Asterinas framekernel fork is
the *target* path, not the developer-v0 path.

## Decision

| Path | Role in v0 | Role later |
| --- | --- | --- |
| Userspace daemon (`gaia-kernel`) | **Default.** Runnable without root or a custom kernel. | Remains the compatibility shim and test double |
| Linux out-of-tree module | Documented, not required to claim Phase 1 | Optional acceleration |
| Asterinas framekernel fork | Not in this repo yet | Phase 1.5+ when the fork has a home |

Linux userspace programs keep running because we do not replace the host
kernel. Syscalls are library + Unix-domain / in-process ABI, not raw
`syscall(2)` numbers (still RFC-SYS-001).

## TCB notes

v0 trusted computing base is: host kernel, Rust standard library,
`ed25519-dalek`, and this crate. No new privileged code.

## Neural / VRAM scheduler

Interface only: capability flags `vram_mb`, `npu`, `cuda`, `rocm`, `opencl`.
The v0 executor records the hint and ignores it when dispatching no-op tasks.

## Alternatives rejected

- Forking Asterinas in-tree now: would freeze a multi-hundred-kLOC kernel
  before the ABI is stable.
- Requiring FUSE + a kernel module to pass CI: not available in
  unprivileged GitHub runners.
