#!/usr/bin/env python3
import json
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TESTS = ROOT / "tests"
EXAMPLES = ROOT / "examples"
REQUIRED = {"id", "scope", "proposal", "responsible_attention", "diamond", "authority", "decision", "feedback"}
ALLOWED = REQUIRED
DECISIONS = {"proceed", "narrow", "defer", "escalate", "refuse"}
AUTH = {"authorized", "requires_escalation", "not_authorized"}
CONSENT = {"not_required", "valid", "missing", "refused"}

def fail(message):
    raise ValueError(message)

def validate(data, name):
    unknown = set(data) - ALLOWED
    if unknown:
        fail(f"{name}: unknown top-level fields: {sorted(unknown)}")
    missing = REQUIRED - set(data)
    if missing:
        fail(f"{name}: missing top-level fields: {sorted(missing)}")
    attention = data["responsible_attention"]
    for key in ("observations", "uncertainty", "affected_parties", "alternatives"):
        if key not in attention:
            fail(f"{name}: missing responsible_attention.{key}")
    for key in ("observations", "affected_parties", "alternatives"):
        if not attention[key]:
            fail(f"{name}: responsible_attention.{key} must not be empty")
    diamond = data["diamond"]
    for key in ("truth", "care", "growth", "balance", "wisdom"):
        if not diamond.get(key):
            fail(f"{name}: missing diamond.{key}")
    authority = data["authority"]
    if authority.get("status") not in AUTH:
        fail(f"{name}: invalid authority.status")
    if authority.get("consent_status") not in CONSENT:
        fail(f"{name}: invalid authority.consent_status")
    if not authority.get("subsidiarity"):
        fail(f"{name}: missing authority.subsidiarity")
    if "hard_constraints" not in authority:
        fail(f"{name}: missing authority.hard_constraints")
    if data["decision"] not in DECISIONS:
        fail(f"{name}: invalid decision")
    feedback = data["feedback"]
    for key in ("metrics", "review_at", "repair_path"):
        if key not in feedback:
            fail(f"{name}: missing feedback.{key}")
    if not feedback["metrics"] or not feedback["repair_path"]:
        fail(f"{name}: feedback metrics and repair_path must not be empty")
    datetime.fromisoformat(feedback["review_at"].replace("Z", "+00:00"))
    if data["decision"] == "proceed" and (authority["status"] != "authorized" or authority["consent_status"] in {"missing", "refused"}):
        fail(f"{name}: proceed requires authorized status and non-missing/non-refused consent")
    if authority["status"] == "not_authorized" and data["decision"] == "proceed":
        fail(f"{name}: not_authorized cannot proceed")
    if authority["consent_status"] == "refused" and data["decision"] == "proceed":
        fail(f"{name}: refused consent cannot proceed")

def load(path):
    with path.open() as handle:
        return json.load(handle)

def expect(path, valid):
    try:
        validate(load(path), path.name)
    except Exception as error:
        if valid:
            raise
        print(f"expected failure: {path.name}: {error}")
        return
    if not valid:
        fail(f"{path.name}: expected validation failure")
    print(f"validated: {path.name}")

def main():
    expect(TESTS / "valid-case.json", True)
    for name in ("missing-evidence.json", "no-repair-path.json", "prohibited-aggregate-score.json"):
        expect(TESTS / name, False)
    for path in sorted(EXAMPLES.glob("*.json")):
        expect(path, True)

if __name__ == "__main__":
    main()
