#!/usr/bin/env python3
"""Admit a stub intent through the Phase 0 SDK."""
from gaia_sdk import GaiaClient

client = GaiaClient()
handle = client.intent("Say hello from the GAIA 2.0 developer profile")
print(handle)
cube = client.context("hello")
print(cube)
