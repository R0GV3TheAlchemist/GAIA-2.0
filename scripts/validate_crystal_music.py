#!/usr/bin/env python3
"""Lint crystals.csv and music.csv. Stdlib only. Issue #903."""
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
CHAKRA = {
    "crown",
    "third-eye",
    "throat",
    "heart",
    "solar-plexus",
    "sacral",
    "root",
    "none",
}


def err(msg: str) -> None:
    print(f"error: {msg}")


def lint_csv(path: Path, header: list[str], claim_col: str) -> int:
    errors = 0
    if not path.is_file():
        err(f"missing {path.relative_to(ROOT)}")
        return 1
    with path.open(encoding="utf-8", newline="") as fh:
        reader = csv.DictReader(fh)
        if reader.fieldnames != header:
            err(f"{path.name} header mismatch")
            errors += 1
        ids: set[str] = set()
        rows = 0
        for i, row in enumerate(reader, start=2):
            rows += 1
            rid = (row.get("id") or "").strip()
            if not rid:
                err(f"{path.name}:{i} empty id")
                errors += 1
            if rid in ids:
                print(f"warn: {path.name}:{i} duplicate id {rid} (listed debt, not a fail)")
            ids.add(rid)
            if (row.get(claim_col) or "").strip() != "listed-only":
                err(f"{path.name}:{i} claim_class must be listed-only")
                errors += 1
            if path.name == "crystals.csv":
                if (row.get("effects") or "").strip():
                    err(f"{path.name}:{i} effects must stay empty")
                    errors += 1
                chakra = (row.get("chakra_id") or "").strip()
                if chakra not in CHAKRA:
                    err(f"{path.name}:{i} bad chakra_id {chakra!r}")
                    errors += 1
        if rows == 0:
            err(f"{path.name} has no data rows")
            errors += 1
        print(f"{path.name}: {rows} rows")
    return errors


def main() -> int:
    errors = 0
    errors += lint_csv(KNOW / "crystals.csv", CRYSTAL_HEADER, "claim_class")
    errors += lint_csv(KNOW / "music.csv", MUSIC_HEADER, "claim_class")
    if errors:
        print(f"crystal/music lint failed ({errors})")
        return 1
    print("crystal/music lint clean")
    return 0


if __name__ == "__main__":
    sys.exit(main())
