# GAIA Specification v0.1

This directory contains the in-repo interoperability contract for GAIA 2.0.

## Status

- **Version:** `0.1.0`
- **License:** CC0; see [LICENSE](LICENSE).
- **Normative language:** documents use **MUST**, **MUST NOT**, **SHOULD**, and **MAY** where requirements are intended. Material without those terms is informative unless explicitly marked otherwise.
- **Implementation status:** a specification is not evidence that an implementation exists. Each implementation must identify the requirements it enforces.

## Documents

- [Architecture](architecture.md) — L0–L6 layer responsibilities.
- [Identity and zero trust](identity.md) — identity and access principles.
- [Identity, capabilities, and components](sos/identity-capabilities.md) — entity taxonomy, signed bounded grants, non-escalating delegation, strongly consistent revocation semantics, audit events, component roles, and honest HAL-tier targets.
- [Intent graph](intent-graph.md) — intent representation.
- [MemCube](memcube.md) — governed memory unit.
- [AIP Manifest](aip-manifest.md) — agent package declaration.
- [GAIA primitives](syscalls.md) — host primitive vocabulary.
- [Schemas](schemas/) — machine-readable validation schemas.
- [Examples](examples/) — example documents.
- [Open RFCs](rfcs.md) — unresolved design questions.

## Contributing

Specification changes should describe whether they are normative, add examples or schemas where appropriate, and leave unresolved choices as RFCs rather than silently assuming them.
