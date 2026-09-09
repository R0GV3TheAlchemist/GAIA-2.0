# Contributing to GAIA 2.0

Thank you. This project follows an Apache Way / Linux Foundation hybrid: merit, public RFCs, and lazy consensus.

## How to start

1. Read [`gaia-spec/`](gaia-spec/) (normative) and the blueprint in `Documents/GAIA 2.0 — Super Operating System.md` (informative).
2. Find the layer directory for your change.
3. Open an issue using a template, or an RFC for anything that changes a protocol.

## RFC process

- Copy [`rfcs/0000-template.md`](rfcs/0000-template.md) to `rfcs/NNNN-short-title.md`.
- Open a pull request. Protocol RFCs should also be listed in [`gaia-spec/rfcs.md`](gaia-spec/rfcs.md).
- Discussion happens on the PR and the linked issue.
- **Lazy consensus:** if no maintainer objects within 7 days and CI is green, a maintainer may merge.
- **Breaking changes** (syscall rename, Manifest required-field change, identity scheme change): require a **2/3** vote of sitting TSC members (see [GOVERNANCE.md](GOVERNANCE.md)). Until a legal Foundation exists, the repository owner plus any listed maintainers act as the interim TSC.

Do not silently assume answers to questions listed in `gaia-spec/rfcs.md`.

## Code

- Rust: `rustfmt` + `cargo test`
- Python: Ruff optional; `pytest`
- TypeScript: `tsc --noEmit` + package tests
- Spec examples must validate against the JSON Schema in `gaia-spec/schemas/`

## DCO

Commits should be signed off (`git commit -s`). By contributing you agree your work is licensed under the license of the tree you touch.

## Security

Do not file public issues for vulnerabilities. See [SECURITY.md](SECURITY.md).
