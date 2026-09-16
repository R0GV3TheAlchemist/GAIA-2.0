#!/usr/bin/env python3
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TESTS = ROOT / "tests"
EXAMPLES = ROOT / "examples"
REQUIRED = {
    "id",
    "publication_status",
    "epistemic_status",
    "sources",
    "interpretation",
    "diamond",
    "uncertainty",
    "unfinished_horizon",
    "authority",
}
ALLOWED = REQUIRED | {
    "ukd_node_id",
    "library_ring",
    "learning_path",
    "claim",
    "application",
    "symbology",
    "dissent",
    "correction_path",
    "viriditas",
}
PUBLICATION = {
    "private-draft",
    "approved-for-public-spec",
    "approved-for-public-example",
    "restricted",
    "community-governed",
    "do-not-store",
}
EPISTEMIC = {
    "observation",
    "data",
    "information",
    "evidence",
    "knowledge_claim",
    "interpretation",
    "understanding",
    "wisdom_judgment",
    "stewardship_application",
}
AUTH = {"advisory_only", "requires_community_authority", "not_authorized_to_store"}
CONSENT = {"not_required", "valid", "missing", "refused"}
SOURCE_KIND = {
    "observation",
    "document",
    "dataset",
    "community_authority",
    "user_declared",
    "historical",
    "fictional",
}
DIAMOND = ("truth", "memory", "discovery", "understanding", "wisdom")


def fail(message):
    raise ValueError(message)


def validate(data, name):
    unknown = set(data) - ALLOWED
    if unknown:
        fail(f"{name}: unknown top-level fields: {sorted(unknown)}")
    missing = REQUIRED - set(data)
    if missing:
        fail(f"{name}: missing top-level fields: {sorted(missing)}")
    if data["publication_status"] not in PUBLICATION:
        fail(f"{name}: invalid publication_status")
    if data["epistemic_status"] not in EPISTEMIC:
        fail(f"{name}: invalid epistemic_status")
    sources = data["sources"]
    if not isinstance(sources, list) or not sources:
        fail(f"{name}: sources must be a non-empty array")
    for i, source in enumerate(sources):
        if not source.get("label") or source.get("kind") not in SOURCE_KIND:
            fail(f"{name}: sources[{i}] requires label and valid kind")
    if not str(data["interpretation"]).strip():
        fail(f"{name}: interpretation must not be empty")
    diamond = data["diamond"]
    for key in DIAMOND:
        if not diamond.get(key):
            fail(f"{name}: missing diamond.{key}")
    if not str(data["uncertainty"]).strip() or not str(data["unfinished_horizon"]).strip():
        fail(f"{name}: uncertainty and unfinished_horizon must not be empty")
    authority = data["authority"]
    if authority.get("status") not in AUTH:
        fail(f"{name}: invalid authority.status")
    if authority.get("consent_status") not in CONSENT:
        fail(f"{name}: invalid authority.consent_status")
    if not authority.get("community_authority"):
        fail(f"{name}: missing authority.community_authority")
    symbology = data.get("symbology")
    if symbology:
        if "category" not in symbology or "operational_claim" not in symbology:
            fail(f"{name}: symbology requires category and operational_claim")
        if symbology["operational_claim"] and not symbology.get("accountability_mapping"):
            fail(f"{name}: operational_claim requires accountability_mapping")
    dumped = json.dumps(data).lower()
    if "complete knowledge" in dumped:
        fail(f"{name}: Unfinished Horizon forbids complete-knowledge language")


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
    for name in (
        "missing-sources.json",
        "operational-symbol-without-mapping.json",
        "prohibited-wisdom-score.json",
    ):
        expect(TESTS / name, False)
    for path in sorted(EXAMPLES.glob("*.json")):
        expect(path, True)


if __name__ == "__main__":
    main()
