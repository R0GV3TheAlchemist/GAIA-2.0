# gaia-agents

L5 agent policy harness plus a Wasmtime guest proof for #24.

- Policy admission (`AgentRuntime`) is data-level.
- `WasmRuntime` admits that policy, then runs embedded WAT with fuel and memory limits.
- No WASI linker, no marketplace package load, no containerd.

```bash
cargo test -p gaia-agents
```
