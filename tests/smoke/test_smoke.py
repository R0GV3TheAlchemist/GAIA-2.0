#!/usr/bin/env python3
"""System smoke tests (#845 category 4). Stdlib only."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def run() -> int:
    failures = 0
    data = json.loads((ROOT / "docs" / "color" / "color-map.json").read_text(encoding="utf-8"))
    names = [t["name"] for t in data["tablets"]]
    if len(names) != 18:
        print(f"FAIL tablet count {len(names)}")
        failures += 1
    else:
        print("PASS 18 tablets")
    missing = [t["file"] for t in data["tablets"] if not (ROOT / t["file"]).is_file()]
    if missing:
        print(f"FAIL missing tablet files {missing}")
        failures += 1
    else:
        print("PASS tablet files exist")
    for rel in (
        "governance/GAIA_GOVERNANCE.md",
        "docs/tablets/INDEX.md",
        "CONTRIBUTING.md",
    ):
        if not (ROOT / rel).is_file():
            print(f"FAIL missing {rel}")
            failures += 1
        else:
            print(f"PASS {rel}")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(run())
