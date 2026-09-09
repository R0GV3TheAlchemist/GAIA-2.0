#!/usr/bin/env python3
"""Validate AIP Manifest examples against the v1.0 JSON Schema."""
from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "aip-manifest.schema.json"
EXAMPLES = ROOT / "examples"

try:
    from jsonschema import Draft202012Validator
    HAVE_JSONSCHEMA = True
except ImportError:
    HAVE_JSONSCHEMA = False
    Draft202012Validator = None


def load(path: Path):
    with path.open(encoding="utf-8") as fh:
        return json.load(fh)


def fallback_errors(schema: dict, instance: object) -> list[str]:
    if not isinstance(instance, dict):
        return ["(root): manifest must be an object"]
    errs: list[str] = []
    required = schema.get("required", [])
    props = schema.get("properties", {})
    for key in required:
        if key not in instance:
            errs.append(f"{key}: required field is missing")
    if schema.get("additionalProperties") is False:
        for key in instance:
            if key not in props:
                errs.append(f"{key}: additional property not allowed")
    for key, spec in props.items():
        if key not in instance:
            continue
        val = instance[key]
        if "const" in spec and val != spec["const"]:
            errs.append(f"{key}: must equal {spec['const']!r}")
        if "enum" in spec and val not in spec["enum"]:
            errs.append(f"{key}: {val!r} is not one of {spec['enum']}")
        if spec.get("type") == "array" and not isinstance(val, list):
            errs.append(f"{key}: must be an array")
        if spec.get("type") == "array" and spec.get("minItems") and isinstance(val, list):
            if len(val) < spec["minItems"]:
                errs.append(f"{key}: at least {spec['minItems']} item(s) required")
        if spec.get("type") == "array" and isinstance(val, list):
            item = spec.get("items", {})
            if isinstance(item, dict) and item.get("type") == "object":
                item_req = item.get("required", [])
                for i, elem in enumerate(val):
                    if not isinstance(elem, dict):
                        errs.append(f"{key}/{i}: must be an object")
                        continue
                    for rk in item_req:
                        if rk not in elem:
                            errs.append(f"{key}/{i}/{rk}: required field is missing")
    return errs


def format_schema_errors(errors) -> str:
    lines = []
    for err in errors:
        loc = "/".join(str(p) for p in err.absolute_path) or "(root)"
        lines.append(f"  - {loc}: {err.message}")
    return "\n".join(lines) if lines else "  - (unknown error)"


def validate_one(schema, validator, path: Path) -> bool:
    expect_fail = path.name.startswith("invalid-")
    try:
        instance = load(path)
    except json.JSONDecodeError as exc:
        print(f"FAIL {path.name}: JSON parse error: {exc}")
        return False
    if validator is not None:
        errors = sorted(validator.iter_errors(instance), key=lambda e: list(e.path))
        fail = bool(errors)
        detail = format_schema_errors(errors) if errors else ""
    else:
        msgs = fallback_errors(schema, instance)
        fail = bool(msgs)
        detail = "\n".join(f"  - {m}" for m in msgs)
    if expect_fail:
        if fail:
            print(f"OK   {path.name} (rejected as expected)")
            print(detail)
            return True
        print(f"FAIL {path.name}: fixture was supposed to be invalid")
        return False
    if fail:
        print(f"FAIL {path.name}")
        print(detail)
        return False
    print(f"OK   {path.name}")
    return True


def main(argv: list[str]) -> int:
    schema = load(SCHEMA_PATH)
    validator = None
    if HAVE_JSONSCHEMA:
        validator = Draft202012Validator(
            schema, format_checker=Draft202012Validator.FORMAT_CHECKER
        )
    else:
        print("note: jsonschema not installed; using fallback required/enum checks", file=sys.stderr)
    paths = [Path(p) for p in argv[1:]] if len(argv) > 1 else sorted(EXAMPLES.glob("*.json"))
    ok = True
    for path in paths:
        ok = validate_one(schema, validator, path) and ok
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
