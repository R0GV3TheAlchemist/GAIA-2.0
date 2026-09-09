class GaiaError(Exception):
    """Base error for the GAIA SDK."""


class InvalidArgument(GaiaError):
    pass


class Denied(GaiaError):
    pass


class NotImplementedCapability(GaiaError):
    pass
