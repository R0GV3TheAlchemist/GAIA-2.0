//! #105 tool session + sandbox. No host FS. No GAIAN biometrics.

use crate::AikdError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCube {
    pub input: String,
    pub output: String,
    pub signed: bool,
}

impl ToolCube {
    pub fn record(input: &str, output: &str, signed: bool) -> Result<Self, AikdError> {
        if !signed {
            return Err(AikdError::NeedVerify);
        }
        if output.to_ascii_lowercase().contains("face mesh") {
            return Err(AikdError::CannotKnow);
        }
        Ok(Self {
            input: input.into(),
            output: output.into(),
            signed: true,
        })
    }
}

pub fn sandbox_breakout(host_fs: bool) -> Result<(), AikdError> {
    if host_fs {
        return Err(AikdError::NeedVerify);
    }
    Ok(())
}

pub fn embodied_enabled() -> bool {
    false
}
