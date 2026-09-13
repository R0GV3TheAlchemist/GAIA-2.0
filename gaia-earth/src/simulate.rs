//! #37 What-Now, What-Next, What-If.
//! Fixture distributions. Not a GCM, SSP runner, or tipping cascade.

use crate::SourceKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimMode {
    WhatNow,
    WhatNext,
    WhatIf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeKind {
    Climate,
    Biodiversity,
    Economy,
    Welfare,
    TippingRisk,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Distribution {
    pub kind: OutcomeKind,
    pub mean: f64,
    pub uncertainty: f64,
    pub source: SourceKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioRun {
    pub id: String,
    pub mode: SimMode,
    pub outcomes: Vec<Distribution>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimError {
    UnknownScenario,
    MissingUncertainty,
}

impl std::fmt::Display for SimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownScenario => write!(f, "unknown scenario"),
            Self::MissingUncertainty => write!(f, "uncertainty is mandatory"),
        }
    }
}

pub struct ScenarioEngine {
    library: Vec<(&'static str, SimMode)>,
}

impl Default for ScenarioEngine {
    fn default() -> Self {
        Self {
            library: vec![
                ("state-now", SimMode::WhatNow),
                ("seasonal-next", SimMode::WhatNext),
                ("net-zero by 2040", SimMode::WhatIf),
                ("AMOC -30%", SimMode::WhatIf),
            ],
        }
    }
}

impl ScenarioEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn library(&self) -> &[(&'static str, SimMode)] {
        &self.library
    }

    pub fn run(&self, id: &str) -> Result<ScenarioRun, SimError> {
        let mode = self
            .library
            .iter()
            .find(|(name, _)| *name == id)
            .map(|(_, mode)| *mode)
            .ok_or(SimError::UnknownScenario)?;
        Ok(ScenarioRun {
            id: id.into(),
            mode,
            outcomes: stub_outcomes(),
        })
    }
}

fn stub_outcomes() -> Vec<Distribution> {
    [
        OutcomeKind::Climate,
        OutcomeKind::Biodiversity,
        OutcomeKind::Economy,
        OutcomeKind::Welfare,
        OutcomeKind::TippingRisk,
    ]
    .into_iter()
    .map(|kind| Distribution {
        kind,
        mean: 0.0,
        uncertainty: 1.0,
        source: SourceKind::Synthetic,
    })
    .collect()
}
