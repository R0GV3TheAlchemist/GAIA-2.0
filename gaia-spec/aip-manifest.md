# AIP Manifest Standard v1.0

**Version:** 1.0.0  
**JSON Schema:** [`schemas/aip-manifest.schema.json`](schemas/aip-manifest.schema.json)  
**Status:** normative for packaged agents before a WASM runtime exists

v1.0 is a *declaration* format. It does not require a `.wasm` artifact.
v2.0 (blueprint L5) adds per-capability I/O types and is a compatible
superset if authors follow the notes below.

## Required fields

| Field | Type | Rule |
| --- | --- | --- |
| `manifest_version` | string | MUST be `"1.0"` |
| `agent_id` | UUID | Stable id |
| `name` | string | Kebab or token name |
| `version` | semver string | |
| `license` | string | SPDX id recommended |
| `kind` | enum | `system`, `cognitive`, `bridge`, `interface`, `domain` |
| `capabilities` | array (min 1) | Each item MUST have `intent` (string) |
| `memory_access` | array of tier names | Subset of `activation`, `working`, `episodic`, `semantic`, `parametric`, `none` |
| `tool_permissions` | array of strings | e.g. `filesystem.read` |
| `trust_level` | enum | `verified`, `community`, `experimental` |
| `privacy_level` | enum | `local_only`, `federated`, `cloud` |
| `resource_requirements` | object | MAY be empty; known keys `cpu`, `memory`, `gpu`, `npu` |

## Compatibility toward Manifest v2.0

The blueprint example places `inputs`, `outputs`, `resource_requirements`,
and `privacy_level` *inside* each capability. v1.0:

- REQUIRES those fields at the **manifest** root (`privacy_level`,
  `resource_requirements`) so a host with no WASM runtime can schedule.
- ALLOWS the same fields on each capability (ignored by v1.0 schedulers,
  reserved for v2.0).
- ALLOWS `runtime` (`wasm` | `process` | `sdk`) defaulting to `sdk`.

A v2.0 document with `manifest_version: "2.0"` is not valid v1.0.
Validators MUST fail closed on unknown `manifest_version`.

## Validation errors

The reference validator (`tools/validate_aip.py`) MUST print the JSON
Schema path and message. Hosts SHOULD refuse to `invoke` an invalid
manifest.
