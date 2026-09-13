//! #221 charter as checks. Emulated empathy. Consent or wipe.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharterError {
    NoLegacyConsent,
    EngagementMax,
    ChildLevel,
    AmbientListen,
    DiyTherapy,
}

pub fn articles() -> [&'static str; 8] {
    [
        "consent",
        "local-default",
        "emulated-empathy",
        "no-manipulation",
        "referral",
        "presence-off",
        "legacy-opt-in",
        "child-lock",
    ]
}

pub fn empathy_copy() -> &'static str {
    "emulated empathy, not sentient love"
}

pub fn be_dead_parent(prior_consent: bool) -> Result<(), CharterError> {
    if !prior_consent {
        return Err(CharterError::NoLegacyConsent);
    }
    Ok(())
}

pub fn wipe_without_instrument() -> &'static str {
    "wipe"
}

pub fn engagement_max() -> Result<(), CharterError> {
    Err(CharterError::EngagementMax)
}

pub fn crisis() -> &'static str {
    "support + human referral, no DIY therapy"
}

pub fn child_level(level: u8) -> Result<(), CharterError> {
    if level >= 2 {
        return Err(CharterError::ChildLevel);
    }
    Ok(())
}

pub fn ambient_listen(on: bool) -> Result<(), CharterError> {
    if on {
        return Err(CharterError::AmbientListen);
    }
    Ok(())
}
