#!/usr/bin/env python3
"""Epic-first GitHub → work_items ingest. Never dump more than 50 OPEN rows per run."""
from __future__ import annotations

BATCH_LIMIT = 50
META_PARENTS = {
    190: None,
    176: None,
    201: None,
    213: None,
    191: 190,
    177: 176,
    202: 201,
    58: 213,
}

PHASE0 = [34, 78, 94, 108, 122, 133, 145, 156, 167]


def plan_batch(open_numbers: list[int]) -> list[int]:
    """Parents first, then Phase-0, then remaining — capped."""
    ordered = []
    for n in list(META_PARENTS) + PHASE0:
        if n in open_numbers and n not in ordered:
            ordered.append(n)
    for n in open_numbers:
        if n not in ordered:
            ordered.append(n)
    return ordered[:BATCH_LIMIT]


if __name__ == "__main__":
    print("batch_limit", BATCH_LIMIT)
    print("meta_parents", list(META_PARENTS))
    print("phase0", PHASE0)
