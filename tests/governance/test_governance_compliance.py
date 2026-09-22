#!/usr/bin/env python3
"""
GAIA 2.0 Governance Compliance Test Suite
Issue: #843
Governing Tablet: Lapis — The Law of Sovereignty, The Right to Self-Govern
Governing Document: governance/GAIA_GOVERNANCE.md Part III

Tests:
  T-006  Audit Log Format       — every Session block has all 7 required fields
  T-007  Governance Version     — GAIA_GOVERNANCE.md Revision History is well-formed
  T-008  Session Init Currency  — SESSION_INIT Section 2 hex values match color-map.json
  T-009  Decision Log Presence  — SESSION_INIT Section 5 is parseable and non-empty

Usage:
  python tests/governance/test_governance_compliance.py
  pytest tests/governance/test_governance_compliance.py -v
"""

import json
import os
import re
import sys
from pathlib import Path

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------
REPO_ROOT    = Path(__file__).resolve().parents[2]
AUDIT_LOG    = REPO_ROOT / "governance" / "GAIA_AUDIT_LOG.md"
GOVERNANCE   = REPO_ROOT / "governance" / "GAIA_GOVERNANCE.md"
SESSION_INIT = REPO_ROOT / "governance" / "GAIA_SESSION_INIT.md"
COLOR_MAP    = REPO_ROOT / "docs" / "color" / "color-map.json"

# ---------------------------------------------------------------------------
# T-006 constants
# ---------------------------------------------------------------------------
# Exact bold-field prefixes as they appear in GAIA_AUDIT_LOG.md entries.
# Source of truth: GAIA_GOVERNANCE.md Part III audit entry template.
AUDIT_REQUIRED_FIELDS = [
    "**Actions Taken:**",
    "**Actions Rejected:**",
    "**Risks Identified:**",
    "**Known Unknowns:**",
    "**Human Review Required:**",
    "**PRs Opened This Session:**",
    "**Issues Created This Session:**",
]

# ---------------------------------------------------------------------------
# Section-heading separator pattern
# ---------------------------------------------------------------------------
# GAIA documents use the Unicode em-dash (—, U+2014) as the separator
# in section headings: "## Section 2 — Current Canon State".
# A plain hyphen-minus (-) is accepted as a fallback.
#
# IMPORTANT: [—-] is an INVALID regex character class range because
# U+002D (hyphen) < U+2014 (em-dash). Python's re silently mishandles
# it rather than raising an error. Always use (?:—|-) instead.
SEP = r'(?:—|-)'

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def load_color_map() -> dict:
    if not COLOR_MAP.exists():
        raise FileNotFoundError(f"COLOR MAP MISSING: {COLOR_MAP}")
    with COLOR_MAP.open() as f:
        return json.load(f)


class GovTestResult:
    def __init__(self, test_id: str, name: str):
        self.test_id  = test_id
        self.name     = name
        self.failures: list[str] = []
        self.passed   = True

    def fail(self, message: str):
        self.failures.append(message)
        self.passed = False

    def report(self):
        status = "PASS" if self.passed else "FAIL"
        print(f"  [{status}] {self.test_id}: {self.name}")
        for msg in self.failures:
            print(f"         \u2717 {msg}")


# ---------------------------------------------------------------------------
# T-006 — Audit Log Format Test
# ---------------------------------------------------------------------------

def t006_audit_log_format() -> GovTestResult:
    """
    Parse every Session block in GAIA_AUDIT_LOG.md.
    Assert all 7 required fields are present in each block.

    Session blocks are delimited by `### Session:` headings.
    A block ends at the next `### Session:` heading or end-of-file.
    """
    result = GovTestResult("T-006", "Audit Log Format")

    if not AUDIT_LOG.exists():
        result.fail(f"AUDIT LOG MISSING: {AUDIT_LOG} not found")
        return result

    text  = AUDIT_LOG.read_text(encoding="utf-8")
    lines = text.splitlines()

    session_starts: list[tuple[str, int]] = []
    for i, line in enumerate(lines):
        m = re.match(r'^###\s+Session:\s*(.+)', line.strip())
        if m:
            session_starts.append((m.group(1).strip(), i))

    if not session_starts:
        result.fail("AUDIT LOG EMPTY: No `### Session:` blocks found in GAIA_AUDIT_LOG.md")
        return result

    for idx, (label, start_line) in enumerate(session_starts):
        end_line = session_starts[idx + 1][1] if idx + 1 < len(session_starts) else len(lines)
        block    = "\n".join(lines[start_line:end_line])

        for field in AUDIT_REQUIRED_FIELDS:
            if field not in block:
                result.fail(
                    f"AUDIT VIOLATION: Session '{label}' missing field {field}"
                )

    return result


