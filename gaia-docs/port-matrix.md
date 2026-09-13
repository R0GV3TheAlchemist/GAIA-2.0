# Port compatibility matrix (#30)

Documented targets for the userspace kernel. CI today runs on Linux x86_64.
Cross-builds are documented commands, not a claim that every ISA is tested in Actions.

| Profile | rustc target | Budget (RSS / binary) | Crates |
| --- | --- | --- | --- |
| Linux x86_64 | `x86_64-unknown-linux-gnu` | 512 MiB / 64 MiB | full workspace |
| Linux ARM64 | `aarch64-unknown-linux-gnu` | 512 MiB / 64 MiB | full workspace |
| Apple Silicon | `aarch64-apple-darwin` | 512 MiB / 64 MiB | full workspace |
| RISC-V experimental | `riscv64gc-unknown-linux-gnu` | 256 MiB / 32 MiB | kernel executor, no agents |
| IoT | `aarch64-unknown-linux-gnu` | **8 MiB / 2 MiB** | executor + sensors only |
| HPC | `x86_64-unknown-linux-gnu` | 32 GiB / 128 MiB | executor + MemOS + agents |

## Documented builds

```bash
# Linux x86_64 (CI host)
cargo test -p gaia-kernel

# Linux ARM64 (cross)
rustup target add aarch64-unknown-linux-gnu
cargo build -p gaia-kernel --target aarch64-unknown-linux-gnu

# Apple Silicon
rustup target add aarch64-apple-darwin
cargo build -p gaia-kernel --target aarch64-apple-darwin

# RISC-V experimental
rustup target add riscv64gc-unknown-linux-gnu
cargo build -p gaia-kernel --target riscv64gc-unknown-linux-gnu
```

## IoT budget

The IoT profile is executor + sensors. MemOS and the agent pack are out of budget.
Stated cap: 8 MiB RSS, 2 MiB binary. Not a measured firmware image.

## HPC adapter

`HpcAdapter` is a local pull queue. Submit a batch job, pull it. There is no live
Slurm controller or MPI launcher on the wire.
