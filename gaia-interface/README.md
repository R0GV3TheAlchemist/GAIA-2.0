# gaia-interface

L6 local-first session for #6 / #27 / #28.

- `Session` CLI/HTTP adapters: `init`, `start`, `intent`, `revoke`.
- `PermissionConsole` renders an HTML trace and agent matrix a user can open
  without a terminal. That is not React, Vite, or a WebSocket.
- `ACCESSIBILITY.md` records basic WCAG considerations.

```bash
cargo test -p gaia-interface
```
