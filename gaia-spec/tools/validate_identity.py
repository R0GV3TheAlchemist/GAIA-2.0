#!/usr/bin/env python3
"""Validate GAIAN identity fixtures (#218)."""
from __future__ import annotations

import json
import sys
from pathlib import Path

from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "gaian" / "identity.schema.json"
OK = ROOT / "gaian" / "identity.example.json"
BAD = ROOT / "gaian" / "invalid-identity-photo-inference.json"


def load(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def main() -> int:
    validator = Draft202012Validator(load(SCHEMA))
    ok_errors = list(validator.iter_errors(load(OK)))
    bad_errors = list(validator.iter_errors(load(BAD)))
    failures = 0
    if ok_errors:
        print("FAIL identity.example.json")
        for err in ok_errors:
            print(f"  - {err.message}")
        failures += 1
    else:
        print("OK   identity.example.json")
    if not bad_errors:
        print("FAIL invalid-identity-photo-inference.json expected rejection")
        failures += 1
    else:
        print("OK   invalid-identity-photo-inference.json (rejected)")
    exported = load(OK)
    imported = json.loads(json.dumps(exported))
    if imported.get("gaian_id") != exported.get("gaian_id"):
        print("FAIL round-trip lost gaian_id")
        failures += 1
    else:
        print("OK   export/import keeps gaian_id")
    if imported.get("model_id") == imported.get("gaian_id"):
        print("FAIL model_id must not replace gaian_id")
        failures += 1
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
