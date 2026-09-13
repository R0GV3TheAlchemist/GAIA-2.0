# gaia-interface

L6 local-first session for #6 / #27.

- `Session::exec`: `init`, `start`, `intent`, `revoke`.
- `HttpGateway` is the same session over in-process request objects.
- `OrchestratorGateway` forwards an intent through L4 `IntentEngine` parse+store,
  then records the session stream. No Axum socket.

```bash
cargo test -p gaia-interface
```
