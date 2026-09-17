#!/usr/bin/env python3
"""Persistence helpers for TaskGraph / GAIATrace → Supabase anti-chaos tables."""
from __future__ import annotations
from typing import Any

OPEN_CAP = 50


def execution_allowed(gate_row: dict[str, Any]) -> bool:
    return not bool(gate_row.get("blocked"))


def ingest_allowed(open_count: int) -> bool:
    return open_count < OPEN_CAP


def task_run_payload(intent_type: str, gaian_id: str | None, canon_refs: list[str]) -> dict[str, Any]:
    return {
        "intent_type": intent_type,
        "gaian_id": gaian_id,
        "canon_refs": canon_refs,
        "status": "PENDING",
        "context": {},
    }


def trace_payload(event: str, canon_refs: list[str], inputs: dict, outputs: dict) -> dict[str, Any]:
    return {
        "event": event,
        "canon_refs": canon_refs,
        "inputs": inputs,
        "outputs": outputs,
        "meta": {"proof": "PROOF_ANTI_CHAOS_001"},
    }


def synergy_nodes() -> list[dict[str, Any]]:
    return [
        {"engine_id": "schumann", "depends_on": []},
        {"engine_id": "emotional", "depends_on": []},
        {"engine_id": "synergy", "depends_on": ["schumann", "emotional"]},
    ]
