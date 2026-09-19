#!/usr/bin/env python3
"""Fail if a PR revives known-invented APIs from slop agents."""
from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BANNED = (
    "Lake::in_memory",
    "MemCubeId",
    "run_bleaching_drill",
)
SKIP_PARTS = {
    "check_invented_symbols.py",
    "AGENT-HYGIENE.md",
    "AGENTS.md",
}


def main() -> int:
    hits: list[str] = []
    for path in ROOT.rglob("*"):
        if not path.is_file():
            continue
        if path.name in SKIP_PARTS:
            continue
        if any(part.startswith(".") and part not in {".github"} for part in path.parts):
            continue
        if path.suffix not in {".rs", ".md", ".py", ".ts", ".json"}:
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError):
            continue
        for token in BANNED:
            if token in text:
                hits.append(f"{path.relative_to(ROOT)}: {token}")
    if hits:
        print("invented symbols (do not land):")
        print("\n".join(hits))
        return 1
    print("invented-symbol list clean")
    return 0


if __name__ == "__main__":
    sys.exit(main())
