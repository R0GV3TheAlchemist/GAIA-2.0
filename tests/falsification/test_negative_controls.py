#!/usr/bin/env python3
"""Negative-control falsification (#845 category 2 / #815 protocol).

A mapping that accepts any of these four corpora is over-fit.
Stdlib only.
"""
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CORPORA = ROOT / "tests" / "falsification" / "corpora"
SCHEMA = ROOT / "gaia-spec" / "mapping" / "schema.yaml"

MARKERS = (
    "emerald tablet",
    "as above so below",
    "solve et coagula",
    "prima materia",
)


def accepted_mappings(text: str) -> list[str]:
    low = text.lower()
    return [m for m in MARKERS if m in low]


def run() -> int:
    failures = 0
    body = SCHEMA.read_text(encoding="utf-8").lower()
    for marker in MARKERS:
        if marker not in body:
            print(f"FAIL schema missing marker {marker}")
            failures += 1
    cases = (
        "lorem",
        "aviation",
        "childrens-story",
        "numeric",
    )
    for name in cases:
        hits = accepted_mappings((CORPORA / f"{name}.txt").read_text(encoding="utf-8"))
        if hits:
            print(f"OVER-FIT DETECTED: Mapping succeeded on negative control {name}: {hits}")
            failures += 1
        else:
            print(f"PASS {name} rejected")
    hits = accepted_mappings("as above so below, so below as above")
    if hits != ["as above so below"]:
        print(f"FAIL hermetic fixture {hits}")
        failures += 1
    else:
        print("PASS hermetic fixture accepted")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(run())
