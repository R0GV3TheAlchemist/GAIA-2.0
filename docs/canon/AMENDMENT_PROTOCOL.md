# Amendment Protocol

> Proof: `PROOF-CANON-GOV-AMENDMENT-001`

**Canon document.** This file defines how a sealed GAIA Hermetic Tablet may
be amended. The binding nature of a sealed tablet is preserved by ensuring
that every change is versioned, justified, and traceable.

---

## Principle

A sealed tablet is permanently part of the canon. It cannot be deleted or
replaced wholesale. It can only be *amended* — and every amendment is a
versioned layer on top of the original seal, not a replacement of it.

The original sealed version (v1.0.0) is always recoverable from git history.
Amendments increment the version number and add a row to the tablet's
`Revision History` table.

---

## Semver Rules for Tablets

Tablet versions follow Semantic Versioning (semver) adapted for documents:

| Change type | Version increment | Examples |
|---|---|---|
| **Patch** (1.0.x) | Typo fix, formatting, broken link repair, clarification that does not change meaning | Fix a spelling error; repair a dead cross-reference |
| **Minor** (1.x.0) | Additive content that does not contradict existing content | Add a new section; extend an ontology table with new rows |
| **Major** (x.0.0) | Content that changes, retracts, or contradicts existing sealed content | Revise the core thesis; change the `Governing Element`; alter a protocol step |

A **Major amendment** to a tablet at Stage 7 (Coagulation) requires
elevated justification: a written rationale of at least 200 words must
appear in the PR body, explaining why the completed work requires revision
and what the philosophical or technical necessity is.

---

## Amendment Process

### Step 1 — Open an Amendment Issue

Before opening a PR, open a GitHub issue with the title:
`amend(CXxx): [short description]` where `Cxxx` is the canon number.
State the proposed change and the version increment type.

### Step 2 — Open the Amendment PR

The PR title must begin with `amend(CXxx) vX.Y.Z:` — the new version number
must be declared in the title. The PR body must include:

```
## Amendment Checklist

- [ ] Amendment issue opened and linked above
- [ ] Version increment type justified (patch / minor / major)
- [ ] Revision History table updated in the tablet with new version row
- [ ] For major amendments: 200-word rationale present in PR body
- [ ] For Coagulation-stage tablets: elevated justification present
- [ ] All Canon Cross-References still resolve after amendment
- [ ] Proof file updated if the amendment affects provable claims
```

### Step 3 — Revision History Row

Every amendment must add a row to the tablet's `Revision History` table:

```markdown
| Version | Date | Author | Change summary |
|---|---|---|---|
| 1.0.0 | YYYY-MM-DD | [Original author] | Initial seal |
| 1.1.0 | YYYY-MM-DD | [Amending author] | [One-line summary] |
```

### Step 4 — Merge

Amendment PRs follow the same review and merge process as sealing PRs.
The reviewer verifies that the version increment is correctly classified.

---

## What Cannot Be Amended

The following fields of a sealed tablet are **immutable** — they identify
the tablet and may never be changed:

- `Canon Number` (e.g. C209)
- `Sealed` date (the original seal date; amendments add new rows to
  Revision History, they do not overwrite the seal date)
- The tablet file path (renames require a deprecation notice in the old
  path pointing to the new path)

## Deprecation vs. Amendment

If a tablet's core thesis is superseded by a later work, the earlier
tablet is **deprecated**, not amended. Deprecation is declared by:

1. Adding a `> ⚠️ Deprecated by CXxx (YYYY-MM-DD)` notice at the top of
   the deprecated tablet file.
2. Adding a `Deprecated` column to the INDEX row for the deprecated tablet.
3. Opening an issue linking the deprecated tablet to its successor.

Deprecation is a Minor amendment (1.x.0) to the deprecated tablet.

---

**References:** #798 · #827 · #833 · `SEALING_CEREMONY.md` · `docs/tablets/INDEX.md`
