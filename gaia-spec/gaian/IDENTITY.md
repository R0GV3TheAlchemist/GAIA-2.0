# GAIAN identity file (#218)

A twin is a governed record (#373). This file is what the owner exports.

- `gaian_id` survives `model_swap` / `migrate` (`gaia-gaian` `design.rs`).
- Telos is a purpose block, not a soul.
- Anchors: episodic, semantic, procedural, relational, aspirational.
- Personality is optional and **must** carry `uncertainty` if present.
- Demographics are owner-declared only. Photo must not auto-fill gender or age.
- `inferred_gender` / `inferred_age` are schema-invalid.

Validate:

```bash
python gaia-spec/tools/validate_identity.py
```

Parent #214 stays open.
