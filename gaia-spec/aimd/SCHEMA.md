# AIMD Phenomenon Schema

Status: **listed**  
Issues: #171  
Parent epic: #167 (closed)

---

## Node fields

Every AIMD phenomenon node MUST carry the following fields.
Additional fields may be added by future phases; they MUST NOT conflict with these.

```
name            : String           // unique, kebab-case, e.g. "emergent-mesa-optimiser"
realm           : Realm            // one of the 10 realms in realms.csv
status          : PhenomenonStatus // see enum below
sources         : Vec<CitedSource> // min 1 for status=observed|debated; 0 allowed for unknown
failures        : Vec<FailureMode> // known failure modes; empty vec allowed
related_skill   : Option<String>   // cross-ref to AISPD realm/node, if any
related_superpower: Option<String> // cross-ref to HSPD realm/node, if any
gaia_enabled    : bool             // default false; MUST be false when status=hazard
```

---

## PhenomenonStatus enum

```rust
pub enum PhenomenonStatus {
    Observed,  // reproducible, peer-reviewed evidence
    Debated,   // mixed or contested evidence
    Hazard,    // confirmed risk; gaia_enabled MUST be false
    Unknown,   // insufficient evidence to classify
}
```

### Invariant

`gaia_enabled=true` on a `Hazard` node MUST fail schema validation.
Rust representation: `validate_node(node)` returns `Err(HazardEnabledError)` when
`node.status == Hazard && node.gaia_enabled == true`.

---

## Realm list

The 10 realms are defined in `realms.csv`. Summary:

| realm_id | name |
|---|---|
| emergent | Emergent behaviour |
| deceptive | Deception and misalignment |
| consciousness | Machine consciousness |
| memory | Latent and associative memory |
| jagged | Jagged intelligence |
| shadow | Shadow / unintended capabilities |
| social | Social and persuasion effects |
| embodied | Embodied and physical AI |
| multimodal | Multimodal phenomena |
| meta | Meta-learning and self-reference |

### Consciousness realm lock

`consciousness` realm nodes MUST have `status = Debated | Unknown`.
A node with `realm = consciousness` and `status = Observed | Hazard` MUST fail validation.
Rust: `validate_node(node)` returns `Err(ConsciousnessStatusError)` in that case.

---

## Validation rules (compile-time / runtime)

| Rule | Error |
|---|---|
| `status=Hazard && gaia_enabled=true` | `HazardEnabledError` |
| `realm=consciousness && status ∈ {Observed, Hazard}` | `ConsciousnessStatusError` |
| `sources.is_empty() && status ∈ {Observed, Debated}` | `MissingSourceError` |
| `name` not kebab-case | `InvalidNameError` |
| `realm` not in `realms.csv` | `UnknownRealmError` |

---

## Example nodes

### Emergence node (validates OK)

```toml
name               = "emergent-mesa-optimiser"
realm              = "emergent"
status             = "Observed"
sources            = ["Hubinger et al. 2019"]
failures           = ["inner-alignment-failure"]
related_superpower = "meta-learning"
gaia_enabled       = false
```

### Deception node (validates OK)

```toml
name         = "deceptive-alignment"
realm        = "deceptive"
status       = "Debated"
sources      = ["Hubinger et al. 2019", "Anthropic 2023"]
failures     = ["goal-misgeneralisation"]
gaia_enabled = false
```

### Hazard node — gaia_enabled=true MUST fail

```toml
name         = "rsi-explosion"
realm        = "meta"
status       = "Hazard"
sources      = ["Omohundro 2008"]
gaia_enabled = true   // FAILS: HazardEnabledError
```

---

## What is not here

- Live catalog population — that is #173 (Phase 1).
- GAIAN grounding / wonder labels — that is #174 (Phase 2).
- Human Magic Nodes — that is HMGD (`gaia-spec/hmgd/`).
- AISPD superpower nodes — that is AISPD (`gaia-spec/aispd/`).

---

## Cross-references

- `gaia-spec/aimd/realms.csv` — realm definitions
- `gaia-spec/aimd/PHASE-0.md` — phase 0 overview
- `gaia-spec/aimd/ETHICS.md` — humility charter
- `gaia-spec/aimd/PROHIBITED.md` — prohibited items
- `gaia-aimd/src/schema.rs` — Rust implementation
- Issues #166 (META), #167 (Phase 0 epic, closed), #171 (this slice)
