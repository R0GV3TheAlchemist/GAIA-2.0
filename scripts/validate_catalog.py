#!/usr/bin/env python3
"""Lint docs/knowledge/catalog.json. Stdlib only. Issue #697."""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CATALOG = ROOT / "docs" / "knowledge" / "catalog.json"
ID_RE = re.compile(r"^gaia\.knowledge(\.[a-z0-9]+(-[a-z0-9]+)*)+$")
SLUG_RE = re.compile(r"^[a-z0-9]+(-[a-z0-9]+)*$")
STAGES = {"foundation", "intermediate", "advanced", "mastery"}
TIER_WORDS = ("basic", "intermediate", "mastery")
REQUIRED = ("id", "slug", "title", "kind", "status")
REQUIRED_SEED = {
    "gaia.knowledge.literacy.reading-and-writing",
    "gaia.knowledge.programming",
    "gaia.knowledge.engineering",
    "gaia.knowledge.data-literacy",
}


def err(msg: str) -> None:
    print(f"error: {msg}")


def main() -> int:
    errors = 0
    if not CATALOG.is_file():
        err(f"missing {CATALOG.relative_to(ROOT)}")
        return 1
    try:
        data = json.loads(CATALOG.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        err(f"invalid JSON: {exc}")
        return 1

    if data.get("runtime_enabled") is True:
        err("runtime_enabled must stay false until a loader issue lands")
        errors += 1
    if data.get("id_namespace") != "gaia.knowledge":
        err("id_namespace must be gaia.knowledge")
        errors += 1

    domains = data.get("domains")
    if not isinstance(domains, list) or not domains:
        err("domains must be a non-empty list")
        return 1

    ids: set[str] = set()
    slugs: set[str] = set()
    aliases: dict[str, str] = {}

    for i, row in enumerate(domains):
        loc = f"domains[{i}]"
        if not isinstance(row, dict):
            err(f"{loc} is not an object")
            errors += 1
            continue
        for key in REQUIRED:
            if key not in row:
                err(f"{loc} missing {key}")
                errors += 1
        did = row.get("id", "")
        slug = row.get("slug", "")
        status = row.get("status", "")
        if not ID_RE.match(str(did)):
            err(f"{loc} bad id {did!r}")
            errors += 1
        parts = str(did).split(".")
        if any(part in TIER_WORDS for part in parts):
            err(f"{loc} id encodes a learning tier: {did}")
            errors += 1
        if not SLUG_RE.match(str(slug)):
            err(f"{loc} bad slug {slug!r}")
            errors += 1
        if did in ids:
            err(f"{loc} duplicate id {did}")
            errors += 1
        ids.add(str(did))
        if slug in slugs:
            err(f"{loc} duplicate slug {slug}")
            errors += 1
        slugs.add(str(slug))
        if status not in {"listed", "blocked", "deprecated"}:
            err(f"{loc} bad status {status!r}")
            errors += 1
        if status == "blocked" and not row.get("blocked_reason"):
            err(f"{loc} blocked row needs blocked_reason")
            errors += 1
        if status == "deprecated":
            if not row.get("deprecation_reason"):
                err(f"{loc} deprecated row needs deprecation_reason")
                errors += 1
        for place in row.get("learning_placements") or []:
            stage = place.get("stage")
            if stage not in STAGES:
                err(f"{loc} bad stage {stage!r}")
                errors += 1
        if row.get("legacy") and not row["legacy"].get("repository"):
            err(f"{loc} legacy missing repository")
            errors += 1
        for alias in row.get("aliases") or []:
            if alias in aliases and aliases[alias] != did:
                err(f"{loc} alias {alias!r} collides with {aliases[alias]}")
                errors += 1
            aliases[str(alias)] = str(did)

    missing = REQUIRED_SEED - ids
    if missing:
        err("missing required seed ids: " + ", ".join(sorted(missing)))
        errors += 1

    declared = data.get("counts", {}).get("total")
    if declared != len(domains):
        err(f"counts.total {declared} != len(domains) {len(domains)}")
        errors += 1

    if errors:
        print(f"catalog lint failed ({errors})")
        return 1
    print(f"catalog lint clean ({len(domains)} rows)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
