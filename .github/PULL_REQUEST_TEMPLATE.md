## Summary

## Issue

Closes #

## Layer

- [ ] gaia-spec
- [ ] gaia-sdk
- [ ] gaia-kernel / sfs / memos
- [ ] gaia-orchestrator / agents / interface
- [ ] gaia-earth / gaia-gaian / gaia-sa
- [ ] docs / governance

## Moral architecture (#954)

Cite the principle(s) this change serves. See `docs/canon/moral-architecture.md`.

- [ ] Power without domination
- [ ] Autonomy without abandonment of accountability
- [ ] Knowledge without pretending certainty
- [ ] Intelligence without superiority
- [ ] Transformation without violating consent
- [ ] Stewardship without ownership of other people
- [ ] Correction without shame
- [ ] Strength without cruelty

## Agent hygiene

- [ ] I read the files I changed; no invented types
- [ ] Official repo only (`R0GV3TheAvatar/GAIA-2.0`)
- [ ] Did not strip NOTICE / license / author
- [ ] Tests call APIs that exist on this branch

## Checklist

- [ ] Spec examples still validate (`python gaia-spec/tools/validate_aip.py`)
- [ ] Claim tags still valid (`python gaia-spec/tools/check_claim_tags.py`)
- [ ] No silent protocol assumptions — linked an RFC if needed
- [ ] Tests added or explained why not
- [ ] If this closes a `bug`/`defect` issue: regression lock added (`gaia-integrity/tests/regression.rs`, see `tests/regression/README.md`)
- [ ] Signed-off commit (`git commit -s`)
- [ ] Did not promote a prohibited claim to established
