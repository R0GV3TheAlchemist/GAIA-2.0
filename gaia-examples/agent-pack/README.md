# agent-pack

Local #25 starter library. Manifests here are documentation copies of the
`gaia-agents` catalog. Deploy is in-process:

```bash
cargo run -p gaia-agents --bin gaia-agent -- agent deploy memory-manager
cargo run -p gaia-agents --bin gaia-agent -- agent deploy researcher
cargo run -p gaia-agents --bin gaia-agent -- agent deploy critic
cargo run -p gaia-agents --bin gaia-agent -- agent review researcher "draft notes"
```

Not included: marketplace publish, WASI package load, live external APIs, A2A wire.
