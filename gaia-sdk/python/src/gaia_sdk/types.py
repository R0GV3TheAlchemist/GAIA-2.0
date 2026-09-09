from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any
from uuid import UUID, uuid4


@dataclass
class Intent:
    goal: str


@dataclass
class TaskHandle:
    intent_id: UUID
    state: str = "admitted"


@dataclass
class MemCube:
    id: UUID
    type: str = "plaintext"
    lifecycle: str = "active"
    content: str | None = None


@dataclass
class AgentSpec:
    agent_id: str
    name: str
    params: dict[str, Any] = field(default_factory=dict)


@dataclass
class ResourceSpec:
    name: str
    kind: str = "generic"


def new_id() -> UUID:
    return uuid4()
