#!/usr/bin/env python3
from ingest_github_work_items import plan_batch, OPEN_CAP
from gaia_persist import execution_allowed, ingest_allowed, synergy_nodes

assert OPEN_CAP == 50
assert plan_batch([190, 999], already_mirrored=50) == []
assert ingest_allowed(50) is False
assert ingest_allowed(49) is True
assert execution_allowed({"blocked": False}) is True
assert execution_allowed({"blocked": True}) is False
assert [n["engine_id"] for n in synergy_nodes()] == ["schumann", "emotional", "synergy"]
print("ok")
