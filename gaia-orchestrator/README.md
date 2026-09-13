# gaia-orchestrator

L4 cognitive orchestration. Local-first intent parse, inspectable DAG, pull broker, MCP registry, Ed25519 intent signatures from `gaia-kernel`.

```bash
cargo run -p gaia-orchestrator --bin gaia -- intent "research and summarize CARE"
cargo run -p gaia-orchestrator --bin gaia -- intent "research and summarize CARE" --accept
```

This crate does not yet provide a real local model backend, production scheduler, or OpenTelemetry.
