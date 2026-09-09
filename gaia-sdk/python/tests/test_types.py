from gaia_sdk import GaiaClient, GaiaError
from gaia_sdk.errors import InvalidArgument, NotImplementedCapability
from gaia_sdk.types import AgentSpec, ResourceSpec


def test_intent_admits():
    handle = GaiaClient().intent("hello gaia")
    assert handle.state == "admitted"
    assert handle.intent_id


def test_intent_rejects_empty():
    try:
        GaiaClient().intent("  ")
        raise AssertionError("expected InvalidArgument")
    except InvalidArgument:
        pass


def test_context_and_declare():
    cube = GaiaClient().context("hello")
    assert cube.type == "plaintext"
    rid = GaiaClient().declare(ResourceSpec(name="cpu"))
    assert rid


def test_invoke():
    out = GaiaClient().invoke(AgentSpec(agent_id="a", name="researcher"))
    assert out.startswith("invoked:")


def test_sign_unimplemented():
    try:
        GaiaClient().sign(b"x")
        raise AssertionError("expected NotImplementedCapability")
    except NotImplementedCapability:
        pass


def test_error_hierarchy():
    assert issubclass(InvalidArgument, GaiaError)
