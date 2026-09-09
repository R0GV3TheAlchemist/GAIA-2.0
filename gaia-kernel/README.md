# gaia-kernel

- **Layer:** L1 / L0 executor
- **License:** Apache-2.0
- **Phase:** 1

Userspace kernel path for developer machines. Decision: [RFC 0001](../rfcs/0001-kernel-path.md).

```bash
cargo test -p gaia-kernel
cargo run -p gaia-kernel --bin gaia-executor
```

The executor **pulls** tasks from an in-process broker. It does not listen
on a port, so it works behind NAT.
