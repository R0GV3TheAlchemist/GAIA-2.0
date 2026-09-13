//! #63 permissioned agent stub. Not GAIAN v1.0 and not C2PA.

use crate::GaianError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentAct {
    Email,
    Pay,
    PublishLikeness,
    Ask,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Grant {
    pub email: bool,
    pub pay: bool,
    pub publish_likeness: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Agent {
    pub grant: Grant,
    pub revoked: bool,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            grant: Grant::default(),
            revoked: false,
        }
    }

    pub fn act(&self, act: AgentAct) -> Result<(), GaianError> {
        if self.revoked {
            return Err(GaianError::Revoked);
        }
        match act {
            AgentAct::Ask => Ok(()),
            AgentAct::Email if self.grant.email => Ok(()),
            AgentAct::Pay if self.grant.pay => Ok(()),
            AgentAct::PublishLikeness if self.grant.publish_likeness => Ok(()),
            _ => Err(GaianError::GrantRequired),
        }
    }

    pub fn revoke(&mut self) {
        self.revoked = true;
        self.grant = Grant::default();
    }
}

pub fn gaian_release_checklist() -> [&'static str; 4] {
    [
        "child-safety tests",
        "non-impersonation tests",
        "revoke stops the agent",
        "no GAIAN v1.0 tag",
    ]
}
