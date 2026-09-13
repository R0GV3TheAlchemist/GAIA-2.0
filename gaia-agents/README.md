# gaia-agents

L5 agent policy harness, Wasmtime guest proof (#24), and A2A handoff (#26).

- Policy admission and Wasmtime fuel/memory limits.
- Handoff carries intent id, allowed memory scope, and cube ids — not raw vault.
- Default package policy rejects unsigned installs.
- Federated jobs redact plaintext memory.

Not included: Git/OCI registry, off-box A2A transport, containerd.

```bash
cargo test -p gaia-agents
```
