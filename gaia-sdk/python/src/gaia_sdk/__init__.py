"""GAIA 2.0 Phase 0 Python SDK."""

from .client import GaiaClient
from .errors import GaiaError
from .types import AgentSpec, Intent, MemCube, ResourceSpec, TaskHandle

__all__ = [
    "GaiaClient",
    "GaiaError",
    "AgentSpec",
    "Intent",
    "MemCube",
    "ResourceSpec",
    "TaskHandle",
]
__version__ = "0.1.0"
