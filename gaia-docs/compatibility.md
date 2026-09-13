# Compatibility matrix (pre-1.0)

Honest matrix for #32. CI proves Linux x86_64. Other rows are targets.

| Surface | Status |
| --- | --- |
| Host OS: Linux x86_64 | CI (`cargo test --workspace`) |
| Host OS: Linux ARM64 | documented target only |
| Host OS: macOS Apple Silicon | documented target only |
| Host OS: RISC-V | experimental target only |
| WASM guest | Wasmtime 25, capability admit |
| MCP | in-process JSON-RPC, unsigned reject |
| WASI filesystem / network | denied without grant |
| IoT profile | stated 8 MiB / 2 MiB budget |
| HPC Slurm/MPI | local pull adapter only |

See also `gaia-docs/port-matrix.md` if that file is on the branch you are reading.
