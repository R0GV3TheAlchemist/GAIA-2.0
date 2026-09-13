# gaia-interface

L6 local-first session for #6 / #27.

- `Session::exec` is the CLI adapter (`init`, `start`, `intent`, `revoke`, `status`).
- `HttpGateway` is the same session over in-process request objects. No socket is bound.
- Cloud is denied unless the profile opts in.

Not included: Ratatui, Axum/WebSocket/gRPC, Web UI, Studio, voice, vision, mobile.

```bash
cargo test -p gaia-interface
```
