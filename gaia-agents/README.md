# gaia-agents

L5 agent policy harness, Wasmtime guest proof (#24), and local core pack (#25).

```bash
cargo test -p gaia-agents
cargo run -p gaia-agents --bin gaia-agent -- agent deploy memory-manager
cargo run -p gaia-agents --bin gaia-agent -- agent deploy researcher
```

`gaia agent deploy` is the command shape. The binary is `gaia-agent` so it does
not collide with the L4 `gaia` intent CLI. Deploy admits policy and records the
agent. It does not start a container or publish to a marketplace.

Critic review is host-local. It is not the #26 A2A protocol.
