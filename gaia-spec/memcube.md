# MemCube schema

**Version:** spec v0.1  
**JSON Schema:** [`schemas/memcube.schema.json`](schemas/memcube.schema.json)

MemCube is the portable memory object (from MemOS; adapted).

## Required fields

| Field | Type |
| --- | --- |
| `id` | UUID |
| `type` | `parametric` ∣ `activation` ∣ `plaintext` ∣ `episodic` ∣ `procedural` ∣ `semantic` |
| `lifecycle` | `active` ∣ `archived` ∣ `compressed` ∣ `migrated` |
| `content_encoding` | `utf-8` ∣ `base64` ∣ `external` |
| `metadata.provenance.source` | string |
| `metadata.importance` | number 0–1 |

`semantic` is included in v0.1 even though the original blueprint sketch
listed five types; world-knowledge cubes need a home. See RFC-MEM-001.

## Tiers (informative mapping)

| Tier | Typical cube types |
| --- | --- |
| 1 Activation | `activation` |
| 2 Working | `plaintext`, `activation` |
| 3 Episodic | `episodic` |
| 4 Semantic | `semantic` |
| 5 Parametric | `parametric` |
