//! Operator dormancy envelope (#934). No SOS live signal.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DormancyMode {
    Live,
    Dormant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DormantSystem;

#[derive(Debug, Clone)]
pub struct DormancyGuard {
    mode: DormancyMode,
}

impl Default for DormancyGuard {
    fn default() -> Self {
        Self {
            mode: DormancyMode::Live,
        }
    }
}

impl DormancyGuard {
    pub fn mode(&self) -> DormancyMode {
        self.mode
    }

    pub fn enter_operator(&mut self) {
        self.mode = DormancyMode::Dormant;
    }

    pub fn resume_operator(&mut self) {
        self.mode = DormancyMode::Live;
    }

    pub fn allow_external_call(&self) -> Result<(), DormantSystem> {
        match self.mode {
            DormancyMode::Live => Ok(()),
            DormancyMode::Dormant => Err(DormantSystem),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dormant_rejects_external_calls() {
        let mut g = DormancyGuard::default();
        g.enter_operator();
        assert_eq!(g.allow_external_call(), Err(DormantSystem));
        g.resume_operator();
        g.allow_external_call().expect("resume restores live");
    }
}
