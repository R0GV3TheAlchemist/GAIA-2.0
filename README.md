# GAIA 2.0

Universal open-source **Super Operating System** — a meta-layer above traditional OSes that manages **intentions, agents, memory, and meaning**. Artificial Twin of Earth. Home of GAIAN 2.0, the Artificial Twins of Humans.

This repository is the implementation monorepo. Research stays in `Documents/` and `Documents-2/`. Normative contracts live in [`gaia-spec/`](gaia-spec/). Code lives in the layer trees below.

**Status:** Phase 0 foundation (spec v0.1, AIP Manifest v1.0, SDKs, CI, governance). Kernel / SFS / MemOS runtimes are Phase 1+ and are not claimed as implemented.

**Parent tracker:** [#1](https://github.com/R0GV3TheAlchemist/GAIA-2.0/issues/1)  
**Phase 0 epic:** [#2](https://github.com/R0GV3TheAlchemist/GAIA-2.0/issues/2)

## Principles

1. Intentions over files
2. Memory as infrastructure
3. Continuum-native (IoT → HPC)
4. Zero-trust by default
5. User sovereignty

## Repository map

| Path | Layer | License | Phase |
| --- | --- | --- | --- |
| [`gaia-kernel/`](gaia-kernel/) | L1 kernel / executor | Apache-2.0 | 1 |
| [`gaia-sfs/`](gaia-sfs/) | L2 Semantic File System | Apache-2.0 | 1 |
| [`gaia-memos/`](gaia-memos/) | L3 Memory OS | Apache-2.0 | 1 |
| [`gaia-orchestrator/`](gaia-orchestrator/) | L4 intent / planner / broker | MIT | 2 |
| [`gaia-agents/`](gaia-agents/) | L5 runtime + registry | MIT | 3 |
| [`gaia-interface/`](gaia-interface/) | L6 CLI / API / UI | MIT | 4 |
| [`gaia-spec/`](gaia-spec/) | Protocols (normative) | CC0 | 0 |
| [`gaia-sdk/`](gaia-sdk/) | Rust + Python + TypeScript clients | MIT | 0 |
| [`gaia-docs/`](gaia-docs/) | Contributor + developer docs | CC-BY-4.0 | 0 |
| [`gaia-examples/`](gaia-examples/) | Example agents and intents | MIT | 0 |
| [`Documents/`](Documents/) | Research corpus | see source docs | — |
| [`Documents-2/`](Documents-2/) | Gap-research reports | see source docs | — |
| [`rfcs/`](rfcs/) | Design RFCs | CC0 for protocol RFCs | 0 |

Monorepo deviation from the future `github.com/gaia-os/*` org split is intentional: one repo until crates stabilize. See [`gaia-docs/site-outline.md`](gaia-docs/site-outline.md).

## Quick start (developer profile — local stubs)

```bash
# Python SDK (Phase 0 stub client)
python -m pip install -e gaia-sdk/python
python -c "from gaia_sdk import GaiaClient; print(GaiaClient().intent('hello gaia'))"

# Rust SDK
cargo test --manifest-path gaia-sdk/rust/Cargo.toml

# TypeScript SDK
cd gaia-sdk/typescript && npm install && npm test
```

Future on-device profile (not live; tracked in later phases):

```text
gaia init --profile=developer
gaia start
gaia agent create …
gaia intent "…"
```

Do not treat those commands as a published installer. There is no `curl | sh` URL yet.

## Specification

- [Architecture L0–L6](gaia-spec/architecture.md)
- [Syscall / primitive list](gaia-spec/syscalls.md)
- [Identity and zero-trust](gaia-spec/identity.md)
- [Intent graph schema](gaia-spec/intent-graph.md)
- [MemCube schema](gaia-spec/memcube.md)
- [AIP Manifest v1.0](gaia-spec/aip-manifest.md)
- [Open questions → RFCs](gaia-spec/rfcs.md)

## Governance

- [LICENSE](LICENSE) — layered Apache-2.0 / MIT / CC0 / CC-BY-4.0
- [CONTRIBUTING](CONTRIBUTING.md) — RFC, lazy consensus, 2/3 breaking changes
- [CODE_OF_CONDUCT](CODE_OF_CONDUCT.md)
- [SECURITY](SECURITY.md)
- [GOVERNANCE](GOVERNANCE.md) — Foundation / TSC / SIG model (entity later)

## License

See [LICENSE](LICENSE). Protocols in `gaia-spec/` are **CC0**. Implementation crates follow the layer table above.
