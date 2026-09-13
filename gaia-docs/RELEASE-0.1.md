# GAIA 2.0 — 0.1.0-dev notes

This is not `v1.0.0`. Tracked by #32 and RFC 0002.

## Layer map (what is actually in this tree)

| Layer | Crate | Shipped in-tree | Not shipped |
| --- | --- | --- | --- |
| L0 hardware | docs + kernel ports (open #30) | documented targets | ARM64/RISC-V CI |
| L1 kernel | `gaia-kernel` | userspace executor, Ed25519 audit | Asterinas fork |
| L2 SFS | `gaia-sfs` | in-process semantic store | FUSE mount |
| L3 MemOS | `gaia-memos` | MemCube recall | multi-host migration |
| L4 orchestrator | `gaia-orchestrator` | signed intent, DAG, MCP JSON-RPC | live model backends |
| L5 agents | `gaia-agents` | Wasmtime admit + pack + A2A tags | marketplace / containers |
| L6 interface | `gaia-interface` | local session, HTML console | Ratatui, Axum, Flutter |

## Phase status

- Phase 0 (#2): closed
- Phase 1 (#3): closed
- Phase 2 (#4): open
- Phase 3 (#5): open
- Phase 4 (#6): open
- Phase 5 (#7): open

## How to run what exists

```bash
cargo test --workspace
cargo run -p gaia-kernel --bin gaia-executor
```

`gaia init --profile=developer` is an in-process session command inside
`gaia-interface`. It is not a published installer.
