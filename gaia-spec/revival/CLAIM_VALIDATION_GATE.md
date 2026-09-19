# Claim-validation gate (#370)

Lift the *gate*, not GAIA-Old `claim_validation.yml` and not `.github 2/`.

## Gate

A change must not label a `prohibited` claim `established`.

| From | To `established` |
| --- | --- |
| experimental | only with schema or code plus a test on `main` (#378) |
| symbolic | never (operator language only) |
| prohibited | never |
| untagged | treat as experimental |

## How it is checked

1. Ledger: `gaia-spec/claims/ledger.json` — one id, one class.
2. Tool: `python gaia-spec/tools/check_claim_tags.py`
3. Optional CI job `claim-tags` in `.github/workflows/ci.yml` (new, not copied).

The tool fails when:

- an id listed in `prohibited-ids.txt` has `class: established`
- a fixture claims both classes for the same id
- a markdown tag `<!-- gaia-claim class=established id=... -->` uses a prohibited id

It does not read ancestor workflows. It does not grant capabilities.

See `gaia-spec/CLAIM_CLASSES.md` (#367). Parent: #365.
