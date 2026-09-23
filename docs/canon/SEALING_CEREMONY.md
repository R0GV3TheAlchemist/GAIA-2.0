# Sealing Ceremony

**Canon document.** This file defines the formal process by which a new GAIA
Hermetic Tablet is sealed. A tablet that has not completed this ceremony is a
**draft**. Only a sealed tablet is binding canon.

---

## What Sealing Means

Sealing a tablet is an irreversible declaration that the work is complete,
reviewed, and admitted to the permanent canon. It does not mean the work
cannot be amended — it means the original sealed version is preserved and any
change must follow `AMENDMENT_PROTOCOL.md`.

## Prerequisites

Before a tablet may be sealed, the following must be true:

1. **All required fields are populated.** The tablet must pass the schema
   compliance check defined in #823. Required fields include: `Title`,
   `Canon Number`, `Governing Element`, `Governing Stage`, `Author`,
   `Sealed`, `Revision History`, `Canon Cross-References`, `Proof File`.

2. **The governing element is valid.** The `Governing Element` value must
   appear in `ELEMENT_ONTOLOGY.md`.

3. **The governing stage is valid.** The `Governing Stage` value must appear
   in `STAGE_SEQUENCE.md`.

4. **A proof file exists.** The `Proof File` field must point to an existing
   file in `proofs/`. That file must back-link to this tablet (#826).

5. **The tablet is indexed.** The tablet must appear in `docs/tablets/INDEX.md`
   with its canon number, title, path, governing element, governing stage,
   and sealed date.

6. **Cross-references are verified.** All `Canon Cross-References` entries
   must resolve to existing files (#825).

## The Ceremony

### Step 1 — Draft Review

The author opens a pull request targeting `main`. The PR title must begin
with `seal(CXxx):` where `Cxxx` is the canon number. The PR body must
include the **Tablet Sealing Checklist** (reproduced below).

### Step 2 — Checklist Completion

The author or reviewer marks each checklist item. The PR may not be merged
until all items are checked.

```
## Tablet Sealing Checklist

- [ ] All required schema fields populated
- [ ] Governing Element validated against ELEMENT_ONTOLOGY.md
- [ ] Governing Stage validated against STAGE_SEQUENCE.md
- [ ] Proof file exists and back-links to this tablet
- [ ] Tablet appears in docs/tablets/INDEX.md
- [ ] All Canon Cross-References resolve
- [ ] Revision History table present with v1.0.0 entry
- [ ] Sealed date set to today's date (YYYY-MM-DD)
- [ ] Author field matches the GitHub handle or legal name of the sealing party
```

### Step 3 — The Seal Date

The `Sealed` field in the tablet header is set to the merge date of the
sealing PR, in `YYYY-MM-DD` format. It must not be set before merge.

### Step 4 — INDEX Update

Upon merge, `docs/tablets/INDEX.md` must be updated (in the same PR or a
immediate follow-up PR) to include the newly sealed tablet with its
`Sealed` date populated.

### Step 5 — Announcement (optional)

For tablets of Canon Number C200 and above, a summary comment is posted
on the tracking epic (#798 or successor) noting the canon number, title,
and sealed date.

## Retroactive Sealing

Tablets that were written before this ceremony document existed may be
retroactively sealed by completing the checklist against their existing
content. The sealed date for retroactively sealed tablets is the date the
sealing PR merges, not the date the tablet was originally written.

## Roles

| Role | Responsibility |
|---|---|
| Author | Drafts the tablet, opens the sealing PR, completes the checklist |
| Reviewer | Verifies schema compliance and cross-references; approves the PR |
| Maintainer | Merges the sealing PR; may act as both Reviewer and Maintainer |

For a single-contributor repository, one person may hold all three roles.
Self-review is permitted but the checklist must still be completed.

**References:** #798 · #823 · #825 · #826 · #833 · #835 · `AMENDMENT_PROTOCOL.md`
