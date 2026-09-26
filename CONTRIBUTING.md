# Contributing to GAIA 2.0

Thank you for contributing to GAIA 2.0. This project is an open, human-governed effort to create an artificial twin of Earth and humanity.

## Ways to contribute

- Report bugs or propose features through GitHub Issues.
- Improve specifications, research notes, documentation, and tests.
- Submit focused pull requests with a clear purpose and verification notes.
- Participate respectfully in technical, ethical, and governance discussions.

## Development expectations

- Keep changes narrowly scoped and explain *why* they are needed.
- Add or update tests when changing behavior.
- Every PR that closes an issue tagged `bug` or `defect` MUST include a regression test in `gaia-integrity/tests/regression.rs` (or a new `rNNNN` integration test in that crate) that would have caught the original defect.
- Preserve privacy, consent, data sovereignty, and least-privilege principles.
- Do not introduce credentials, personal data, or proprietary material into the repository.
- Follow existing repository conventions unless an RFC or issue explicitly changes them.

## Pull requests

Before opening a pull request:

1. Link the relevant issue(s).
2. Describe the problem, approach, and verification performed.
3. Keep commits understandable and avoid unrelated formatting churn.
4. Confirm that applicable tests and checks pass.
5. Call out known limitations, deferred work, or safety considerations.

## Governance and conduct

Contributors must follow the [Code of Conduct](CODE_OF_CONDUCT.md), [Security Policy](SECURITY.md), and [Governance Guide](GOVERNANCE.md).

## Knowledge-domain naming

Knowledge-domain identities, slugs, aliases, legacy mappings, and deprecations must follow [`docs/knowledge/DOMAIN-NAMING.md`](docs/knowledge/DOMAIN-NAMING.md). In particular, curriculum stage is metadata rather than part of a canonical domain identifier; do not introduce new tier-prefixed IDs or directory names such as `basic-*`, `intermediate-*`, or `mastery-*` without an approved compatibility exception.