# ---------------------------------------------------------------------------
# T-007 — Governance Version Integrity Test
# ---------------------------------------------------------------------------

def t007_governance_version() -> GovTestResult:
    """
    Validate GAIA_GOVERNANCE.md Revision History table is well-formed
    and the **Version:** header matches the most recent table row.
    """
    result = GovTestResult("T-007", "Governance Version Integrity")

    if not GOVERNANCE.exists():
        result.fail(f"GOVERNANCE MISSING: {GOVERNANCE} not found")
        return result

    text = GOVERNANCE.read_text(encoding="utf-8")

    version_header_m = re.search(r'^\*\*Version:\*\*\s*([\d.]+)', text, re.MULTILINE)
    if not version_header_m:
        result.fail("GOVERNANCE DRIFT: No **Version:** field found in GAIA_GOVERNANCE.md")
        return result
    header_version = version_header_m.group(1).strip()

    rev_section_m = re.search(
        rf'##\s+Part\s+VI\s+{SEP}\s+Revision History(.+?)(?=^##|\Z)',
        text, re.MULTILINE | re.DOTALL
    )
    if not rev_section_m:
        result.fail(
            "GOVERNANCE DRIFT: 'Part VI — Revision History' section not found "
            "in GAIA_GOVERNANCE.md"
        )
        return result

    rev_section = rev_section_m.group(1)
    row_re      = re.compile(r'^\|\s*([\d.]+)\s*\|', re.MULTILINE)
    rows        = row_re.findall(rev_section)

    if not rows:
        result.fail(
            "GOVERNANCE DRIFT: Revision History table has no version data rows "
            "in GAIA_GOVERNANCE.md"
        )
        return result

    latest_row_version = rows[-1].strip()
    if header_version != latest_row_version:
        result.fail(
            f"GOVERNANCE DRIFT: **Version:** header is {header_version} but "
            f"latest Revision History row is {latest_row_version} — they must match"
        )

    return result


# ---------------------------------------------------------------------------
# T-008 — Session Init Currency Test
# ---------------------------------------------------------------------------

def t008_session_init_currency(data: dict) -> GovTestResult:
    """
    Parse Section 2 tablet table in GAIA_SESSION_INIT.md.
    For every sealed tablet in color-map.json, assert it is present
    in the Section 2 table and its hex matches.
    """
    result = GovTestResult("T-008", "Session Init Currency")

    if not SESSION_INIT.exists():
        result.fail(f"SESSION INIT MISSING: {SESSION_INIT} not found")
        return result

    text = SESSION_INIT.read_text(encoding="utf-8")

    # Extract Section 2 block.
    # Uses SEP = (?:—|-) to avoid the invalid [—-] character-class range.
    section2_m = re.search(
        rf'##\s+Section\s+2\s+{SEP}\s+Current Canon State(.+?)(?=^##|\Z)',
        text, re.MULTILINE | re.DOTALL
    )
    if not section2_m:
        result.fail(
            "SESSION INIT STALE: Section 2 'Current Canon State' not found "
            "in GAIA_SESSION_INIT.md"
        )
        return result

    section2 = section2_m.group(1)

    # Parse tablet rows: | 01 | Amber | `#8B4513` | ... |
    row_re = re.compile(
        r'^\|\s*\d+\s*\|\s*([A-Za-z]+)\s*\|\s*`(#[0-9A-Fa-f]{3,8})`',
        re.MULTILINE
    )
    init_hexes: dict[str, str] = {}
    for m in row_re.finditer(section2):
        init_hexes[m.group(1).strip()] = m.group(2).strip().upper()

    if not init_hexes:
        result.fail(
            "SESSION INIT STALE: No tablet rows parsed from Section 2 table "
            "in GAIA_SESSION_INIT.md — expected format: | ## | Name | `#HEX` | ..."
        )
        return result

    for tablet in data["tablets"]:
        name      = tablet["name"]
        map_hex   = tablet["hex"].upper()
        is_sealed = bool(tablet.get("sealed"))

        if name not in init_hexes:
            if is_sealed:
                result.fail(
                    f"SESSION INIT STALE: Sealed tablet '{name}' not present "
                    f"in GAIA_SESSION_INIT.md Section 2 table"
                )
            continue

        if map_hex != init_hexes[name]:
            result.fail(
                f"SESSION INIT STALE: '{name}' hex INIT={init_hexes[name]} "
                f"color-map={map_hex} — SESSION_INIT.md Section 2 needs updating"
            )

    return result


