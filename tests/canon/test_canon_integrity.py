#!/usr/bin/env python3
"""
GAIA 2.0 Canon Integrity Test Suite
Issue: #841
Governing Tablet: Obsidian — The Law of Boundaries, What Must Not Be Crossed
Governing Document: governance/GAIA_GOVERNANCE.md Rule 4

Tests:
  T-001  Hex collision — no two tablets share the same hex
  T-002  INDEX.md sync — color-map.json matches INDEX.md hex values
  T-003  Tablet file completeness — every tablet's .md file exists on disk
  T-004  Required fields — sealed tablets contain all required metadata
  T-005  Naming conventions — canonical spellings used across canon files

Regression locks:
  RL-001  Terra hex is Bistre (#3D2B1F), resolves Issue #831

Usage:
  python tests/canon/test_canon_integrity.py
  pytest tests/canon/test_canon_integrity.py -v
"""

import json
import re
import sys
from pathlib import Path

# ---------------------------------------------------------------------------
# Paths — all relative to repository root
# ---------------------------------------------------------------------------
REPO_ROOT = Path(__file__).resolve().parents[2]
COLOR_MAP = REPO_ROOT / "docs" / "color" / "color-map.json"
INDEX_MD  = REPO_ROOT / "docs" / "tablets" / "INDEX.md"

# Canon directories scanned by T-005
CANON_SCAN_DIRS = [
    REPO_ROOT / "docs" / "tablets",
    REPO_ROOT / "governance",
    REPO_ROOT / "proofs",
]

# Required fields that must appear in every SEALED tablet .md file
REQUIRED_SEALED_FIELDS = [
    "Sealed",
    "Author",
    "Governing Color",
    "Revision History",
]

# ---------------------------------------------------------------------------
# Naming Rules
# ---------------------------------------------------------------------------
# Each entry: (regex_pattern, canonical_form, description, exempt_fn)
# exempt_fn(line, filepath) -> bool: return True to skip this line/file.
# Use exempt_fn=None for rules with no exceptions.

def _amber_file_or_line(line: str, filepath: Path) -> bool:
    """
    Exempt lines where #8B4513 appears alongside 'Amber' on the same line,
    OR the file is the Amber tablet itself.
    #8B4513 is Amber's legitimate canonical hex.
    """
    if "AMBER_TABLET" in filepath.name:
        return True  # entire Amber tablet file is exempt for this hex
    if re.search(r'(?i)amber', line):
        return True  # cross-reference lines that name Amber alongside the hex
    return False

NAMING_RULES = [
    # pattern                      canonical              description                              exempt_fn
    # --- GAIA casing ---
    # Correct forms: 'GAIA', 'GAIA 2.0' — exempt both
    (r'\bGAIA\b',                  None,   None,                                                   None),   # correct
    (r'\bGAIA 2\.0\b',             None,   None,                                                   None),   # correct
    (r'\bGaia\b',                  "GAIA", "'Gaia' should be 'GAIA'",                              None),
    (r'\bgaia\b',                  "GAIA", "'gaia' should be 'GAIA' or 'GAIA 2.0'",               None),
    # --- Terra color name ---
    (r'Terra Brown',               "Bistre", "'Terra Brown' is obsolete — use 'Bistre'",           None),
    # --- Old Terra hex: exempt if on an Amber line or in the Amber tablet file ---
    (r'#8B4513',  "#3D2B1F (Bistre)",  "Old Terra hex #8B4513 found outside Amber context — use #3D2B1F",  _amber_file_or_line),
]

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def load_color_map() -> dict:
    """Load and return the parsed color-map.json."""
    if not COLOR_MAP.exists():
        raise FileNotFoundError(f"COLOR MAP MISSING: {COLOR_MAP}")
    with COLOR_MAP.open() as f:
        return json.load(f)


def parse_index_hex_table(index_path: Path) -> dict[str, str]:
    """
    Parse hex values from INDEX.md's tablet registry table.

    INDEX.md table format (actual):
      | 01 | [Amber Tablet](...) | Amber `#8B4513` | The Law... | ... |

    The color cell contains: ColorName `#HEXVALUE`
    We match the backtick-wrapped hex and pair it with the tablet name
    extracted from the markdown link in column 2.

    Returns {tablet_name: hex_value} e.g. {"Amber": "#8B4513", ...}
    """
    hexes: dict[str, str] = {}
    text = index_path.read_text(encoding="utf-8")

    # Match table data rows (skip header and separator rows)
    # Row pattern: | number | [Name Tablet](...) | ... `#HEX` ... | ...
    row_re = re.compile(
        r'^\|\s*\d+\s*'                          # | number |
        r'\|\s*\[([A-Za-z]+)\s+Tablet\]'        # | [Name Tablet](...)
        r'[^|]*'                                  # rest of link cell
        r'\|[^|]*`(#[0-9A-Fa-f]{3,8})`',        # | ... `#HEX` ...
        re.MULTILINE
    )

    for match in row_re.finditer(text):
        name    = match.group(1).strip()           # e.g. "Amber"
        hex_val = match.group(2).strip().upper()   # e.g. "#8B4513"
        hexes[name] = hex_val

    return hexes


