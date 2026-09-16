# Publication Status

Compatible with #311 `symbolic-traditions-boundary.md`. Every Magic Circle artifact MUST declare exactly one status:

| Status | Meaning |
| --- | --- |
| `private-draft` | Local working material. MUST NOT be treated as public or as authorized knowledge. |
| `approved-for-public-spec` | Specification text approved for a public repository. |
| `approved-for-public-example` | Example or fixture approved for a public repository. |
| `restricted` | Lawful, safety, or rights restriction. MUST NOT be published or trained on by default. |
| `community-governed` | Living community or tradition holds publication and withdrawal authority. |
| `do-not-store` | Retention is forbidden. Honor destruction or non-collection. |

## Rules

1. Only artifacts explicitly approved for public use MAY be committed to a public repository.
2. Public artifacts MUST exclude private personal material.
3. `restricted`, `community-governed`, and `do-not-store` fail closed: no public example, no silent copy into UKD retrieval, no training use.
4. Status MAY move toward restriction or withdrawal more easily than toward public approval.
5. This issue's specification files, if merged, are `approved-for-public-spec`. JSON examples remain uncommitted until review.

## Advisory-only

Publication status is not a truth value. A public spec does not make a knowledge claim true. A private draft is not therefore false. Authority, consent, and epistemic status remain separate fields.
