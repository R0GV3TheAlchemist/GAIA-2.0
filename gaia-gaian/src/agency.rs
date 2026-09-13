//! #75 permissioned agent I/O. Ask-first. No raw biometrics to Earth Twin.

use crate::GaianError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Mail,
    Calendar,
    Research,
    HomeIot,
    CitizenScience,
    GovernanceVote,
}

#[derive(Debug, Default)]
pub struct ScopedAgent {
    grants: Vec<Scope>,
    paused: bool,
    log: Vec<String>,
}

impl ScopedAgent {
    pub fn grant(&mut self, scope: Scope) {
        if !self.grants.contains(&scope) {
            self.grants.push(scope);
        }
    }

    pub fn act(&mut self, scope: Scope) -> Result<(), GaianError> {
        if self.paused {
            self.log.push("paused".into());
            return Err(GaianError::NoConsent);
        }
        if !self.grants.contains(&scope) {
            self.log.push(format!("denied {scope:?}"));
            return Err(GaianError::NoConsent);
        }
        self.log.push(format!("acted {scope:?}"));
        Ok(())
    }

    pub fn pause(&mut self) {
        self.paused = true;
        self.log.push("paused mid-task".into());
    }

    pub fn earth_twin_payload(
        &self,
        include_face: bool,
        include_voice: bool,
        include_health: bool,
    ) -> Result<&'static str, GaianError> {
        if include_face || include_voice || include_health {
            return Err(GaianError::ThirdPartyLikeness);
        }
        Ok("aggregate-only fixture")
    }

    pub fn paid_amplification() -> bool {
        false
    }

    pub fn log(&self) -> &[String] {
        &self.log
    }
}