class CanonTestResult:
    def __init__(self, test_id: str, name: str):
        self.test_id = test_id
        self.name = name
        self.failures: list[str] = []
        self.passed = True

    def fail(self, message: str):
        self.failures.append(message)
        self.passed = False

    def report(self):
        status = "PASS" if self.passed else "FAIL"
        print(f"  [{status}] {self.test_id}: {self.name}")
        for msg in self.failures:
            print(f"         \u2717 {msg}")


# ---------------------------------------------------------------------------
# T-001 — Hex Collision Test
# ---------------------------------------------------------------------------

def t001_hex_collision(data: dict) -> CanonTestResult:
    """
    Assert no two tablets share the same hex value.
    Every tablet must have a unique color signature.
    """
    result = CanonTestResult("T-001", "Hex Collision")
    seen: dict[str, str] = {}  # hex -> tablet name

    for tablet in data["tablets"]:
        name    = tablet["name"]
        hex_val = tablet["hex"].upper()
        if hex_val in seen:
            result.fail(
                f"COLLISION: '{name}' and '{seen[hex_val]}' both use hex {hex_val}"
            )
        else:
            seen[hex_val] = name

    return result


# ---------------------------------------------------------------------------
# T-002 — INDEX.md Sync Test
# ---------------------------------------------------------------------------

def t002_index_sync(data: dict) -> CanonTestResult:
    """
    Assert color-map.json hex values match INDEX.md exactly (case-insensitive).
    INDEX.md is the authoritative source; any divergence must be surfaced.
    """
    result = CanonTestResult("T-002", "INDEX.md Sync")

    if not INDEX_MD.exists():
        result.fail(f"INDEX MISSING: {INDEX_MD} not found — cannot verify sync")
        return result

    index_hexes = parse_index_hex_table(INDEX_MD)

    if not index_hexes:
        result.fail(
            "INDEX PARSE FAILURE: No hex values found in INDEX.md — "
            "expected rows like: | 01 | [Name Tablet](...) | Color `#HEXVAL` | ..."
        )
        return result

    for tablet in data["tablets"]:
        name    = tablet["name"]
        map_hex = tablet["hex"].upper()

        if name not in index_hexes:
            # Non-fatal for unsealed tablets — they may not yet appear in the table
            # Sealed tablets must be in INDEX
            if tablet.get("sealed"):
                result.fail(
                    f"MISSING IN INDEX: '{name}' is sealed but not found in INDEX.md table"
                )
            continue

        idx_hex = index_hexes[name].upper()
        if map_hex != idx_hex:
            result.fail(
                f"SYNC DRIFT: '{name}' — INDEX.md={idx_hex}, color-map.json={map_hex}"
            )

    return result


# ---------------------------------------------------------------------------
# T-003 — Tablet File Completeness Test
# ---------------------------------------------------------------------------

def t003_file_completeness(data: dict) -> CanonTestResult:
    """
    Assert the .md file listed for every tablet in color-map.json exists on disk.
    """
    result = CanonTestResult("T-003", "Tablet File Completeness")

    for tablet in data["tablets"]:
        name          = tablet["name"]
        relative_path = tablet.get("file", "")
        if not relative_path:
            result.fail(f"NO FILE FIELD: '{name}' has no 'file' key in color-map.json")
            continue
        full_path = REPO_ROOT / relative_path
        if not full_path.exists():
            result.fail(f"MISSING FILE: '{name}' \u2192 expected {relative_path}")

    return result


# ---------------------------------------------------------------------------
# T-004 — Required Fields on Sealed Tablets
# ---------------------------------------------------------------------------

def t004_required_fields(data: dict) -> CanonTestResult:
    """
    Assert every sealed tablet's .md file contains all required metadata fields.
    Unsealed tablets are exempt — they are works in progress.
    """
    result = CanonTestResult("T-004", "Required Fields (Sealed Tablets)")

    for tablet in data["tablets"]:
        if not tablet.get("sealed"):
            continue

        name          = tablet["name"]
        relative_path = tablet.get("file", "")
        if not relative_path:
            continue

        full_path = REPO_ROOT / relative_path
        if not full_path.exists():
            result.fail(
                f"SEALED BUT MISSING: '{name}' is sealed (since {tablet['sealed']}) "
                f"but file {relative_path} does not exist"
            )
            continue

        content = full_path.read_text(encoding="utf-8")
        for field in REQUIRED_SEALED_FIELDS:
            if field not in content:
                result.fail(
                    f"MISSING FIELD: '{name}' is sealed but '{field}' "
                    f"not found in {relative_path}"
                )

    return result


