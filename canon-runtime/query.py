"""Read-only canon-runtime lookup over manifest.json (#896).

No network. No mutation. Unknown ids return None.
Presence in the manifest is not ACP enforcement.
"""

from __future__ import annotations

import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
DEFAULT_MANIFEST = HERE / "manifest.json"


class CanonQuery:
    def __init__(self, manifest_path: Path | None = None) -> None:
        path = manifest_path or DEFAULT_MANIFEST
        payload = json.loads(path.read_text(encoding="utf-8"))
        tablets = payload.get("tablets") or []
        self._by_id = {row["id"]: row for row in tablets}

    def get_tablet(self, tablet_id: str) -> dict | None:
        return self._by_id.get(tablet_id)

    def get_constraints(self, tablet_id: str) -> list[str] | None:
        row = self.get_tablet(tablet_id)
        return None if row is None else list(row["constraints"])

    def get_affordances(self, tablet_id: str) -> list[str] | None:
        row = self.get_tablet(tablet_id)
        return None if row is None else list(row["affordances"])

    def get_prohibitions(self, tablet_id: str) -> list[str] | None:
        row = self.get_tablet(tablet_id)
        return None if row is None else list(row["prohibitions"])

    def get_tablet_by_color(self, hex_code: str) -> dict | None:
        needle = hex_code.strip().upper()
        for row in self._by_id.values():
            if row["color"]["hex"].upper() == needle:
                return row
        return None

    def get_tablets_by_element(self, element: str) -> list[dict]:
        return [row for row in self._by_id.values() if row.get("element") == element]

    def get_tablets_by_stage(self, stage: str) -> list[dict]:
        return [row for row in self._by_id.values() if row.get("stage") == stage]


def getTablet(tablet_id: str, q: CanonQuery | None = None):
    return (q or CanonQuery()).get_tablet(tablet_id)


def getConstraints(tablet_id: str, q: CanonQuery | None = None):
    return (q or CanonQuery()).get_constraints(tablet_id)


def getAffordances(tablet_id: str, q: CanonQuery | None = None):
    return (q or CanonQuery()).get_affordances(tablet_id)


def getProhibitions(tablet_id: str, q: CanonQuery | None = None):
    return (q or CanonQuery()).get_prohibitions(tablet_id)


def getTabletByColor(hex_code: str, q: CanonQuery | None = None):
    return (q or CanonQuery()).get_tablet_by_color(hex_code)


def getTabletsByElement(element: str, q: CanonQuery | None = None):
    return (q or CanonQuery()).get_tablets_by_element(element)


def getTabletsByStage(stage: str, q: CanonQuery | None = None):
    return (q or CanonQuery()).get_tablets_by_stage(stage)
