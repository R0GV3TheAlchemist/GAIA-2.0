# OPA vs Cedar — decision record (#344)

First implementation is typed Rust in `PolicyEngine`. Neither OPA nor Cedar is a dependency.

## Criteria

| Criterion | Typed Rust now | Later OPA/Rego | Later Cedar |
| --- | --- | --- | --- |
| Deployment fit | In-process with fake adapter | Sidecar / library | In-process library |
| Policy lifecycle | crate version `gaia-acp-policy-v0.1` | bundle + review | schema + review |
| Schema validation | Rust types | partial | first-class schema |
| Latency | sync, local | extra hop if sidecar | sync |
| Testability | `cargo test` fixtures | extra harness | extra harness |
| TCB impact | small crate | Rego runtime | Cedar runtime |

## Adoption gate

Do not add OPA or Cedar until: policy change-control exists, schema is frozen, and a second independent implementation is required. Compatibility means the same facts (`principal`, `action`, `resource`, `manifest`, `approval`, `now`, `revoked`, `stop`) can be exported later.
