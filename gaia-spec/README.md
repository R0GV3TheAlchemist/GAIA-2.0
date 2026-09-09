# gaia-spec — GAIA Specification v0.1

**Status:** Draft v0.1  
**License:** CC0 1.0  
**Normative language:** RFC 2119 MUST / SHOULD / MAY

This tree is the interoperability contract. The research blueprint in
`Documents/GAIA 2.0 — Super Operating System.md` is **informative**.

| Document | Kind |
| --- | --- |
| [architecture.md](architecture.md) | Normative overview L0–L6 |
| [syscalls.md](syscalls.md) | Normative primitive list |
| [identity.md](identity.md) | Normative zero-trust model |
| [intent-graph.md](intent-graph.md) | Normative schema + JSON Schema |
| [memcube.md](memcube.md) | Normative schema + JSON Schema |
| [aip-manifest.md](aip-manifest.md) | Normative AIP Manifest **v1.0** |
| [rfcs.md](rfcs.md) | Open questions (MUST NOT be silently assumed) |

Validate examples:

```bash
python gaia-spec/tools/validate_aip.py
```
