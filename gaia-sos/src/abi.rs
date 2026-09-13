//! #195 Intent ABI. Userspace first.

use crate::SosError;

pub const HOST_CALLS: [&str; 8] = [
    "intent", "context", "invoke", "observe", "sign", "verify", "declare", "learn",
];

pub fn submit_intent(signed: bool, cap: bool) -> Result<(), SosError> {
    if !signed {
        return Err(SosError::Unsigned);
    }
    if !cap {
        return Err(SosError::NoCapability);
    }
    Ok(())
}

pub fn learn(rewrite_weights: bool) -> Result<(), SosError> {
    if rewrite_weights {
        return Err(SosError::WeightRewrite);
    }
    Ok(())
}