# ---------------------------------------------------------------------------
# T-005 — Naming Convention Test
# ---------------------------------------------------------------------------

def t005_naming_conventions() -> CanonTestResult:
    """
    Scan canon files for known bad patterns and assert canonical spellings are used.
    Scans: docs/tablets/, governance/, proofs/

    Rules with exempt_fn: if exempt_fn(line, filepath) returns True, the
    match is skipped. Used for cases where the same string is legitimate
    in one context but a drift signal in another (e.g. #8B4513 is correct
    for Amber but wrong everywhere else).
    """
    result = CanonTestResult("T-005", "Naming Conventions")

    for scan_dir in CANON_SCAN_DIRS:
        if not scan_dir.exists():
            continue
        for md_file in scan_dir.rglob("*.md"):
            try:
                content = md_file.read_text(encoding="utf-8")
            except Exception:
                continue
            lines = content.splitlines()
            for line_no, line in enumerate(lines, start=1):
                for rule in NAMING_RULES:
                    pattern, canonical, description, exempt_fn = rule
                    if canonical is None:
                        continue  # exempt — this is a correct form
                    if not re.search(pattern, line):
                        continue
                    if exempt_fn and exempt_fn(line, md_file):
                        continue  # context-specific exemption
                    rel = md_file.relative_to(REPO_ROOT)
                    result.fail(
                        f"NAMING DRIFT: {rel} line {line_no} — {description}"
                    )

    return result


# ---------------------------------------------------------------------------
# RL-001 — Regression Lock: Terra Hex = Bistre #3D2B1F (Issue #831)
# ---------------------------------------------------------------------------

def rl001_terra_bistre_regression(data: dict) -> CanonTestResult:
    """
    Permanent regression lock for Issue #831.
    Terra's hex must be Bistre (#3D2B1F), never the old Amber Brown (#8B4513).
    Terra and Amber must never share a hex value.
    """
    result = CanonTestResult("RL-001", "Terra/Bistre Regression Lock (Issue #831)")

    terra = next((t for t in data["tablets"] if t["name"] == "Terra"), None)
    amber = next((t for t in data["tablets"] if t["name"] == "Amber"), None)

    if terra is None:
        result.fail("REGRESSION: 'Terra' tablet not found in color-map.json")
        return result

    terra_hex = terra["hex"].upper()
    bistre    = "#3D2B1F"
    old_hex   = "#8B4513"

    if terra_hex == old_hex.upper():
        result.fail(
            f"REGRESSION #831: Terra hex reverted to old Amber Brown {old_hex} — "
            f"must be Bistre {bistre}"
        )
    elif terra_hex != bistre.upper():
        result.fail(
            f"REGRESSION #831: Terra hex is {terra_hex}, expected Bistre {bistre}"
        )

    if amber and terra["hex"].upper() == amber["hex"].upper():
        result.fail(
            f"REGRESSION #831: Terra ({terra['hex']}) and Amber ({amber['hex']}) "
            f"hex collision has returned"
        )

    if terra.get("color_name", "").lower() != "bistre":
        result.fail(
            f"REGRESSION #831: Terra color_name is '{terra.get('color_name')}', "
            f"expected 'Bistre'"
        )

    return result


# ---------------------------------------------------------------------------
# Runner
# ---------------------------------------------------------------------------

def run_all() -> int:
    """
    Run all canon integrity tests.
    Returns 0 if all pass, 1 if any fail.
    """
    print()
    print("=" * 60)
    print("  GAIA 2.0 — Canon Integrity Test Suite (#841)")
    print("  Governing Tablet: Obsidian")
    print("  Source of Truth: docs/color/color-map.json")
    print("=" * 60)
    print()

    try:
        data = load_color_map()
    except FileNotFoundError as e:
        print(f"  [FATAL] {e}")
        return 1

    results = [
        t001_hex_collision(data),
        t002_index_sync(data),
        t003_file_completeness(data),
        t004_required_fields(data),
        t005_naming_conventions(),
        rl001_terra_bistre_regression(data),
    ]

    print("  Results:")
    print()
    for r in results:
        r.report()

    total  = len(results)
    passed = sum(1 for r in results if r.passed)
    failed = total - passed

    print()
    print("-" * 60)
    print(f"  {passed}/{total} tests passed", end="")
    if failed:
        print(f"  |  {failed} FAILED")
    else:
        print("  — All canon boundaries intact.")
    print("-" * 60)
    print()

    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(run_all())
