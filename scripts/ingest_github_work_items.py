#!/usr/bin/env python3
"""Epic-first GitHub → work_items ingest. Never dump more than 50 OPEN rows."""
from __future__ import annotations

BATCH_LIMIT = 50
OPEN_CAP = 50
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


def plan_batch(open_numbers: list[int], already_mirrored: int = 0) -> list[int]:
    room = max(0, OPEN_CAP - already_mirrored)
    ordered = []
    for n in list(META_PARENTS) + PHASE0:
        if n in open_numbers and n not in ordered:
            ordered.append(n)
    for n in open_numbers:
        if n not in ordered:
            ordered.append(n)
    return ordered[: min(BATCH_LIMIT, room)]


if __name__ == "__main__":
    print("open_cap", OPEN_CAP)
    print("room_if_at_cap", plan_batch(list(range(1, 300)), already_mirrored=50))
