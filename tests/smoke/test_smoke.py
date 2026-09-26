#!/usr/bin/env python3
"""System smoke tests (#845 category 4)."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def test_eighteen_tablets_in_color_map() -> None:
    data = json.loads((ROOT / "docs" / "color" / "color-map.json").read_text(encoding="utf-8"))
    names = [t["name"] for t in data["tablets"]]
    assert len(names) == 18
    assert "Terra" in names
    assert "Amber" in names


def test_tablet_files_exist() -> None:
    data = json.loads((ROOT / "docs" / "color" / "color-map.json").read_text(encoding="utf-8"))
    missing = [t["file"] for t in data["tablets"] if not (ROOT / t["file"]).is_file()]
    assert missing == []


def test_governance_docs_present() -> None:
    for rel in (
        "governance/GAIA_GOVERNANCE.md",
        "docs/tablets/INDEX.md",
        "CONTRIBUTING.md",
    ):
        assert (ROOT / rel).is_file(), rel
