# Intent graph schema

**Version:** spec v0.1  
**JSON Schema:** [`schemas/intent-graph.schema.json`](schemas/intent-graph.schema.json)

An intent graph is a DAG. Nodes are goals or sub-goals. Edges are
dependencies (`blocks`, `refines`, `fallback-of`).

## Required fields (node)

| Field | Type | Notes |
| --- | --- | --- |
| `intent_id` | UUID | Spec name. Runtime `IntentGraph.id` is the same value. |
| `goal` | string | Human-readable objective |
| `created_by` | principal id | Spec field. Runtime stores this on `SignedIntent.issuer_did`. |
| `constraints` | object | MAY include `time`, `cost`, `privacy`, `compute` |
| `state` | enum | `draft`, `admitted`, `running`, `blocked`, `done`, `failed`, `cancelled` |

## Optional fields

`sub_intents[]`, `context_cubes[]` (MemCube ids), `assigned_agents[]`,
`parent_id`.

Privacy constraint values SHOULD be `local_only`, `federated`, or `cloud`
to match AIP `privacy_level`.

## Runtime store (#20)

`gaia-orchestrator` persists a `StoredIntent` envelope: the parsed graph plus
a kernel Ed25519 `SignedIntent`. The graph document itself is not self-signed.
Ollama and llama.cpp backends are named and refused until wired. Default is
the local stub.
