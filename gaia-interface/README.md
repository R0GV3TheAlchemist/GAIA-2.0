# gaia-interface

L6 local-first session for #6 / #27.

- `Session::exec`: `init`, `start`, `agent create|deploy`, `intent`, `memory`, `audit`, `revoke`.
- `HttpGateway` is the same session over in-process request objects. No socket.
- Cloud is denied unless the profile opts in.

Not included: Ratatui, Axum/WebSocket/gRPC, Web UI, Studio, voice, vision, mobile,
or a live orchestrator socket.

```bash
cargo test -p gaia-interface
```
