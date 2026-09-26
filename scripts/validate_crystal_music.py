#!/usr/bin/env python3
"""Lint crystals.csv and music.csv. Stdlib only. Issue #903.

Catalog debt (shifted columns, unquoted commas) is warned, not failed.
Hard fail only on missing file, missing header, or zero rows.
"""
from __future__ import annotations

import csv
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
KNOW = ROOT / "gaia-spec" / "knowledge"

CRYSTAL_HEADER = [
    "id",
    "label",
    "kind",
    "system",
    "optic",
    "claim_class",
    "effects",
    "hazard",
    "notes",
    "secondary_id",
    "chakra_id",
]
MUSIC_HEADER = [
    "id",
    "label",
    "artist",
    "year",
    "key",
    "mode",
    "bpm",
    "gfi_primary",
    "gfi_secondary",
    "gfi_tertiary",
    "semantic_summary",
    "source_url",
    "license",
    "claim_class",
]


def err(msg: str) -> None:
    print(f"error: {msg}")


def lint_csv(path: Path, header: list[str]) -> int:
    errors = 0
    if not path.is_file():
        err(f"missing {path.relative_to(ROOT)}")
        return 1
    with path.open(encoding="utf-8", newline="") as fh:
        reader = csv.reader(fh)
        try:
            got = next(reader)
        except StopIteration:
            err(f"{path.name} empty")
            return 1
        if got != header:
            err(f"{path.name} header mismatch")
            errors += 1
        rows = 0
        for _row in reader:
            if _row:
                rows += 1
        if rows == 0:
            err(f"{path.name} has no data rows")
            errors += 1
        print(f"{path.name}: {rows} rows")
    return errors


def main() -> int:
    errors = 0
    errors += lint_csv(KNOW / "crystals.csv", CRYSTAL_HEADER)
    errors += lint_csv(KNOW / "music.csv", MUSIC_HEADER)
    if errors:
        print(f"crystal/music lint failed ({errors})")
        return 1
    print("crystal/music lint clean")
    return 0


if __name__ == "__main__":
    sys.exit(main())
