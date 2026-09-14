#!/usr/bin/env python3
"""Validate GAIA example artifacts against their explicit versioned schemas."""

import json
from pathlib import Path

from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]
SCHEMAS = ROOT / "schemas"
EXAMPLES = ROOT / "examples"

SCHEMA_BY_KIND = {
    "aip": "aip-manifest.schema.json",
    "token": "capability-token.schema.json",
    "case": "capability-revocation-case.schema.json",
    "github-source-policy": "github-source-policy.schema.json",
    "github-source-result": "source-result.schema.json",
}


def fixture_kind(path: Path) -> str:
    name = path.name
    if name == "github-source-policy.example.json":
        return "github-source-policy"
    if name.startswith("github-source-result.") and name.endswith(".example.json"):
        return "github-source-result"
    if "capability-token" in name:
        return "token"
    if ".case." in name:
        return "case"
    if name.startswith("invalid-") or name.endswith("-agent.json"):
        return "aip"
    raise ValueError(f"no schema routing rule for fixture {name}")


def load_json(path: Path):
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def main() -> int:
    validators = {
        kind: Draft202012Validator(load_json(SCHEMAS / schema_name))
        for kind, schema_name in SCHEMA_BY_KIND.items()
    }
    failures = 0

    for path in sorted(EXAMPLES.glob("*.json")):
        try:
            kind = fixture_kind(path)
            errors = sorted(validators[kind].iter_errors(load_json(path)), key=lambda error: list(error.path))
        except (OSError, ValueError, json.JSONDecodeError) as error:
            print(f"FAIL {path.name} [{locals().get('kind', 'unknown')}]\n  - {error}")
            failures += 1
            continue

        expected_invalid = path.name.startswith("invalid-")
        if expected_invalid and errors:
            print(f"OK   {path.name} (rejected as expected) [{kind}]")
            for error in errors:
                location = ".".join(str(item) for item in error.path) or "(root)"
                print(f"  - {location}: {error.message}")
        elif expected_invalid:
            print(f"FAIL {path.name} [{kind}]\n  - expected validation failure")
            failures += 1
        elif errors:
            print(f"FAIL {path.name} [{kind}]")
            for error in errors:
                location = ".".join(str(item) for item in error.path) or "(root)"
                print(f"  - {location}: {error.message}")
            failures += 1
        else:
            print(f"OK   {path.name} [{kind}]")

    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
