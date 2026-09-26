#!/usr/bin/env python3
"""Negative-control falsification (#845 category 2 / #815 protocol).

A mapping that accepts any of these four corpora is over-fit.
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


def test_schema_lists_markers() -> None:
    body = SCHEMA.read_text(encoding="utf-8").lower()
    for marker in MARKERS:
        assert marker in body


def test_lorem_is_rejected() -> None:
    hits = accepted_mappings((CORPORA / "lorem.txt").read_text(encoding="utf-8"))
    assert hits == [], f"OVER-FIT DETECTED: Mapping succeeded on negative control lorem: {hits}"


def test_aviation_is_rejected() -> None:
    hits = accepted_mappings((CORPORA / "aviation.txt").read_text(encoding="utf-8"))
    assert hits == [], f"OVER-FIT DETECTED: Mapping succeeded on negative control aviation: {hits}"


def test_childrens_story_is_rejected() -> None:
    hits = accepted_mappings((CORPORA / "childrens-story.txt").read_text(encoding="utf-8"))
    assert hits == [], f"OVER-FIT DETECTED: Mapping succeeded on negative control childrens-story: {hits}"


def test_numeric_is_rejected() -> None:
    hits = accepted_mappings((CORPORA / "numeric.txt").read_text(encoding="utf-8"))
    assert hits == [], f"OVER-FIT DETECTED: Mapping succeeded on negative control numeric: {hits}"


def test_hermetic_fixture_is_accepted() -> None:
    hits = accepted_mappings("as above so below, so below as above")
    assert hits == ["as above so below"]
