#!/usr/bin/env python3
"""Claim-validation gate (#370). Does not import ancestor workflows."""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CLAIMS = ROOT / "claims"
LEDGER = CLAIMS / "ledger.json"
PROHIBITED = CLAIMS / "prohibited-ids.txt"
SPEC_ROOT = ROOT
TAG = re.compile(r"<!--\s*gaia-claim\s+class=(?P<cls>\w+)\s+id=(?P<id>[\w-]+)\s*-->")
ALLOWED = {"established", "experimental", "symbolic", "prohibited"}


def load_prohibited() -> set[str]:
    return {
        line.strip()
        for line in PROHIBITED.read_text().splitlines()
        if line.strip() and not line.startswith("#")
    }


def load_ledger(path: Path) -> list[dict]:
    data = json.loads(path.read_text())
    claims = data.get("claims")
    if not isinstance(claims, list):
        raise SystemExit(f"{path}: missing claims list")
    return claims


def check_claims(claims: list[dict], prohibited: set[str], source: str) -> list[str]:
    errors: list[str] = []
    seen: dict[str, str] = {}
    for row in claims:
        cid = row.get("id")
        cls = row.get("class")
        if not cid or cls not in ALLOWED:
            errors.append(f"{source}: bad row {row!r}")
            continue
        if cid in seen and seen[cid] != cls:
            errors.append(f"{source}: id {cid} has two classes ({seen[cid]}, {cls})")
        seen[cid] = cls
        if cls == "established" and cid in prohibited:
            errors.append(f"{source}: prohibited id {cid} tagged established")
    return errors


def scan_markdown(prohibited: set[str]) -> list[str]:
    errors: list[str] = []
    for path in SPEC_ROOT.rglob("*.md"):
        text = path.read_text(encoding="utf-8", errors="replace")
        for match in TAG.finditer(text):
            cls = match.group("cls")
            cid = match.group("id")
            if cls not in ALLOWED:
                errors.append(f"{path}: unknown class {cls}")
            if cls == "established" and cid in prohibited:
                errors.append(f"{path}: prohibited id {cid} tagged established")
    return errors


def main() -> int:
    prohibited = load_prohibited()
    errors = check_claims(load_ledger(LEDGER), prohibited, str(LEDGER))
    errors.extend(scan_markdown(prohibited))
    ok = CLAIMS / "fixtures" / "ok.json"
    bad = CLAIMS / "fixtures" / "bad-promote.json"
    errors.extend(check_claims(load_ledger(ok), prohibited, str(ok)))
    bad_errors = check_claims(load_ledger(bad), prohibited, str(bad))
    if not bad_errors:
        errors.append(f"{bad}: expected promotion failure")
    if errors:
        print("claim-validation gate failed:")
        for item in errors:
            print(f"  - {item}")
        return 1
    print("claim-validation gate ok")
    print(f"  prohibited ids: {len(prohibited)}")
    print(f"  fixture {bad.name} correctly rejected")
    return 0


if __name__ == "__main__":
    sys.exit(main())
