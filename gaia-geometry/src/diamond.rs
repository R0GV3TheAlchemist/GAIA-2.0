//! Diamond of Alignment. Gates only. No moral scalar.

use crate::GeoError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gate {
    Pass,
    Narrow,
    Defer,
    Escalate,
    Refuse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Alignment {
    pub truth: Gate,
    pub care: Gate,
    pub growth: Gate,
    pub balance: Gate,
}

impl Alignment {
    pub fn wisdom(&self) -> Gate {
        for gate in [self.truth, self.care, self.growth, self.balance] {
            if gate == Gate::Refuse {
                return Gate::Refuse;
            }
        }
        for gate in [self.truth, self.care, self.growth, self.balance] {
            if gate == Gate::Escalate {
                return Gate::Escalate;
            }
        }
        for gate in [self.truth, self.care, self.growth, self.balance] {
            if gate == Gate::Defer {
                return Gate::Defer;
            }
        }
        for gate in [self.truth, self.care, self.growth, self.balance] {
            if gate == Gate::Narrow {
                return Gate::Narrow;
            }
        }
        Gate::Pass
    }

    /// Forbidden by #311. Exists so tests can prove the refusal.
    pub fn score(&self) -> Result<f64, GeoError> {
        Err(GeoError::AggregateScoreForbidden)
    }

    pub fn refuse_if_no_evidence(has_evidence: bool) -> Result<Gate, GeoError> {
        if !has_evidence {
            return Err(GeoError::MissingEvidence);
        }
        Ok(Gate::Pass)
    }

    pub fn refuse_if_no_steward(named: bool) -> Result<Gate, GeoError> {
        if !named {
            return Err(GeoError::MissingSteward);
        }
        Ok(Gate::Pass)
    }
}