# ---------------------------------------------------------------------------
# T-009 — Decision Log Presence Test
# ---------------------------------------------------------------------------

def t009_decision_log_presence() -> GovTestResult:
    """
    Validate Section 5 Decision Log in GAIA_SESSION_INIT.md is
    parseable and non-empty. In CI mode (CHANGED_FILES env var set),
    assert that any PR touching canon files also updates SESSION_INIT.
    """
    result = GovTestResult("T-009", "Decision Log Presence")

    if not SESSION_INIT.exists():
        result.fail(f"SESSION INIT MISSING: {SESSION_INIT} not found")
        return result

    text = SESSION_INIT.read_text(encoding="utf-8")

    section5_m = re.search(
        rf'##\s+Section\s+5\s+{SEP}\s+Decision Log(.+?)(?=^##|\Z)',
        text, re.MULTILINE | re.DOTALL
    )
    if not section5_m:
        result.fail(
            "DECISION UNLOGGED: Section 5 'Decision Log' not found "
            "in GAIA_SESSION_INIT.md"
        )
        return result

    section5 = section5_m.group(1)
    row_re   = re.compile(r'^\|\s*(\d{4}-\d{2}-\d{2})\s*\|', re.MULTILINE)
    rows     = row_re.findall(section5)

    if not rows:
        result.fail(
            "DECISION UNLOGGED: Section 5 Decision Log table has no data rows "
            "in GAIA_SESSION_INIT.md — every architectural decision must be logged"
        )
        return result

    # CI mode: if CHANGED_FILES env var is set, enforce the invariant that
    # any canon file change is accompanied by a SESSION_INIT update.
    changed_files_env = os.environ.get("CHANGED_FILES", "")
    if changed_files_env:
        changed = set(changed_files_env.split())
        canon_changed = any(
            "docs/tablets/" in f or "INDEX.md" in f
            for f in changed
        )
        session_init_changed = any("GAIA_SESSION_INIT.md" in f for f in changed)
        if canon_changed and not session_init_changed:
            for cf in (f for f in changed if "docs/tablets/" in f or "INDEX.md" in f):
                result.fail(
                    f"DECISION UNLOGGED: '{cf}' was changed but "
                    f"GAIA_SESSION_INIT.md Section 5 was not updated "
                    f"— every canon change requires a decision log entry"
                )

    return result


# ---------------------------------------------------------------------------
# Runner
# ---------------------------------------------------------------------------

def run_all() -> int:
    print()
    print("=" * 60)
    print("  GAIA 2.0 — Governance Compliance Test Suite (#843)")
    print("  Governing Tablet: Lapis")
    print("  Source of Truth: governance/GAIA_GOVERNANCE.md Part III")
    print("=" * 60)
    print()

    try:
        data = load_color_map()
    except FileNotFoundError as e:
        print(f"  [FATAL] {e}")
        return 1

    results = [
        t006_audit_log_format(),
        t007_governance_version(),
        t008_session_init_currency(data),
        t009_decision_log_presence(),
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
        print("  — All governance obligations intact.")
    print("-" * 60)
    print()

    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(run_all())
